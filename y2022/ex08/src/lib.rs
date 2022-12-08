pub fn part1(input: &str) -> usize {
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();

    // from top to bottom (column by column)
    for col in 0..cols {
        let mut prev: Option<u8> = None;
        for row in 0..rows {
            let curr_tree_height = trees[row][col];
            match prev {
                Some(prev_tree_height) if prev_tree_height >= curr_tree_height => {
                    if curr_tree_height == 9 {
                        break; // won't be able to see any other tree in this line of sight
                    }
                    continue; // can't see the next tree so skip to the next one
                }
                _ => {
                    prev = Some(curr_tree_height);
                    visible_trees[row][col] = true;
                }
            };
        }
    }

    // from bottom to top (column by column)
    for col in 0..cols {
        let mut prev: Option<u8> = None;
        for row in (0..rows).rev() {
            let curr_tree_height = trees[row][col];
            match prev {
                Some(prev_tree_height) if prev_tree_height >= curr_tree_height => {
                    if curr_tree_height == 9 {
                        break; // won't be able to see any other tree in this line of sight
                    }
                    continue; // can't see the next tree so skip to the next one
                }
                _ => {
                    prev = Some(curr_tree_height);
                    visible_trees[row][col] = true;
                }
            };
        }
    }

    // from left to right (row by row)
    for row in 0..rows {
        let mut prev: Option<u8> = None;
        for col in 0..cols {
            let curr_tree_height = trees[row][col];
            match prev {
                Some(prev_tree_height) if prev_tree_height >= curr_tree_height => {
                    if curr_tree_height == 9 {
                        break; // won't be able to see any other tree in this line of sight
                    }
                    continue; // can't see the next tree so skip to the next one
                }
                _ => {
                    prev = Some(curr_tree_height);
                    visible_trees[row][col] = true;
                }
            };
        }
    }

    // from right to left (row by row)
    for row in 0..rows {
        let mut prev: Option<u8> = None;
        for col in (0..cols).rev() {
            let curr_tree_height = trees[row][col];
            match prev {
                Some(prev_tree_height) if prev_tree_height >= curr_tree_height => {
                    if curr_tree_height == 9 {
                        break; // won't be able to see any other tree in this line of sight
                    }
                    continue; // can't see the next tree so skip to the next one
                }
                _ => {
                    prev = Some(curr_tree_height);
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
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut scenic_score: Vec<Vec<u64>> = trees
        .iter()
        .map(|l| l.iter().map(|_| 0).collect())
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let tree_height = trees[row][col];

            // up score
            let mut y = row;
            let mut up_score = 0;
            while y > 0 {
                y -= 1;
                up_score += 1;
                let other_tree_height = trees[y][col];
                if other_tree_height >= tree_height {
                    break;
                }
            }
            if up_score == 0 {
                continue;
            }

            // down score
            let mut y = row;
            let mut down_score = 0;
            while y < rows - 1 {
                y += 1;
                down_score += 1;
                let other_tree_height = trees[y][col];
                if other_tree_height >= tree_height {
                    break;
                }
            }
            if down_score == 0 {
                continue;
            }

            // left score
            let mut x = col;
            let mut left_score = 0;
            while x > 0 {
                x -= 1;
                left_score += 1;
                let other_tree_height = trees[row][x];
                if other_tree_height >= tree_height {
                    break;
                }
            }
            if left_score == 0 {
                continue;
            }

            // right score
            let mut x = col;
            let mut right_score = 0;
            while x < cols - 1 {
                x += 1;
                right_score += 1;
                let other_tree_height = trees[row][x];
                if other_tree_height >= tree_height {
                    break;
                }
            }
            if right_score == 0 {
                continue;
            }

            scenic_score[row][col] = up_score * down_score * left_score * right_score;
        }
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
