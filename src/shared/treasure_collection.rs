use std::collections::{HashMap, HashSet};

// > SUPER
use super::{
    treasures::*,
    WorldCoordinates,
    traits::Identifiable
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// ## UniqueItem
///
/// A containing the `TreasureID` and `TreasureQuantity` of a Unique (hashable) Item
///
/// #### Version: 0.0.1
///
/// #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct UniqueItem {
    id: TreasureID,
    qty: TreasureQuantity,
}

// IMPL
impl UniqueItem {

    /// Creates a new `UniqueItem` from a given `TreasureType` and `TreasureQuantity`.
    ///
    /// ## Arguments
    ///
    /// * `ttype` - The type of the treasure.
    /// * `quantity` - The quantity of the treasure.
    pub fn from_treasure_type(ttype: TreasureType, quantity: TreasureQuantity) -> Self {
        Self {
            id: ttype.to_id(),
            qty: quantity,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// An `enum` indicating the various types of `TreasureCollection`'s available
///
/// * `Uninst` - An uninstantiated TreasureCollection (used for starting the `builder`) process
/// * `TreasureChest` - A TreasureCollection used on a `Tile` type
/// * `PlayerInventory` - A TreasureCollection used within the `Player` type 
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub(crate) enum TreasureCollectionVariant {
    #[default]
    Uninst,
    TreasureChest,
    PlayerInventory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// ### TreasureCollection
/// 
/// Encapsulates a TreasureCollection (constructed using the
/// [TreasureCollectionBuilder](crate::shared::TreasureCollectionBuilder))
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreasureCollection {
    pub variant: TreasureCollectionVariant,
    pub uitems: Option<HashSet<UniqueItem>>,
    pub items: HashMap<TreasureID, TreasureQuantity>,
    pub world_coords: Option<WorldCoordinates>,
}

// IMPL
impl TreasureCollection {
    pub fn builder() -> TreasureCollectionBuilder {
        TreasureCollectionBuilder::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// ### TreasureCollectionBuilder
///
/// Creates a [TreasureCollection](crate::shared::TreasureCollection)
/// 
#[derive(Default)]
pub struct TreasureCollectionBuilder {
    variant: TreasureCollectionVariant,
    uitems: Option<HashSet<UniqueItem>>,
    items: HashMap<TreasureID, TreasureQuantity>,
    world_coords: Option<WorldCoordinates>,
}

// IMPL
impl TreasureCollectionBuilder {
    /// Creates a new `TreasureCollectionBuilder` with the specified variant.
    ///
    /// # Arguments
    ///
    /// * `variant` - The variant of the treasure collection.
    ///
    /// # Returns
    ///
    /// A new `TreasureCollectionBuilder` instance.
    pub fn new(variant: TreasureCollectionVariant) -> TreasureCollectionBuilder {
        TreasureCollectionBuilder {
            variant,
            uitems: None,
            items: HashMap::with_capacity(1),
            world_coords: None,
        }
    }

    /// Sets the unique items for the treasure collection.
    ///
    /// ## Arguments
    ///
    /// * `uitems` - A slice of unique items to be added to the collection.
    ///
    /// ## Returns
    ///
    /// The updated `TreasureCollectionBuilder` instance.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Uninst`.
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

    /// Sets the unique items for the treasure collection.
    ///
    /// ## Arguments
    ///
    /// * `uitems` - A slice of unique items to be added to the collection.
    ///
    /// ## Returns
    ///
    /// The updated `TreasureCollectionBuilder` instance.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Uninst`.
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

    /// Sets the items for the treasure collection.
    ///
    /// ## Arguments
    ///
    /// * `items` - A slice of tuples containing the treasure ID and quantity.
    ///
    /// ## Returns
    ///
    /// The updated `TreasureCollectionBuilder` instance.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Uninst`.
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

    /// Sets the world coordinates for the treasure collection.
    ///
    /// ## Arguments
    ///
    /// * `coords` - The world coordinates to be set.
    ///
    /// ## Returns
    ///
    /// The updated `TreasureCollectionBuilder` instance.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is not `TreasureChest`.
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

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A struct encapsulating a treasure with a specified `t_`type
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Treasure {
    pub t_type: TreasureType
}
