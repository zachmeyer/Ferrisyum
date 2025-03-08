// > USE CRATE / LOCAL
use crate::shared::treasures::*;
use crate::shared::traits::{IdentifiableChar, Positionable};
use crate::shared::{
    Column, 
    CommonState, 
    KeyDoorLink, 
    Row, 
    WorldCoordinates
};

///////////////////////////////////////////////////////////////////////////////////////////////////

//
// ** Tile ENUM
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Door(TileProperties, CommonState),
    Floor(TileProperties),
    Key(TileProperties),
    Treasure(TileProperties, CommonState),
    Wall(TileProperties),
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Floor(TileProperties::default())
    }
}

impl Tile {
    pub fn get_properties_mut(&mut self) -> &mut TileProperties {
        match self {
            Tile::Door(props, _) => props,
            Tile::Floor(props) => props,
            Tile::Key(props) => props,
            Tile::Wall(props) => props,
            Tile::Treasure(props, _) => props,
        }
    }

    pub fn get_properties(&self) -> &TileProperties {
        match self {
            Tile::Door(props, _) => props,
            Tile::Floor(props) => props,
            Tile::Key(props) => props,
            Tile::Wall(props) => props,
            Tile::Treasure(props, _) => props,
        }
    }

    pub fn change_state(&mut self, new_state: CommonState) {
        match self {
            Tile::Door(_, state) => *state = new_state,
            Tile::Treasure(_, state) => *state = new_state,
            _ => {}
        }
    }

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
    fn row(&self) -> Row {
        self.get_properties().row
    }

    fn col(&self) -> Column {
        self.get_properties().col
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl IdentifiableChar for Tile {
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
    pub fn set_position(&mut self, coords: WorldCoordinates) {
        (self.row, self.col) = (coords.0, coords.1);
        self.world_coordinates = coords;
    }

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
