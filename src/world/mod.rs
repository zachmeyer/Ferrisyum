//! # *mod* World
//!
//! Re-exports various sub-modules from [crate::world]
//!
//! ## Methods
//! 
//! Contains inline methods that facilitate the interaction between the WorldController,
//! the WorldView and objects that must interact with either or both of these outside of their
//! respective lifetimes. All methods here must inherit ***lifetime <'gloop>*** (representing one
//! iteration of the game loop).
//!
//! #### Version: 0.0.1
//!
//! #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)

// >> CRATE (RE-EXPORT)
mod world_map;
pub(crate) use world_map::*;

mod world_update;
pub(crate) use world_update::*;

mod world_view;
pub(crate) use world_view::*;

mod world_controller;
pub(crate) use world_controller::*;

// > CRATE
use crate::shared::traits::{Moveable, Positionable};
use crate::shared::{CommonState, Tile, MoveDirection};

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Translates an `impl Moveable` within the [World](WorldController)
///
/// # Arguments
/// * `moveable_mut` ( `&mut impl Moveable` ) - A ***mutable reference*** to any object that 
///   implements `Moveable` ( and `Positionable` which is ***required*** by `Moveable` )
/// * `world`        ( `&mut WorldController` ) - A ***mutable reference*** to the active 
///   WorldController
/// * `direction` ( [MoveDirection] ) - The `enum MoveDirection` indicating the direction to move
/// 
/// #### Objects must survive for lifetime <'gloop> (one iteration of the game loop)
pub fn translate<'gloop>(
    moveable_mut: &'gloop mut impl Moveable,
    world: &'gloop mut WorldController,
    direction: MoveDirection,
) {
    type Wup = WorldUpdate<WorldUpdateEventType>;
    type Wut = WorldUpdateEventType;

    moveable_mut.translate(direction);
    let new_coords = (moveable_mut.new_row(), moveable_mut.new_col());

    match &world.map.grid[new_coords] {

        // MOVING ONTO FLOOR TILE
        // -> Can contain event, but currently no floor tiles do
        Tile::Floor(_) => moveable_mut.translate_into(),

        // MOVING ONTO KEY TILE
        // -> Send a key pickup event to the World Controller
        Tile::Key(props) => { 
            moveable_mut.translate_into();
            if let Some(kdl) = props.kdl {
                world.queue_update(
                    Wup::new(Wut::KeyPickup(kdl))
                );
            }
        },

        // MOVING ONTO DOOR TILE
        // -> Check to see if the player possesses the proper key and allow entry, changing the
        // tile state
        Tile::Door(props, state) => {
            match *state {
                CommonState::LOCKED => {
                    world.queue_update(Wup::new(Wut::TryOpenDoor(props.world_coordinates)))
                },
                CommonState::UNLOCKED => moveable_mut.translate_into(),
                _ => {}
            }
        }
        
        _ => () // WALL OR ANY UNIMPLEMENTED TILE = UNPASSABLE
    }
}

pub enum ExplicitPickupType {
    TreasureChest
}

/// Sends a signal to the WorldController to trigger a "pickup" event explicitly (versus
/// pickups that occur automatically during translation, e.g. `KeyPickup`)
///
/// # Arguments
/// * `moveable_mut` ( `&mut impl Positionable` ) - A ***mutable reference*** to any object that 
///   implements `Positionable`
/// * `world`        ( `&mut WorldController` ) - A ***mutable reference*** to the active 
///   WorldController
/// 
/// #### Objects must survive for lifetime <'gloop> (one iteration of the game loop)
pub fn pickup_explicit<'gloop>(
    positionable_mut: &'gloop mut impl Positionable, 
    world: &'gloop mut WorldController,
    explicit_pickup_type: ExplicitPickupType
) {
    type Wup = WorldUpdate<WorldUpdateEventType>;
    type Wut = WorldUpdateEventType;

    let coords = (positionable_mut.row(), positionable_mut.col());
    let adjacents: [[usize; 2]; 4] = [
        [coords.0.wrapping_sub(1), coords.1],
        [coords.0.wrapping_add(1), coords.1],
        [coords.0, coords.1.wrapping_sub(1)],
        [coords.0, coords.1.wrapping_add(1)]
    ];

    for &[r, c] in &adjacents {
        if  world.within_bounds(&(r as isize), &(c as isize)) 
            && world.map.grid[(r, c)].get_properties_mut().treasure.is_some() 
        {
            match explicit_pickup_type {
                ExplicitPickupType::TreasureChest => world.queue_update(
                    Wup::new(Wut::PickupTreasure((r, c)))
                )
            }
        }
    }
}
