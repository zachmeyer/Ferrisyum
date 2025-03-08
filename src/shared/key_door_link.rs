// > SUPER
use super::WorldCoordinates;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyDoorLink {
    pub key: WorldCoordinates,
    pub door: WorldCoordinates,
}
