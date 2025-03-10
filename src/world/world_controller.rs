//! # World Controller
//!
//! Manages the game world state, interactions, and generating renderables to pass to the 
//! [WorldView](crate::world::WorldView)
//!
//! ## Overview
//! The WorldController handles:
//! - Map generation (but *not* rendering - this is handled by the WorldView)
//! - Processing world update events
//! - Managing interactions between player and world elements (doors, keys, treasure, etc.)
//!
//! #### Version: 0.0.1
//!
//! #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)

// > USE
use std::collections::BinaryHeap;
use std::cell::RefCell;
use std::io::Read;
use grid::*;
use regex::Regex;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget, StatefulWidget};

// > CRATE
use crate::shared::constants::TILE_WIDTH;
use crate::shared::traits::{
    IdentifiableChar, Moveable, Positionable, ToIdentifiableChar
};
use crate::shared::{
    CommonState,
    KeyDoorLink, 
    MoveDirection,  
    Tile,
};
use crate::shared::extlib::{RatatuiRect};
use crate::Player;
use crate::world::{WorldMap, WorldUpdate, WorldUpdateEventType};

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Manages the game world and processes world update events
///
/// Controls the map state and handles queued world update events,
/// such as key pickups and door interactions.
pub struct WorldController<'wctrl> {
    pub map: WorldMap,
    pub update_queue: &'wctrl mut BinaryHeap<WorldUpdate<WorldUpdateEventType>>,
}

impl<'wctrl> WorldController<'wctrl> {
    /// Creates a new WorldController with the default map
    ///
    /// # Arguments
    /// * `update_queue` ([BinaryHeap]) - Handles [WorldUpdate] requests 
    pub fn new(update_queue: &'wctrl mut BinaryHeap<WorldUpdate<WorldUpdateEventType>>) -> Self {
        
        // TODO: THIS NEEDS TO BE ABLE TO READ OTHER FILES (MAPS) ///////////////////////////
        let mut buf: Vec<u8> = vec![];
        std::fs::File::open_buffered("assets/test_map1.txt")
                                .expect("Unable to open test map file.")
                                .read_to_end(&mut buf)
                                .expect("Unable to read contents of test map file.");
        Self {
            map: WorldMap::from_bytes(&buf),
            update_queue
        }
    }

    /// Handles updates pushed to the update event queue
    ///
    /// # Arguments
    /// * `&player` - ([`Player`]) An ***immutable*** reference to the player object.
    /// 
    pub fn generate_map(&'wctrl self, player: &'wctrl Player) -> impl Widget + 'wctrl {
        /// ## MapWidget
        /// 
        /// Widget for rendering the world map with the player character
        struct MapWidget<'wctrl> {
            controller: &'wctrl WorldController<'wctrl>,
            player: &'wctrl Player,
        }

        // IMPL MapWidget
        impl<'wctrl> Widget for MapWidget<'wctrl> {
            fn render(self, area: Rect, buf: &mut Buffer) {
                let map_width = self.controller.map.grid.cols();
                let map_height = self.controller.map.grid.rows();

                let world_width = map_width as u16 + 2;
                let world_height = map_height as u16 + 2;

                let map_area = centered_rect(world_width, world_height, area);

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White));
                
                let inner = block.inner(map_area);

                for row in 0..map_height {
                    for col in 0..(map_width / TILE_WIDTH as usize) {
                        if  row >= self.controller.map.grid.rows() 
                            || col >= self.controller.map.grid.cols() 
                        {
                            continue;
                        }

                        let x = inner.x + (col as u16 * TILE_WIDTH);
                        let y = inner.y + row as u16;

                        let tile = &self.controller.map.grid[(row, col)];
                        let c = tile.to_char_id();
                        let style = Style::default().bg(Color::Black);

                        // Make sure we're within bounds
                        if x < inner.x + inner.width && y < inner.y + inner.height {
                            if row == self.player.row() && col == self.player.col() {
                                buf[(x, y)]
                                    .set_symbol(&format!("{:2}", self.player.to_char_id()))
                                    .set_style(style);
                            }
                            // Draw the tile
                            else {
                                buf[(x, y)].set_symbol(&format!("{:2}", c)).set_style(style);
                            }
                        }
                    }
                }

                block.render(map_area, buf);
            }
        }

        // Helper function to create a centered rect
        fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
            let x = r.x + (r.width.saturating_sub(width)) / 2;
            let y = r.y + (r.height.saturating_sub(height)) / 2;
            
            Rect::new(
                x,
                y,
                width.min(r.width),
                height.min(r.height)
            )
        }

        // Return the MapWidget
        MapWidget {
            controller: self,
            player,
        }
    }

    /// Adds a world update to the queue for later processing
    ///
    /// # Arguments
    /// * `update<T: WorldUpdateEventType>` - ([`WorldUpdate`]) The world update event to queue
    /// 
    pub fn queue_update(&mut self, update: WorldUpdate<WorldUpdateEventType>) {
        self.update_queue.push(update);
    }

    /// Handles updates pushed to the update event queue
    ///
    /// # Arguments
    /// * `&mut player` - ([`Player`]) A ***mutable*** reference to the player
    /// 
    pub fn update_world(&mut self, player: &mut Player) {
        while let Some(update) = self.update_queue.pop() {
            match update.event_type {
                // IDLE EVENT
                // -> Nothing Happens
                WorldUpdateEventType::Idle => {},

                // CHANGE TILE STATE
                // -> TODO: Not yet implemented.
                // NOTE: used for changing tile states outside of interaction, such as pressing a 
                // button to open a door. Interaction should be handled within a separate event 
                // state if there's translation or state transfer involved (see TryOpenDoor)
                WorldUpdateEventType::ChangeTileState(_, _) => { todo!() },

                // KEY PICKUP
                WorldUpdateEventType::KeyPickup(kdl) => { 
                    let (kr, kc) = (kdl.key.0, kdl.key.1);

                    // We can get away with just getting a reference to the TileProperties
                    let tp_mut = &mut self.map.grid[(kr, kc)].get_properties_mut();

                    player.keyring.insert(kdl);
                    tp_mut.kdl = None;
                    tp_mut.draw_character = '.';
                 },

                 // DOOR INTERACTION
                 // -> Does not open without the appropriate KeyDoorLink on the player's keyring
                 WorldUpdateEventType::TryOpenDoor(door_coords) => {
                    // Find the matching KeyDoorLink that opens this door
                    if let Some(position) = player
                        .keyring
                        .iter()
                        .position(|kdl| kdl.door == door_coords) 
                    {
                        let (dr, dc) = (door_coords.0, door_coords.1);

                        // Note we have to get a reference to the whole-ass tile here, 
                        // not just its properties since we need to manipulate the tile's 
                        // state as well (unlike KeyPickup which uses tp_mut)
                        let t_mut = &mut self.map.grid[(dr, dc)];
                        
                        let kdl_to_remove = player.keyring.iter()
                            .find(|kdl| kdl.door == door_coords)
                            .cloned();
                        
                        if let Some(kdl) = kdl_to_remove {
                            player.keyring.remove(&kdl);
                        }
                        
                        t_mut.get_properties_mut().kdl = None;
                        t_mut.get_properties_mut().draw_character = '\\';
                        t_mut.change_state(CommonState::UNLOCKED);
                        player.translate_into();
                    }
                }
            }
        }
    }
}