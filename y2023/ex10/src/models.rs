use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn is_opposite(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Self::North, Self::South)
                | (Self::South, Self::North)
                | (Self::East, Self::West)
                | (Self::West, Self::East)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Start,
    Vertical,
    Horizontal,
    BendL,
    BendJ,
    Bend7,
    BendF,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Start => 'S',
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::BendL => 'L',
            Self::BendJ => 'J',
            Self::Bend7 => '7',
            Self::BendF => 'F',
        };
        write!(f, "{}", c)
    }
}

impl Pipe {
    fn can_connect(&self, dir: Direction, pipe: &Pipe) -> bool {
        match (self, dir) {
            //    |  7  F
            // ↑  S  S  S
            (Self::Start, Direction::North) => {
                pipe == &Self::Vertical || pipe == &Self::Bend7 || pipe == &Self::BendF
            }

            //  →
            //     S-  SJ  S7
            (Self::Start, Direction::East) => {
                pipe == &Self::Horizontal || pipe == &Self::BendJ || pipe == &Self::Bend7
            }

            // ↓  S  S  S
            //    |  L  J
            (Self::Start, Direction::South) => {
                pipe == &Self::Vertical || pipe == &Self::BendL || pipe == &Self::BendJ
            }

            //  ←
            //     -S  LS  FS
            (Self::Start, Direction::West) => {
                pipe == &Self::Horizontal || pipe == &Self::BendL || pipe == &Self::BendF
            }

            //    |  7  F  S
            // ↑  |  |  |  |
            (Self::Vertical, Direction::North) => {
                pipe == &Self::Vertical
                    || pipe == &Self::Bend7
                    || pipe == &Self::BendF
                    || pipe == &Self::Start
            }

            // ↓  |  |  |  |
            //    |  L  J  S
            (Self::Vertical, Direction::South) => {
                pipe == &Self::Vertical
                    || pipe == &Self::BendL
                    || pipe == &Self::BendJ
                    || pipe == &Self::Start
            }

            //  ←
            //     --  L- F-  S-
            (Self::Horizontal, Direction::West) => {
                pipe == &Self::Horizontal
                    || pipe == &Self::BendL
                    || pipe == &Self::BendF
                    || pipe == &Self::Start
            }

            //  →
            //     --  -J -7  -S
            (Self::Horizontal, Direction::East) => {
                pipe == &Self::Horizontal
                    || pipe == &Self::BendJ
                    || pipe == &Self::Bend7
                    || pipe == &Self::Start
            }

            //    |  7  F  S
            // ↑  L  L  L  L
            (Self::BendL, Direction::North) => {
                pipe == &Self::Vertical
                    || pipe == &Self::Bend7
                    || pipe == &Self::BendF
                    || pipe == &Self::Start
            }

            //  →
            //     L-  LJ  L7  LS
            (Self::BendL, Direction::East) => {
                pipe == &Self::Horizontal
                    || pipe == &Self::BendJ
                    || pipe == &Self::Bend7
                    || pipe == &Self::Start
            }

            //    |  F  7  S
            // ↑  J  J  J  J
            (Self::BendJ, Direction::North) => {
                pipe == &Self::Vertical
                    || pipe == &Self::BendF
                    || pipe == &Self::Bend7
                    || pipe == &Self::Start
            }

            //  ←
            //     -J  FJ  LJ  SJ
            (Self::BendJ, Direction::West) => {
                pipe == &Self::Horizontal
                    || pipe == &Self::BendF
                    || pipe == &Self::BendL
                    || pipe == &Self::Start
            }

            // ↓  7  7  7  7
            //    |  J  L  S
            (Self::Bend7, Direction::South) => {
                pipe == &Self::Vertical
                    || pipe == &Self::BendJ
                    || pipe == &Self::BendL
                    || pipe == &Self::Start
            }

            //  ←
            //     -7  F7  L7  S7
            (Self::Bend7, Direction::West) => {
                pipe == &Self::Horizontal
                    || pipe == &Self::BendF
                    || pipe == &Self::BendL
                    || pipe == &Self::Start
            }

            // ↓  F  F  F  F
            //    |  J  L  S
            (Self::BendF, Direction::South) => {
                pipe == &Self::Vertical
                    || pipe == &Self::BendJ
                    || pipe == &Self::BendL
                    || pipe == &Self::Start
            }

            //  →
            //     F-  F7  FJ  FS
            (Self::BendF, Direction::East) => {
                pipe == &Self::Horizontal
                    || pipe == &Self::Bend7
                    || pipe == &Self::BendJ
                    || pipe == &Self::Start
            }

            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Ground,
    Pipe(Pipe),
}

impl Tile {
    fn is_start(&self) -> bool {
        matches!(self, Self::Pipe(Pipe::Start))
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ground,
            'S' => Self::Pipe(Pipe::Start),
            '|' => Self::Pipe(Pipe::Vertical),
            '-' => Self::Pipe(Pipe::Horizontal),
            'L' => Self::Pipe(Pipe::BendL),
            'J' => Self::Pipe(Pipe::BendJ),
            '7' => Self::Pipe(Pipe::Bend7),
            'F' => Self::Pipe(Pipe::BendF),
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

#[derive(Debug)]
pub struct Map<const W: usize, const H: usize> {
    data: [[Tile; W]; H],
    start: (usize, usize),
}

impl<const W: usize, const H: usize> Display for Map<W, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        Tile::Ground => ".".to_string(),
                        Tile::Pipe(pipe) => pipe.to_string(),
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Map<W, H> {
    pub fn get_tile(&self, pos: (usize, usize)) -> &Tile {
        let (x, y) = pos;
        &self.data[y][x]
    }
}

impl<const W: usize, const H: usize> FromStr for Map<W, H> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [[Tile::Ground; W]; H];
        let mut start = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile: Tile = c.into();
                if tile.is_start() {
                    start = (x, y);
                }
                data[y][x] = tile;
            }
        }
        Ok(Self { data, start })
    }
}

