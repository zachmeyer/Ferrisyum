// > USE STD
use std::collections::{HashMap, HashSet};

// > USE CRATE/SUPER
use super::{
    treasures::*,
    WorldCoordinates,
    traits::Identifiable
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct UniqueItem {
    id: TreasureID,
    qty: TreasureQuantity,
}

impl UniqueItem {
    pub fn from_treasure_type(ttype: TreasureType, quantity: TreasureQuantity) -> Self {
        Self {
            id: ttype.to_id(),
            qty: quantity,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub(crate) enum TreasureCollectionVariant {
    #[default]
    Uninst,
    TreasureChest,
    PlayerInventory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreasureCollection {
    pub variant: TreasureCollectionVariant,
    pub uitems: Option<HashSet<UniqueItem>>,
    pub items: HashMap<TreasureID, TreasureQuantity>,
    pub world_coords: Option<WorldCoordinates>,
}

impl TreasureCollection {
    pub fn builder() -> TreasureCollectionBuilder {
        TreasureCollectionBuilder::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct TreasureCollectionBuilder {
    variant: TreasureCollectionVariant,
    uitems: Option<HashSet<UniqueItem>>,
    items: HashMap<TreasureID, TreasureQuantity>,
    world_coords: Option<WorldCoordinates>,
}

impl TreasureCollectionBuilder {
    pub fn new(variant: TreasureCollectionVariant) -> TreasureCollectionBuilder {
        TreasureCollectionBuilder {
            variant,
            uitems: None,
            items: HashMap::with_capacity(1),
            world_coords: None,
        }
    }

    pub fn unique_items(mut self, uitems: &[UniqueItem]) -> TreasureCollectionBuilder {
        match self.variant {
            TreasureCollectionVariant::PlayerInventory
            | TreasureCollectionVariant::TreasureChest => {
                self.uitems = Some(HashSet::from_iter(uitems.iter().cloned()));
            }
            _ => panic!(
                "Unique items cannot be provided to uninstantiated-variant treasure collections."
            ),
        }

        self
    }

    pub fn items(mut self, items: &[(TreasureID, TreasureQuantity)]) -> TreasureCollectionBuilder {
        match self.variant {
            TreasureCollectionVariant::PlayerInventory
            | TreasureCollectionVariant::TreasureChest => {
                self.items = HashMap::from_iter(items.iter().cloned());
                self
            }
            _ => panic!(
                "Items cannot be provided to 
                uninstantiated-variant treasure collections."
            ),
        }
    }

    pub fn coords(mut self, coords: WorldCoordinates) -> TreasureCollectionBuilder {
        match self.variant {
            TreasureCollectionVariant::TreasureChest => {
                self.world_coords = Some(coords);
                self
            }
            _ => panic!(
                "World coordinates cannot be used in 
                uninstantiated-type or inventory-type treasure collections."
            ),
        }
    }

    pub fn build(self) -> TreasureCollection {
        TreasureCollection {
            variant: self.variant,
            uitems: self.uitems,
            items: self.items,
            world_coords: self.world_coords,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreasureType {
    Gold,
    Potion,
    Armor,
    Weapon,
}

impl Identifiable for TreasureType {
    fn to_id(&self) -> usize {
        match self {
            TreasureType::Gold => 1,
            TreasureType::Potion => 2,
            TreasureType::Armor => 3,
            TreasureType::Weapon => 4,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Treasure {
    pub t_type: TreasureType
}
