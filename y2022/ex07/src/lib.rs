mod fs;
mod parser;
use fs::*;
use parser::*;

pub fn part1(input: &str) -> usize {
    let fs: Fs = parse_input(input).collect();

    fs.get_folders_size()
        .iter()
        .map(|(_, size)| *size)
        .filter(|s| *s < 100000)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let fs: Fs = parse_input(input).collect();

    let max_space = 70000000_usize;
    let needed_space = 30000000_usize;
    let used_space = fs.used_space();
    let free_space = max_space - used_space;
    let space_to_free = needed_space - free_space;

    fs.get_folders_size()
        .iter()
        .map(|(_, size)| *size)
        .filter(|s| *s >= space_to_free)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1_example() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1778099);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(TEST_INPUT), 24933642);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1623571);
    }
}
