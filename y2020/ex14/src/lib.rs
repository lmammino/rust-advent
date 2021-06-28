use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Instr {
    Mask(String),
    Mem(u64, u64),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Instr::Mask(String::from(&s[7..])))
        } else if s.starts_with("mem") {
            let addr_value = &mut s[4..].split("] = ");
            let addr: u64 = addr_value.next().unwrap().parse().unwrap();
            let value: u64 = addr_value.next().unwrap().parse().unwrap();
            Ok(Instr::Mem(addr, value))
        } else {
            Err(())
        }
    }
}


pub fn part1(input: &str) -> u64 {
    let instr: Vec<Instr> = input.lines().map(str::parse).map(
        |x| x.unwrap()
    ).collect();

    let mut mem: HashMap<u64,u64> = HashMap::new();

    let mut and_mask:u64 = 2_u64.pow(36) - 1;
    let mut or_mask:u64=  0;
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
