#![allow(dead_code, unused_imports)]

// > USE
use std::cmp::Ordering;

// > CRATE
use crate::shared::{CommonState, KeyDoorLink, WorldCoordinates};
use crate::shared::treasure::*;

///////////////////////////////////////////////////////////////////////////////////////////////////

/// An `enum` encapsulating different types of events that can occur in the world.
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum WorldUpdateEventType {
    Idle,
    ChangeTileState(WorldCoordinates, CommonState),
    KeyPickup(KeyDoorLink),
    TryOpenDoor(WorldCoordinates),
    PickupTreasure(WorldCoordinates)
}

/// A generic struct to encapsulate world update events
#[derive(Debug, Eq, PartialEq)]
pub struct WorldUpdate<T: Eq + PartialEq> {
    pub event_type: T,
}

// IMPL (Generic)
impl<T> WorldUpdate<T> 
where
    T: Eq + PartialEq + Clone
{
    /// Creates a new `WorldUpdate` with the specified event type.
    pub fn new(event_type: T) -> Self {
        Self {
            event_type
        }
    }
    
    /// Creates a new `WorldUpdate` with the specified event type and no payload.
    pub fn with_no_payload(event_type: T) -> Self {
        Self {
            event_type
        }
    }
}

// Specific implementation for WorldUpdateEventType
impl Ord for WorldUpdate<WorldUpdateEventType> 
{
    /// Compares two `WorldUpdate` instances based on their event types.
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
     /// Partially compares two `WorldUpdate` instances based on their event types.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}