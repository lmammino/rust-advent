pub fn part1(_input: &str) -> u32 {
    let lines: Vec<Vec<char>> = _input
            .lines()
            .map(|s| {
                s.chars().collect::<Vec<char>>()
            })
            .collect();
    let mut x = 0;
    let mut cnt = 0;
    for row in lines {
        let c = row.get(x % row.len()).unwrap();
        x += 3;
        if *c == '#' {
            cnt += 1;
        }
    }
    cnt
}

pub fn part2(_input: &str) -> u32 {
    let lines: Vec<Vec<char>> = _input
            .lines()
            .map(|s| {
                s.chars().collect::<Vec<char>>()
            })
            .collect();
    let mut xs = vec![0;5];
    let ixs = vec![1,3,5,7,1];
    let mut cnt = vec![0;5];
    for (j, row) in lines.iter().enumerate() {
        for i in 0..xs.len() {
            if i==4 && j%2==1 {  // jump the odd row when is the last rule (y must increment by 2)
                continue
            }
            let c = row.get(xs[i] % row.len()).unwrap();
            if *c == '#' {
                cnt[i] += 1;
            }
            xs[i] += ixs[i];
        }
    }
    cnt.iter().fold(1, |a, b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 299);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3621285278);
    }
}
