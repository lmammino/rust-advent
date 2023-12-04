use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    id: u32,
    x: usize,
    y: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Symbol {
    char: char,
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
struct Schematic {
    parts: Vec<Part>,
    parts_by_pos: HashMap<(usize, usize), usize>,
    symbols: HashSet<Symbol>,
    gears: HashSet<Symbol>,
}

impl Schematic {
    fn get_part_by_pos(&self, x: usize, y: usize) -> Option<&Part> {
        self.parts_by_pos.get(&(x, y)).map(|&idx| &self.parts[idx])
    }

    fn get_neighbour_parts(&self, x: usize, y: usize) -> HashSet<&Part> {
        let mut neighbours = HashSet::new();
        for (dx, dy) in NEIGHBOURS.iter() {
            let neighbour_x = x as isize + dx;
            let neighbour_y = y as isize + dy;
            if neighbour_x >= 0 && neighbour_y >= 0 {
                if let Some(part) = self.get_part_by_pos(neighbour_x as usize, neighbour_y as usize)
                {
                    neighbours.insert(part);
                }
            }
        }
        neighbours
    }
}

#[derive(Debug, Default)]
struct SchematicParser {
    schematic: Schematic,
    num_buffer: String,
    num_buffer_start_x: usize,
}

impl SchematicParser {
    fn add_new_part(&mut self, y: usize) {
        let x = self.num_buffer_start_x;
        let id = self.num_buffer.parse::<u32>().unwrap();
        let len = self.num_buffer.len();
        let new_part = Part { id, x, y, len };
        self.schematic.parts.push(new_part);
        let new_part_idx = self.schematic.parts.len() - 1;
        for curr_x in x..x + len {
            self.schematic
                .parts_by_pos
                .insert((curr_x, y), new_part_idx);
        }
        self.num_buffer = "".to_string();
        self.num_buffer_start_x = x + 1;
    }

    fn add_new_symbol(&mut self, char: char, x: usize, y: usize) {
        let new_symbol = Symbol { char, x, y };
        if char == '*' {
            self.schematic.gears.insert(new_symbol.clone());
        }
        self.schematic.symbols.insert(new_symbol);
    }

    pub fn parse(mut self, input: &str) -> Schematic {
        for (y, line) in input.lines().enumerate() {
            // line resets
            if !self.num_buffer.is_empty() {
                self.add_new_part(y - 1);
            }

            for (x, char) in line.char_indices() {
                match char {
                    '0'..='9' => {
                        if self.num_buffer.is_empty() {
                            self.num_buffer_start_x = x;
                        }
                        self.num_buffer.push(char);
                    }
                    '.' => {
                        if !self.num_buffer.is_empty() {
                            self.add_new_part(y);
                        }
                    }
                    symbol => {
                        if !self.num_buffer.is_empty() {
                            self.add_new_part(y);
                        }
                        self.add_new_symbol(symbol, x, y);
                    }
                }
            }
        }

        self.schematic
    }
}

pub fn part1(input: &str) -> u32 {
    let schematic = SchematicParser::default().parse(input);

    schematic
        .symbols
        .iter()
        .flat_map(|symbol| schematic.get_neighbour_parts(symbol.x, symbol.y))
        .map(|part| part.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let schematic = SchematicParser::default().parse(input);
    schematic
        .gears
        .iter()
        .filter_map(|gear| {
            let parts = schematic.get_neighbour_parts(gear.x, gear.y);
            if parts.len() == 2 {
                return Some(parts.iter().map(|part| part.id).product::<u32>());
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 557705);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 84266818);
    }
}
