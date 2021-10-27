use std::{collections::HashMap, convert::TryInto, str::FromStr};

use crate::tile::Tile;

pub struct TilesIndex<const N: usize>(pub HashMap<u16, Tile<N>>);

impl<const N: usize> FromStr for TilesIndex<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: HashMap<u16, Tile<N>> = HashMap::new();

        for raw_tile in s.split("\n\n").take_while(|t| !t.is_empty()) {
            let mut lines = raw_tile.lines();
            let raw_id = lines.next().unwrap();
            let id: u16 = raw_id[5..9].parse().unwrap();
            let cells: [[char; N]; N] = lines
                .map(|l| l.chars().collect::<Vec<char>>().try_into().unwrap())
                .collect::<Vec<[char; N]>>()
                .try_into()
                .unwrap();

            let tile = Tile::new(id, cells);
            tiles.insert(id, tile);
        }

        Ok(Self(tiles))
    }
}
