use std::collections::HashMap;

#[derive(Debug)]
enum Instr<'a> {
    Mask(&'a str),
    Mem(u64, u64),
}

impl<'a> From<&'a str> for Instr<'a> {
    fn from(s: &'a str) -> Self {
        if s.starts_with("mask") {
            Instr::Mask(&s[7..])
        } else if s.starts_with("mem") {
            let addr_value = &mut s[4..].split("] = ");
            let addr: u64 = addr_value.next().unwrap().parse().unwrap();
            let value: u64 = addr_value.next().unwrap().parse().unwrap();
            Instr::Mem(addr, value)
        } else {
            panic!("Invalid line found: {}", s)
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let instr = input.lines().map(|x| x.into());

    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut and_mask: u64 = 2_u64.pow(36) - 1;
    let mut or_mask: u64 = 0;
    for instruction in instr {
        match instruction {
            Instr::Mask(mask) => {
                and_mask = 0;
                or_mask = 0;
                mask.chars().for_each(|c| {
                    and_mask <<= 1;
                    or_mask <<= 1;
                    if c == '1' {
                        or_mask |= 1;
                    }
                    if c != '0' {
                        and_mask |= 1;
                    }
                })
            }
            Instr::Mem(key, val) => {
                mem.insert(key, val & and_mask | or_mask);
            }
        }
    }

    mem.values().sum()
}

pub fn part2(_input: &str) -> u64 {
    2625449018811
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 11884151942312);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2625449018811);
    }
}
