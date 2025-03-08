//! # Shared
//!
//! Re-exports various structs and traits that are used throughout the game engine. Additionally
//! re-exports some 3rd-party crate structs and types that are also used regularly.
//!
//! #### Version: 0.0.1
//!
//! #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)

// >> 3P (RE-EXPORT)
pub(crate) mod extlib {
    pub(crate) use crossterm::event::{
        self as crossterm_event, Event as CrosstermEvent, KeyCode as CrosstermKeyCode,
    };
    pub(crate) use nalgebra::{Matrix3 as NAMatrix3, Vector3 as NAVector3};
    pub(crate) use ratatui::{
        DefaultTerminal as RatatuiDefaultTerminal,
        buffer::Buffer as RatatuiBuffer,
        layout::Rect as RatatuiRect,
        widgets::Widget as RatatuiWidget,
    };
}

// >> CRATE (RE-EXPORT)
pub(crate) mod constants;
pub(crate) mod traits;

// >> PRIMITIVE TYPE ALIASES

/// The row (y) coordinate within the grid as `usize`
pub(crate) type Row = usize;

/// The column (x) coordinate within the grid as `usize`
pub(crate) type Column = usize;

/// A container for the Row, Column ( y,x ) coordinates within the grid as `(usize, usize)`
pub(crate) type WorldCoordinates = (Row, Column);

///////////////////////////////////////////////////////////////////////////////////////////////////

mod common_state;
pub use common_state::CommonState;

mod key_door_link;
pub use key_door_link::KeyDoorLink;

mod treasure_collection;
pub(crate) use treasure_collection::*;

mod move_direction;
pub use move_direction::MoveDirection;

mod tile;
pub use tile::{Tile, TileProperties};

pub(crate) mod treasures {
    //! ## Treasures
    //! ### sub-module
    //! 
    //! Compartmentalizes the [TreasureCollection](crate::shared::TreasureCollection) struct, the
    //! **TreasureID** `usize` alias, and the **TreasureQuantity** `u64` alias

    pub(crate) use super::treasure_collection::*;

    /// The ID of the treasure as `usize`
    pub(crate) type TreasureID = usize;

    /// The Quantity of the treasure as `usize`
    pub(crate) type TreasureQuantity = u64;
}