struct FindLoopState {
    visited_list: Vec<(usize, usize)>,
    visited_hash: HashSet<(usize, usize)>,
    current_pos: (usize, usize),
    came_from: Option<Direction>,
}

pub fn find_loop<const W: usize, const H: usize>(map: &Map<W, H>) -> Option<Vec<(usize, usize)>> {
    //find_loop_rec(map, map.start, vec![], HashSet::new(), None)

    let visited_list = vec![];
    let visited_hash = HashSet::new();
    let current_pos = map.start;
    let came_from = None;

    let mut stack: Vec<FindLoopState> = vec![FindLoopState {
        visited_list,
        visited_hash,
        current_pos,
        came_from,
    }];

    while let Some(state) = stack.pop() {
        let pos = state.current_pos;
        let visited_list = state.visited_list;
        let visited_hash = state.visited_hash;
        let came_from = state.came_from;
        let current_tile = map.get_tile(pos);

        match current_tile {
            Tile::Ground => return None,
            Tile::Pipe(curr_pipe) => {
                let mut visited_list = visited_list.clone();
                let mut visited_hash = visited_hash.clone();
                visited_list.push(pos);
                visited_hash.insert(pos);

                let steps_to_evaluate = [
                    (Direction::North, (0, -1), pos.1 > 0),
                    (Direction::East, (1, 0), pos.0 < W - 1),
                    (Direction::South, (0, 1), pos.1 < H - 1),
                    (Direction::West, (-1, 0), pos.0 > 0),
                ];

                for (direction, next_pos_delta, is_in_bounds) in steps_to_evaluate {
                    let next_pos = (
                        (pos.0 as isize + next_pos_delta.0) as usize,
                        (pos.1 as isize + next_pos_delta.1) as usize,
                    );
                    if is_in_bounds {
                        let next_tile = map.get_tile(next_pos);
                        if next_tile.is_start() || visited_hash.get(&next_pos).is_none() {
                            if let Tile::Pipe(next_pipe) = next_tile {
                                if curr_pipe.can_connect(direction, next_pipe) {
                                    if next_tile.is_start()
                                        && came_from.is_some()
                                        && !came_from.unwrap().is_opposite(&direction)
                                    {
                                        // FOUND THE LOOP
                                        return Some(visited_list);
                                    }

                                    // else keep exploring
                                    let new_state = FindLoopState {
                                        visited_list: visited_list.clone(),
                                        visited_hash: visited_hash.clone(),
                                        current_pos: next_pos,
                                        came_from: Some(direction),
                                    };
                                    stack.push(new_state);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn get_enclosed<const W: usize, const H: usize>(map: &Map<W, H>) -> Vec<(usize, usize)> {
    let loop_path = find_loop(map).unwrap();
    let mut closed_loop_path = loop_path.clone();
    closed_loop_path.push(loop_path[0]);
    let loop_cells: HashSet<(usize, usize)> = loop_path.iter().copied().collect();
    let mut enclosed = vec![];
    for y in 0..H {
        for x in 0..W {
            if !loop_cells.contains(&(x, y)) {
                // potentially enclosed.
                // to check if it is enclosed we walk the loop path and see if we wrap around the point
                // for all the points in pairs of the closed_loop_path
                // every time we pass vertically beside (x,y) and it is left to the path we flip the
                // is_enclosed flag
                let mut is_enclosed = false;
                closed_loop_path.windows(2).for_each(|step| {
                    let (_x1, y1) = step[0];
                    let (x2, y2) = step[1];
                    if (y2 > y) != (y1 > y) && x < x2 {
                        is_enclosed = !is_enclosed;
                    }
                });
                if is_enclosed {
                    enclosed.push((x, y));
                }
            }
        }
    }

    enclosed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_loop() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let map: Map<5, 5> = input.parse().unwrap();
        let loop_path = find_loop(&map);
        assert_eq!(
            loop_path,
            Some(vec![
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 3),
                (3, 3),
                (3, 2),
                (3, 1),
                (2, 1)
            ])
        );

        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let map: Map<5, 5> = input.parse().unwrap();
        let loop_path = find_loop(&map);
        assert_eq!(
            loop_path,
            Some(vec![
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 4),
                (1, 3),
                (2, 3),
                (3, 3),
                (4, 3),
                (4, 2),
                (3, 2),
                (3, 1),
                (3, 0),
                (2, 0),
                (2, 1),
                (1, 1),
                (1, 2)
            ])
        );
    }

    #[test]
    fn test_get_enclosed() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let map: Map<12, 12> = input.parse().unwrap();
        let enclosed = get_enclosed(&map);
        assert_eq!(enclosed.len(), 4);

        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let map: Map<20, 20> = input.parse().unwrap();
        let enclosed = get_enclosed(&map);
        assert_eq!(enclosed.len(), 8);
    }
}
