// > USE CRATE
use crate::shared::treasure::*;
use crate::shared::traits::{IdentifiableChar, Positionable};
use crate::shared::{
    Column, 
    CommonState, 
    KeyDoorLink, 
    Row, 
    WorldCoordinates
};

use super::traits::ToIdentifiableChar;

///////////////////////////////////////////////////////////////////////////////////////////////////

/// ## Tile
///
/// A struct compartmentalizing the various types of `Tile` which can pass a
/// [TileProperties](crate::shared::TileProperties) and possibly a 
/// [CommonState](crate::shared::CommonState)
///
/// #### Version: 0.0.1
///
/// #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Door(TileProperties, CommonState),
    Floor(TileProperties),
    Key(TileProperties),
    Treasure(TileProperties, CommonState),
    Wall(TileProperties),
}

// IMPL Default
impl Default for Tile {

    /// Returns a default tile (Floor tile)
    fn default() -> Tile {
        Tile::Floor(TileProperties::default())
    }
}

// IMPL
impl Tile {

    /// ## Returns 
    /// 
    /// A ***mutable*** reference to the `TileProperties`
    pub fn get_properties_mut(&mut self) -> &mut TileProperties {
        match self {
            Tile::Door(props, _) => props,
            Tile::Floor(props) => props,
            Tile::Key(props) => props,
            Tile::Wall(props) => props,
            Tile::Treasure(props, _) => props,
        }
    }

    /// ## Returns 
    /// 
    /// An ***immutable*** reference to the `TileProperties`
    pub fn get_properties(&self) -> &TileProperties {
        match self {
            Tile::Door(props, _) => props,
            Tile::Floor(props) => props,
            Tile::Key(props) => props,
            Tile::Wall(props) => props,
            Tile::Treasure(props, _) => props,
        }
    }

    /// Changes the `CommonState` of the `Tile`
    /// 
    /// ## Arguments
    /// * `new_state` ( `CommonState` ) - A [CommonState](crate::shared::CommonState)
    pub fn change_state(&mut self, new_state: CommonState) {
        match self {
            Tile::Door(_, state) => *state = new_state,
            Tile::Treasure(_, state) => *state = new_state,
            _ => {}
        }
    }

    /// ## Returns
    /// 
    /// * `Some(&CommonState)` - If the `Tile` is a `Door` or `Treasure`, returns a reference 
    ///   to its `CommonState`.
    /// * `None` - If the `Tile` is a `Floor`, `Key`, or `Wall`, returns `None`.
    pub fn get_state(&self) -> Option<&CommonState> {
        match self {
            Tile::Door(_, state) => Some(state),
            Tile::Treasure(_, state) => Some(state),
            // Return None for variants without state
            Tile::Floor(_) | Tile::Key(_) | Tile::Wall(_) => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl Positionable for Tile {

    /// Returns the immutable `row` ( y ) coordinate of the tile
    fn row(&self) -> Row {
        self.get_properties().row
    }

    /// Returns the immutable `col` ( x ) coordinate of the tile
    fn col(&self) -> Column {
        self.get_properties().col
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl IdentifiableChar for Tile {

    /// Creates a `Tile` from a given character identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - A character representing the type of the tile.
    ///
    /// # Returns
    ///
    /// A `Tile` instance corresponding to the provided character identifier.
    ///
    /// # Panics
    ///
    /// Panics if an invalid character identifier is supplied.
    fn from_char_id(id: char) -> Self {
        match id {
            '|' => Self::Door(
                TileProperties {
                    draw_character: '|',
                    ..Default::default()
                },
                CommonState::LOCKED,
            ),
            '\\' => Self::Door(
                TileProperties {
                    draw_character: '\\',
                    ..Default::default()
                },
                CommonState::UNLOCKED,
            ),
            '.' => Self::Floor(TileProperties {
                draw_character: '.',
                ..Default::default()
            }),
            'K' => Self::Key(TileProperties {
                draw_character: {
                    if cfg!(debug_assertions) { 'K' } else { '.' } // Reveal keys in debug mode
                },
                ..Default::default()
            }),
            'D' => Self::Treasure(
                TileProperties {
                    draw_character: 'D',
                    ..Default::default()
                },
                CommonState::UNCOLLECTED,
            ),
            '0' => Self::Treasure(
                TileProperties {
                    draw_character: '0',
                    ..Default::default()
                },
                CommonState::COLLECTED,
            ),
            '#' => Self::Wall(TileProperties {
                draw_character: '#',
                ..Default::default()
            }),
            _ => panic!("Invalid identity char supplied for Tile"),
        }
    }
}

impl ToIdentifiableChar for Tile {

    /// Returns the character identifier for the `Tile`.
    ///
    /// # Returns
    ///
    /// A character representing the type of the tile.
    fn to_char_id(&self) -> char {
        self.get_properties().draw_character
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileProperties {
    pub row: Row,
    pub col: Column,
    pub world_coordinates: WorldCoordinates,
    pub draw_character: char,
    pub treasure: Option<TreasureCollection>,
    pub kdl: Option<KeyDoorLink>
}

// IMPL TileProperties
//
impl TileProperties {

    /// Sets the position of the tile.
    ///
    /// ## Arguments
    ///
    /// * `coords` - The new world coordinates for the tile.
    /// 
    pub fn set_position(&mut self, coords: WorldCoordinates) {
        (self.row, self.col) = (coords.0, coords.1);
        self.world_coordinates = coords;
    }

    /// Links a door to a key-door link.
    ///
    /// ## Arguments
    ///
    /// * `kdl` - The key-door link to be associated with the door
    /// 
    pub fn link_door(&mut self, kdl: KeyDoorLink) {
        self.kdl = Some(kdl);
    }
}

// IMPL Default
//
impl Default for TileProperties {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
            world_coordinates: (0, 0),
            draw_character: '?',
            treasure: None,
            kdl: None
        }
    }
}
