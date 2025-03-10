/// The id of the treasure as `usize`
pub type TreasureID = usize;

/// The quantity of the treasure as `u64`
pub type TreasureQuantity = u64;

mod treasure_collection;
pub use treasure_collection::*;

mod treasure_type;
pub use treasure_type::TreasureType;