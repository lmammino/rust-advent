use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    vec,
};

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Tile {
    #[default]
    Empty,
    Mirror(MirrorType),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MirrorType {
    ForwardSlash,
    BackwardSlash,
    Horizontal,
    Vertical,
}

impl Tile {
    fn deflect(&self, from_dir: Direction) -> Vec<Direction> {
        match self {
            // Keep going in the same direction
            Tile::Empty => vec![from_dir],
            Tile::Mirror(MirrorType::ForwardSlash) => match from_dir {
                //   →/    GO UP
                //
                //   ↓
                //   /    GO LEFT
                //
                //   /←   GO DOWN
                //
                //   /
                //   ↑    GO RIGHT
                Direction::Right => vec![Direction::Up],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Up => vec![Direction::Right],
            },
            Tile::Mirror(MirrorType::BackwardSlash) => match from_dir {
                //   →\    GO DOWN
                //
                //   ↓
                //   \    GO RIGHT
                //
                //   \←   GO UP
                //
                //   \
                //   ↑    GO LEFT
                Direction::Right => vec![Direction::Down],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Up => vec![Direction::Left],
            },
            Tile::Mirror(MirrorType::Horizontal) => match from_dir {
                //   →-    KEEP RIGHT
                //
                //   ↓
                //   -    SPLIT LEFT AND RIGHT
                //
                //   -←   KEEP LEFT
                //
                //   -
                //   ↑    SPLIT LEFT AND RIGHT
                Direction::Right => vec![Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
                Direction::Up => vec![Direction::Left, Direction::Right],
            },
            Tile::Mirror(MirrorType::Vertical) => match from_dir {
                //   →|    SPLIT UP AND DOWN
                //
                //   ↓
                //   |    KEEP DOWN
                //
                //   |←   SPLIT UP AND DOWN
                //
                //   |
                //   ↑    KEEP UP
                Direction::Right => vec![Direction::Up, Direction::Down],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Up => vec![Direction::Up],
            },
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::Mirror(MirrorType::ForwardSlash),
            '\\' => Tile::Mirror(MirrorType::BackwardSlash),
            '-' => Tile::Mirror(MirrorType::Horizontal),
            '|' => Tile::Mirror(MirrorType::Vertical),
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Mirror(MirrorType::ForwardSlash) => write!(f, "/"),
            Tile::Mirror(MirrorType::BackwardSlash) => write!(f, "\\"),
            Tile::Mirror(MirrorType::Horizontal) => write!(f, "-"),
            Tile::Mirror(MirrorType::Vertical) => write!(f, "|"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Map<const W: usize, const H: usize> {
    tiles: [[Tile; W]; H],
}

impl<const W: usize, const H: usize> Map<W, H> {
    pub fn new(input: &str) -> Self {
        let mut tiles = [[Tile::Empty; W]; H];
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                tiles[y][x] = Tile::from(c);
            }
        }
        Self { tiles }
    }

    pub fn simulate(&self, start_pos: Position, start_dir: Direction) -> HashSet<Position> {
        let mut energized_pos = HashSet::new();

        let mut seen_configs: HashSet<(Position, Direction)> = HashSet::new();
        let mut rays: Vec<(Position, Direction)> = vec![(start_pos, start_dir)];

        while let Some(((ray_pos_x, ray_pos_y), ray_dir)) = rays.pop() {
            let current_config = ((ray_pos_x, ray_pos_y), ray_dir);
            if seen_configs.contains(&current_config) {
                continue;
            }
            seen_configs.insert(current_config);
            energized_pos.insert((ray_pos_x, ray_pos_y));
            let current_tile = self.tiles[ray_pos_y][ray_pos_x];
            let new_dirs = current_tile.deflect(ray_dir);
            for new_dir in new_dirs {
                let (new_dir_offset_x, new_dir_offset_y) = new_dir.offset();
                let new_ray_pos_x = ray_pos_x as isize + new_dir_offset_x;
                let new_ray_pos_y = ray_pos_y as isize + new_dir_offset_y;
                if new_ray_pos_x >= 0
                    && new_ray_pos_x < W as isize
                    && new_ray_pos_y >= 0
                    && new_ray_pos_y < H as isize
                {
                    rays.push(((new_ray_pos_x as usize, new_ray_pos_y as usize), new_dir));
                }
            }
        }

        energized_pos
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn test_simulate() {
        let map = Map::<10, 10>::new(EXAMPLE_INPUT);
        let energized_pos = map.simulate((0, 0), Direction::Right);
        assert_eq!(energized_pos.len(), 46);
    }
}
