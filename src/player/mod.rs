// > USE STD
use std::collections::HashSet;

// > USE CRATE
use crate::shared::extlib::NAVector3;
use crate::shared::treasures::*;
use crate::shared::traits::{
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
    pub fn new(draw: char, position: NAVector3<i64>) -> Self {
        // Build the player's initial inventory
        let inventory = TreasureCollectionBuilder::new(TreasureCollectionVariant::PlayerInventory)
            .unique_items(&[
                UniqueItem::from_treasure_type(TreasureType::Gold, 0)
            ]).build();

        Self {
            new_position: position,
            previous_position: position,
            keyring: HashSet::new(),
            draw,
            position,
            inventory,
        }
    }
}

impl Positionable for Player {
    fn row(&self) -> Row {
        self.position.y as Row
    }

    fn col(&self) -> Column {
        self.position.x as Column
    }
}

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

impl ToIdentifiableChar for Player {
    fn to_char_id(&self) -> char {
        self.draw
    }
}
