//
//
// > CRATE / SUPER
use crate::shared::{WorldCoordinates, traits::{Identifiable, IdentifiableFrom}};
use super::{TreasureID, TreasureQuantity, TreasureType};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// ### TreasureCollection
/// 
/// Encapsulates a TreasureCollection (constructed using the
/// [TreasureCollectionBuilder](crate::shared::TreasureCollectionBuilder))
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreasureCollection {
    pub variant: TreasureCollectionVariant,
    pub items: Vec<(TreasureType, TreasureQuantity)>,
    pub world_coords: Option<WorldCoordinates>,
}

// IMPL
impl TreasureCollection {
    
    #[allow(dead_code)]
    pub fn builder() -> TreasureCollectionBuilder {
        TreasureCollectionBuilder::default()
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

/// ### TreasureCollectionBuilder
///
/// Creates a [TreasureCollection](crate::shared::TreasureCollection)
/// 
#[derive(Default)]
pub struct TreasureCollectionBuilder {
    variant: TreasureCollectionVariant,
    items: Vec<(TreasureType, TreasureQuantity)>,
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
            items: Vec::with_capacity(1),
            world_coords: None,
        }
    }

    /// Sets the unique items for the treasure collection.
    ///
    /// ## Arguments
    ///
    /// * `items` - A slice of unique items to be added to the collection.
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
                items.iter().for_each(|(tid, tqty)| {
                    self.items.push((
                        TreasureType::from_id(*tid),
                        *tqty
                    ))
                });
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
            items: self.items,
            world_coords: self.world_coords,
        }
    }
}
