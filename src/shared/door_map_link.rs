// > SUPER / CRATE
use crate::world::WorldMap;
use super::{MoveDirection, WorldCoordinates};

/// ## DoorMapLink
///
/// A struct containing the [WorldCoordinates](crate::shared::WorldCoordinates) of a door tile which
/// enables transitioning to another map `usize` id once the player moves onto the tile and then 
/// moves off of the tile in the defined MoveDirection
///
/// #### Version: 0.0.1
///
/// #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoorMapLink {
    pub door_coords: WorldCoordinates,
    pub transition_direction: MoveDirection,
    pub map_id: usize
}