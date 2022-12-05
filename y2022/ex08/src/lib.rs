mod cell_iter;
mod dir_iter;
mod map;
use map::*;

pub fn part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    let mut visible_trees: Vec<Vec<bool>> = map
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();

    let lines_of_sight = map.all_outer_lines_of_sight();

    for line in lines_of_sight {
        let mut prev: Option<u8> = None;
        for (row, col, curr_tree_height) in line {
            match prev {
                Some(prev_tree_height) if prev_tree_height >= *curr_tree_height => {
                    if *curr_tree_height == 9 {
                        break; // won't be able to see any other tree in this line of sight
                    }
                }
                _ => {
                    prev = Some(*curr_tree_height);
                    visible_trees[row][col] = true;
                }
            };
        }
    }

    visible_trees
        .iter()
        .map(|row| row.iter().filter(|&b| *b).count())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let map: Map = input.parse().unwrap();

    let mut scenic_score: Vec<Vec<u64>> =
        map.iter().map(|l| l.iter().map(|_| 0).collect()).collect();

    for (row, col, tree_house_height) in map.iter_cells() {
        let scores: [u64; 4] = map.iters_from_cell(row, col).map(|mut line| {
            line.next(); // skips the current tree itself
            let mut score = 0;
            for (_, _, watched_tree_height) in line {
                score += 1;
                if watched_tree_height >= tree_house_height {
                    break;
                }
            }
            score
        });
        scenic_score[row][col] = scores.iter().product();
    }

    *(scenic_score
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_example_part1() {
        // 30373   XXXXX
        // 25512   XXX_X
        // 65332   XX_XX
        // 33549   X_X_X
        // 35390   XXXXX
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1796);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 288120);
    }
}
