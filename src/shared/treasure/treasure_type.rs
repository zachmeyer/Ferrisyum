use crate::shared::traits::{Identifiable, IdentifiableFrom};

/// An `enum` indicating the various types of `TreasureType`s available
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreasureType {
    Gold,
    Potion,
    Armor,
    Weapon,
}

// IMPL
impl Identifiable for TreasureType {

    /// Converts the `TreasureType` to its corresponding unique identifier of type `usize`
    fn to_id(&self) -> usize {
        match self {
            TreasureType::Gold => 1,
            TreasureType::Potion => 2,
            TreasureType::Armor => 3,
            TreasureType::Weapon => 4,
        }
    }
}

impl IdentifiableFrom for TreasureType {
    fn from_id(id: usize) -> Self {
        match id {
            1 => TreasureType::Gold,
            2 => TreasureType::Potion,
            3 => TreasureType::Armor,
            4 => TreasureType::Weapon,
            _ => panic!("Treasure Type usize id not implemented as valid TreasureType.")
        }
    }
}