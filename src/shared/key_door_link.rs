// > SUPER
use super::WorldCoordinates;

/// ## KeyDoorLink
///
/// A struct containing the [WorldCoordinates](crate::shared::WorldCoordinates) of the tile which
/// acts as a key, as well as the associated door tile that the key "opens" (by 
/// changing [CommonState](crate::shared::CommonState) to `UNLOCKED`)
///
/// #### Version: 0.0.1
///
/// #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyDoorLink {
    pub key: WorldCoordinates,
    pub door: WorldCoordinates,
}
