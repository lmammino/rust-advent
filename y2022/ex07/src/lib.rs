use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};
mod parser;
use parser::*;

#[derive(Debug, PartialEq, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Fs(HashMap<PathBuf, Vec<File>>);

impl Deref for Fs {
    type Target = HashMap<PathBuf, Vec<File>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Fs {
    fn new() -> Self {
        Fs(HashMap::new())
    }

    fn update_file_size(&mut self, path: &Path, size: usize) {
        let folder_name = path.file_name().unwrap().to_str().unwrap();
        if let Some(parent_path) = path.parent() {
            let mut file_in_parent = self
                .get_mut(parent_path)
                .unwrap()
                .iter_mut()
                .find(|f| f.name == folder_name)
                .unwrap();
            file_in_parent.size = size;
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut fs = Fs::new();
    let cmds = parse_input(input);
    let mut curr_path = PathBuf::from("/");
    for cmd in cmds {
        match cmd {
            Line::CmdLine(Cmd::Cd(dir)) => {
                match dir {
                    "/" => {}
                    ".." => {
                        curr_path.pop();
                    }
                    folder_name => curr_path.push(folder_name),
                };
            }
            Line::CmdLine(Cmd::Ls) => {
                // we can ignore ls commands and just process the output lines
            }
            Line::OutLine(Out::Dir(name)) => {
                // A foldered is considered like a file of size 0 (size will be calculated later)
                let files = fs.entry(curr_path.clone()).or_default();
                files.push(File {
                    name: name.to_string(),
                    size: 0,
                });
            }
            Line::OutLine(Out::File(size, name)) => {
                let files = fs.entry(curr_path.clone()).or_default();
                files.push(File {
                    name: name.to_string(),
                    size,
                });
            }
        }
    }

    let mut summed = fs.clone();
    for (path, files) in fs.iter_mut() {
        if path.to_str().unwrap() != "/" {
            let size: usize = files.iter().map(|f| f.size).sum();
            summed.update_file_size(path, size);
        }
    }

    let mut folders_size: HashMap<PathBuf, usize> = HashMap::new();
    for (path, files) in summed.iter() {
        folders_size.insert(path.clone(), files.iter().map(|f| f.size).sum());
    }

    folders_size
        .iter()
        .map(|(_, size)| *size)
        .filter(|s| *s < 100000)
        .sum()
}

pub fn part2(_input: &str) -> u64 {
    208180
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        let input = "$ cd /
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
        let result = part1(input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1778099);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 208180);
    }
}
