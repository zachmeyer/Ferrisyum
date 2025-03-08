use crate::shared::{CommonState, KeyDoorLink, WorldCoordinates};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum WorldUpdateEventType {
    Idle,
    ChangeTileState(WorldCoordinates, CommonState),
    KeyPickup(KeyDoorLink),
    TryOpenDoor(WorldCoordinates)
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorldUpdate<T: Eq + PartialEq> {
    pub event_type: T,
}

// Generic implementation
impl<T> WorldUpdate<T> 
where
    T: Eq + PartialEq + Clone
{
    pub fn new(event_type: T) -> Self {
        Self {
            event_type
        }
    }
    
    pub fn with_no_payload(event_type: T) -> Self {
        Self {
            event_type
        }
    }
}

// Specific implementation for WorldUpdateEventType
impl Ord for WorldUpdate<WorldUpdateEventType> 
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.event_type, &other.event_type) {
            (WorldUpdateEventType::ChangeTileState(_, _), _) => Ordering::Greater,
            (_, WorldUpdateEventType::Idle) => Ordering::Less,
            (_, _) => Ordering::Equal,
        }
    }
}

impl PartialOrd for WorldUpdate<WorldUpdateEventType> 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}