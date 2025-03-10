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
use std::collections::{BinaryHeap};
use std::io::Read;
use std::path::Path;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};

// > CRATE
use crate::shared::constants::TILE_WIDTH;
use crate::shared::traits::{
    Moveable, Positionable, ToIdentifiableChar
};
use crate::shared::CommonState;
use crate::shared::treasure::*;
use crate::Player;
use crate::world::{WorldMap, WorldUpdate, WorldUpdateEventType};

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Manages the game world and processes world update events
///
/// Controls the map state and handles queued world update events,
/// such as key pickups and door interactions.
pub struct WorldController<'wctrl> {
    pub maps: Vec<WorldMap>,
    pub update_queue: &'wctrl mut BinaryHeap<WorldUpdate<WorldUpdateEventType>>,
}

impl<'wctrl> WorldController<'wctrl> {
    /// Creates a new WorldController with the default map
    ///
    /// # Arguments
    /// * `update_queue` ([BinaryHeap]) - Handles [WorldUpdate] requests 
    pub fn new(update_queue: &'wctrl mut BinaryHeap<WorldUpdate<WorldUpdateEventType>>) -> Self {
        Self {
            maps: vec![],
            update_queue
        }
    }

    pub fn load_map_from_fstr(&mut self, map_file: &str) {
        let mut buf: Vec<u8> = vec![];
        std::fs::File::open_buffered(Path::new(map_file))
                                .unwrap_or_else(|_| panic!(
                                    "Unable to open map file: {}.", map_file
                                ))
                                .read_to_end(&mut buf)
                                .expect("Unable to read contents of test file.");
        let assigned_id = self.next_map_id();

        self.maps.push(WorldMap::from_bytes(&buf, assigned_id));
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
                let map_width = self.controller.maps[0].grid.cols();
                let map_height = self.controller.maps[0].grid.rows();

                let world_width = map_width as u16 + 2;
                let world_height = map_height as u16 + 2;

                let map_area = centered_rect(world_width, world_height, area);

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White));
                
                let inner = block.inner(map_area);

                for row in 0..map_height {
                    for col in 0..(map_width / TILE_WIDTH as usize) {
                        if  row >= self.controller.maps[0].grid.rows() 
                            || col >= self.controller.maps[0].grid.cols() 
                        {
                            continue;
                        }

                        let x = inner.x + (col as u16 * TILE_WIDTH);
                        let y = inner.y + row as u16;

                        let tile = &self.controller.maps[0].grid[(row, col)];
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
                    let (kr, kc) = (kdl.key_coords.0, kdl.key_coords.1);

                    // We can get away with just getting a reference to the TileProperties
                    let tp_mut = &mut self.maps[0].grid[(kr, kc)].get_properties_mut();

                    player.keyring.push(kdl);
                    tp_mut.kdl = None;
                    tp_mut.draw_character = '.';
                 },

                 // DOOR INTERACTION
                 // -> Does not open without the appropriate KeyDoorLink on the player's keyring
                 WorldUpdateEventType::TryOpenDoor(door_coords) => {
                    // Find the matching KeyDoorLink that opens this door
                    if let Some(kdl) = player.keyring.pop_if(|kdl| kdl.door_coords == door_coords) {
                        let dr = door_coords.0;
                        let dc = door_coords.1;
                        let t_mut = &mut self.maps[0].grid[(dr, dc)];
                        t_mut.get_properties_mut().kdl = None;
                        t_mut.get_properties_mut().draw_character = '\\';
                        t_mut.change_state(CommonState::UNLOCKED);
                        player.translate_into();
                    }
                }

                // TREAURE CHEST INTERACTION
                WorldUpdateEventType::PickupTreasure(tcoords) => {
                    let tr = tcoords.0;
                    let tc = tcoords.1;

                    // Refer to the whole tile to allow state change
                    let t_mut = &mut self.maps[0].grid[(tr, tc)];

                    if t_mut.get_state().unwrap() == &CommonState::UNCOLLECTED {
                        let tcoll = t_mut.get_properties_mut().treasure.as_ref().unwrap();

                        tcoll.items.iter().for_each(|(t, q)| {
                            match *t {
                                TreasureType::Gold => { player.add_gold(*q) }
                                // TODO: Implement Armor, Weapons, and Potion Treasures
                                TreasureType::Armor => { todo!() },
                                TreasureType::Potion => { todo!() },
                                TreasureType::Weapon => { todo!() }
                            }
                        });
                    }
                    
                    t_mut.get_properties_mut().treasure = None;
                    t_mut.get_properties_mut().draw_character = '0';
                    t_mut.change_state(CommonState::COLLECTED);
                }
            }
        }
    }

    /// Checks if a given row/col pair is within the map bounds. Row/col pair is passed as isize
    /// to check for negative bounds (this shouldn't realistically happen, but just in case)
    ///
    /// # Arguments
    /// * `&mut player` - ([`Player`]) A ***mutable*** reference to the player
    /// 
    pub fn within_bounds(&self, row: &isize, col: &isize) -> bool {
        (
            row >= &0 && row <= &(self.maps[0].grid.rows() as isize)) 
            && (col >= &0 && col <= &(self.maps[0].grid.cols() as isize)
        )
    }

    pub fn next_map_id(&self) -> usize {
        if self.maps.is_empty() { self.maps.len() } else { self.maps.len() + 1 }
    }
}