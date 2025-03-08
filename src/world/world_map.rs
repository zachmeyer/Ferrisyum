// > USE 3P
use grid::*;
use regex::Regex;

// > USE CRATE/LOCAL
use crate::shared::{
    KeyDoorLink,
    Tile,
    traits::{IdentifiableChar, Positionable}
};

///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WorldMap {
   pub grid: Grid<Tile>,
}

impl WorldMap {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let contents = String::from_utf8_lossy(bytes);
        let lines = extract_map_lines(&contents);
        let key_door_links = parse_key_door_links(&contents);

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

                if col < grid.cols() && row < grid.rows() {
                    grid[(row, col)] = tile;
                }
            }
        }

        Self { grid }
    }
}

fn extract_map_lines(contents: &str) -> Vec<(usize, &str)> {
    contents
        .lines()
        .enumerate()
        .take_while(|(_, l)| !l.starts_with('~'))
        .collect()
}

// Parses key-door links using regex

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