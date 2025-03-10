//! # World Map
//!
//! A container for the actual map grid, to abstract it away from the 
//! [WorldController](crate::world::WorldController)
//!
//! #### Version: 0.0.1
//!
//! #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)

// > USE
use std::collections::HashSet;
use grid::*;
use regex::Regex;

// > CRATE
use crate::shared::{
    KeyDoorLink,
    Tile,
    traits::{IdentifiableChar, Positionable}
};
use crate::shared::treasures::*;

///////////////////////////////////////////////////////////////////////////////////////////////////

/// "Containerizes" the Grid<Tile> of the WorldMap
pub struct WorldMap {
   pub grid: Grid<Tile>,
}

impl WorldMap {
    /// Loads and parses the [WorldMap] from the raw `&[u8]` bytes of a txt file. 
    ///
    /// # Arguments
    /// * `bytes` (`&[u8]`) - The bytes of the txt WorldMap file. 
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let contents = String::from_utf8_lossy(bytes);
        let lines = extract_map_lines(&contents);
        let key_door_links = parse_key_door_links(&contents);
        let treasure_chests = parse_treasure_chests(&contents);

        let mut grid = Grid::new(
            lines.len(),
            lines.first().map_or(0, |line| line.1.len() * 2),
        );

        for (row, line) in lines {
            for (col, c) in line.char_indices() {
                let mut tile = Tile::from_char_id(c);

                tile.get_properties_mut().set_position((row, col));

                if let Tile::Key(_) = tile {
                    let tr = tile.row();
                    let tc = tile.col();
                    tile.get_properties_mut().link_door(
                        if let Some(kdl) = key_door_links.iter().find(|kdl| kdl.key == (tr, tc)) {
                            *kdl 
                        } else {
                            panic!("Invalid key tile location defined for linking door.")
                        },
                    )
                }
                else if let Tile::Treasure(_, _) = tile {
                    let tr = tile.row();
                    let tc = tile.col();
                    if let Some(treasure) = treasure_chests.iter()
                        .find(|t| t.world_coords.unwrap() == (tr, tc)) 
                    {
                        tile.get_properties_mut().treasure = Some(treasure.clone());
                    } else {
                        panic!("Invalid treasure chest tile coordinates defined for treasure tile.")
                    }
                }

                if col < grid.cols() && row < grid.rows() {
                    grid[(row, col)] = tile;
                }
            }
        }

        Self { grid }
    }
}

/// Extracts map lines from the `&str` contents of the converted file bytes
///
/// # Arguments
/// * `contents` (`&str`) - The utf8 (lossy) string created from the bytes of a world map txt file. 
fn extract_map_lines(contents: &str) -> Vec<(usize, &str)> {
    contents
        .lines()
        .enumerate()
        .take_while(|(_, l)| !l.starts_with('~'))
        .collect()
}

/// Parses [KeyDoorLink](crate::shared::KeyDoorLink) 's from the contents of the converted file
/// bytes.
///
/// # Arguments
/// * `contents` (`&str`) - The utf8 (lossy) string created from the bytes of a world map txt file.
fn parse_key_door_links(contents: &str) -> Vec<KeyDoorLink> {
    let re = Regex::new(r"^~K\((\d+),(\d+)\) = D\((\d+),(\d+)\)$").unwrap();

    contents
        .lines()
        .skip_while(|ln| !ln.starts_with('~'))
        .filter_map(|line| {
            re.captures(line).map(|cap| KeyDoorLink {
                key: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                door: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            })
        })
        .collect()
}

/// Parses [TreasureCollection](crate::shared::TreasureCollection) 's from the contents of the 
/// converted file bytes.
///
/// # Arguments
/// * `contents` (`&str`) - The utf8 (lossy) string created from the bytes of a world map txt file.
fn parse_treasure_chests(contents: &str) -> Vec<TreasureCollection> {
    let treasure_re =
        Regex::new(r"^~T\((\d+),(\d+)\) = \(((?:\d+ = \d+)(?:, \d+ = \d+)*)\)$").unwrap();
    let mut treasures: Vec<TreasureCollection> = vec![];

    for line in contents.lines() {
        if let Some(caps) = treasure_re.captures(line) {
            let y = caps[1].parse().unwrap();
            let x = caps[2].parse().unwrap();
 
            let items_str = caps[3].split(", ");

            let mut tmp_collection: Vec<(TreasureID, TreasureQuantity)> = vec![];
            for item_def in items_str {
                let parts: Vec<&str> = item_def.split(" = ").collect();
                if parts.len() == 2 {
                    let id: TreasureID = parts[0].parse().unwrap();
                    let qty: TreasureQuantity = parts[1].parse().unwrap();
 
                    tmp_collection.push((id, qty));
                }
            }

            let tcoll = TreasureCollectionBuilder::new(TreasureCollectionVariant::TreasureChest)
                .coords((y, x))
                .items(&tmp_collection)
                .build();

            treasures.push(tcoll);
        }
    }

    treasures
}