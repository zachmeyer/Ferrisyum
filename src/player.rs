// > USE STD
use std::collections::HashSet;

// > USE CRATE
use crate::shared::extlib::NAVector3;
use crate::shared::treasures::*;
use crate::shared::traits::{
    Identifiable,
    Moveable, 
    Positionable, 
    ToIdentifiableChar
};
use crate::shared::{
    Column, Row,
    KeyDoorLink, 
    MoveDirection, 
    TileProperties
};


///////////////////////////////////////////////////////////////////////////////////////////////////

/// Encapsulates a player in the game world.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player {
    draw: char,
    position: NAVector3<i64>,
    new_position: NAVector3<i64>,
    previous_position: NAVector3<i64>,
    
    pub keyring: HashSet<KeyDoorLink>,
    pub inventory: TreasureCollection,
}

impl Player {
    /// Creates a new `Player` with the specified character and position.
    ///
    /// # Arguments
    ///
    /// * `draw` - The `char` used to represent the player visually.
    /// * `position` - The initial `NAVector3` of the player.
    pub fn new(draw: char, position: NAVector3<i64>) -> Self {
        // Build the player's initial inventory
        let inventory = TreasureCollectionBuilder::new(TreasureCollectionVariant::PlayerInventory)
            .items(&[(TreasureType::Gold.to_id(), 0)]).build();

        // When creating a new player, the current, new-, and previous- position are all the same
        Self {
            new_position: position,
            previous_position: position,
            keyring: HashSet::new(),
            draw,
            position,
            inventory,
        }
    }

    /// Fetches the player's gold quantity (`u64`) in a safe way, considering the player's gold
    /// should always sit at index 0 in their `inventory`
    pub fn gold_qty(&self) -> u64 {
        self.inventory.items[0].1
    }

    /// Increases the player's gold quantity (`u64`) in a safe way, considering the player's gold
    /// should always sit at index 0 in their `inventory`
    pub fn add_gold(&mut self, quantity: u64) {
        self.inventory.items[0].1 += quantity;
    }
}

// IMPL Positionable
impl Positionable for Player {
    fn row(&self) -> Row {
        self.position.y as Row
    }

    fn col(&self) -> Column {
        self.position.x as Column
    }
}

// IMPL Moveable
impl Moveable for Player {
    fn new_col(&self) -> Column {
        self.new_position.x as Column
    }

    fn new_row(&self) -> Row {
        self.new_position.y as Row
    }

    fn translate(&mut self, direction: MoveDirection) -> &Self {
        self.previous_position = self.position;
        self.new_position = direction.translate() * self.position;
        self
    }

    fn translate_into(&mut self) {
        self.position = self.new_position;
    }
}

// IMPL ToIdentifiableChar
impl ToIdentifiableChar for Player {
    fn to_char_id(&self) -> char {
        self.draw
    }
}