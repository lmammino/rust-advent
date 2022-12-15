use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Map<const W: usize, const H: usize> {
    pub(crate) grid: [[usize; W]; H],
    pub(crate) scenic_points: Vec<Pos>,
    pub(crate) start: Pos,
    pub(crate) end: Pos,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Edge {
    pos: Pos,
    cost: usize,
    prev: Option<Pos>,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub(crate) struct DijkstraResult(HashMap<Pos, (usize, Option<Pos>)>);

impl DijkstraResult {
    fn new() -> Self {
        Self(HashMap::new())
    }

    pub(crate) fn should_explore(&mut self, pos: &Pos, cost: usize, prev_pos: Option<Pos>) -> bool {
        let entry = self.0.entry(*pos).or_insert((usize::MAX, None));
        if cost < entry.0 {
            // found a better path
            entry.0 = cost;
            entry.1 = prev_pos;
            return true;
        }
        false
    }

    pub(crate) fn path_to(&self, end: &Pos) -> VecDeque<Pos> {
        let mut path: VecDeque<Pos> = VecDeque::new();
        if !self.0.contains_key(end) {
            return path;
        }

        path.push_back(*end);
        let mut current = end;
        while let Some((_, Some(prev))) = self.0.get(current) {
            path.push_front(*prev);
            current = prev;
        }
        path
    }
}

impl Deref for DijkstraResult {
    type Target = HashMap<Pos, (usize, Option<Pos>)>;

    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}

impl<const W: usize, const H: usize> Map<W, H> {
    fn walkable_neighbours(&self, pos: &Pos) -> Vec<Pos> {
        let mut possible_steps = Vec::new();
        let current_height = self.grid[pos.y][pos.x];
        // left
        if pos.x > 0 && self.grid[pos.y][pos.x - 1] <= current_height + 1 {
            possible_steps.push(Pos {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        // right
        if pos.x < W - 1 && self.grid[pos.y][pos.x + 1] <= current_height + 1 {
            possible_steps.push(Pos {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        // up
        if pos.y > 0 && self.grid[pos.y - 1][pos.x] <= current_height + 1 {
            possible_steps.push(Pos {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        // down
        if pos.y < H - 1 && self.grid[pos.y + 1][pos.x] <= current_height + 1 {
            possible_steps.push(Pos {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        possible_steps
    }

    pub(crate) fn dijkstra(&self, from: &Pos) -> DijkstraResult {
        let mut result = DijkstraResult::new();
        let mut active_nodes: BinaryHeap<Edge> = Default::default();

        let initial_path = Edge {
            pos: *from,
            cost: 0,
            prev: None,
        };
        active_nodes.push(initial_path);

        while let Some(Edge { pos, cost, prev }) = active_nodes.pop() {
            // check if we already found a better path
            if result.should_explore(&pos, cost, prev) {
                // check the neighbours
                for new_pos in self.walkable_neighbours(&pos) {
                    let next = Edge {
                        cost: cost + 1,
                        pos: new_pos,
                        prev: Some(pos),
                    };

                    active_nodes.push(next);
                }
            }
        }

        result
    }
}

impl<const W: usize, const H: usize> FromStr for Map<W, H> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [[0_usize; W]; H];
        let mut scenic_points = Vec::new();
        let lines = s.lines();
        let mut start: Option<Pos> = None;
        let mut end: Option<Pos> = None;
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                let elevation = match c {
                    'S' => {
                        start = Some(Pos { x, y });
                        scenic_points.push(Pos { x, y });
                        0 // like 'a'
                    }
                    'E' => {
                        end = Some(Pos { x, y });
                        'z' as usize - 'a' as usize
                    }
                    'a' => {
                        scenic_points.push(Pos { x, y });
                        0
                    }
                    'b'..='z' => c as usize - 'a' as usize,
                    _ => return Err(format!("Unexpected character '{}'", c)),
                };
                grid[y][x] = elevation;
            }
        }

        if start.is_none() {
            return Err("Start not found".to_string());
        }

        if end.is_none() {
            return Err("End not found".to_string());
        }

        Ok(Map {
            grid,
            scenic_points,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_parse_map() {
        let map: Map<8, 5> = SAMPLE_INPUT.parse().unwrap();
        assert_eq!(
            map,
            Map {
                grid: [
                    [0, 0, 1, 16, 15, 14, 13, 12],
                    [0, 1, 2, 17, 24, 23, 23, 11],
                    [0, 2, 2, 18, 25, 25, 23, 10],
                    [0, 2, 2, 19, 20, 21, 22, 9],
                    [0, 1, 3, 4, 5, 6, 7, 8]
                ],
                scenic_points: vec![
                    Pos { x: 0, y: 0 },
                    Pos { x: 1, y: 0 },
                    Pos { x: 0, y: 1 },
                    Pos { x: 0, y: 2 },
                    Pos { x: 0, y: 3 },
                    Pos { x: 0, y: 4 }
                ],
                start: Pos { x: 0, y: 0 },
                end: Pos { x: 5, y: 2 }
            }
        );
    }

    #[test]
    fn test_dijkstra() {
        let map: Map<8, 5> = SAMPLE_INPUT.parse().unwrap();
        let result = map.dijkstra(&map.start);
        let end_edge = result.get(&(map.end)).expect("End not found");
        assert_eq!(end_edge.0, 31); // checks the cost
        let full_path = result.path_to(&(map.end));
        // checks the entire path back to the start
        assert_eq!(full_path.len(), 32); // <- includes the start node
        assert_eq!(full_path[0], map.start);
    }
}
