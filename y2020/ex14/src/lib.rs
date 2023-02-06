use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Instr<'a> {
    Mask(&'a str),
    Mem(u64, u64),
}

impl<'a> TryFrom<&'a str> for Instr<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.starts_with("mask") {
            Ok(Instr::Mask(&s[7..]))
        } else if s.starts_with("mem") {
            let addr_value = &mut s[4..].split("] = ");
            let addr: u64 = addr_value.next().unwrap().parse().unwrap();
            let value: u64 = addr_value.next().unwrap().parse().unwrap();
            Ok(Instr::Mem(addr, value))
        } else {
            Err(format!("Invalid line found: {s}"))
        }
    }
}

struct MaskParts {
    or_mask: u64,
    and_mask: u64,
    floating_bits: Vec<usize>,
}

impl FromStr for MaskParts {
    type Err = ();

    fn from_str(mask: &str) -> Result<Self, Self::Err> {
        let mut and_mask = 0;
        let mut or_mask = 0;
        let mut floating_bits: Vec<usize> = Vec::with_capacity(36);

        mask.chars().enumerate().for_each(|(i, c)| {
            and_mask <<= 1;
            or_mask <<= 1;
            if c == '1' {
                or_mask |= 1;
            }
            if c != '0' {
                and_mask |= 1;
            }
            if c == 'X' {
                floating_bits.push(35 - i)
            }
        });

        Ok(MaskParts {
            or_mask,
            and_mask,
            floating_bits,
        })
    }
}

pub fn part1(input: &str) -> u64 {
    let instr = input.lines().map(|x| x.try_into().unwrap());

    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask_parts: MaskParts = "X".repeat(36).parse().unwrap();
    for instruction in instr {
        match instruction {
            Instr::Mask(mask) => {
                mask_parts = mask.parse().unwrap();
            }
            Instr::Mem(key, val) => {
                mem.insert(key, val & mask_parts.and_mask | mask_parts.or_mask);
            }
        }
    }

    mem.values().sum()
}

pub fn part2(input: &str) -> u64 {
    let instr = input.lines().map(|x| x.try_into().unwrap());

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask_parts: MaskParts = "X".repeat(36).parse().unwrap();

    for instruction in instr {
        match instruction {
            Instr::Mask(mask) => {
                mask_parts = mask.parse().unwrap();
            }
            Instr::Mem(addr, val) => {
                let base_addr = addr | mask_parts.or_mask;
                // Now we have to do n inserts where n = 2.pow(numX)
                for i in 0..2_u64.pow(mask_parts.floating_bits.len() as u32) {
                    let mut final_addr = base_addr;
                    for (from_bit, to_bit) in mask_parts.floating_bits.iter().enumerate() {
                        if i & (1 << from_bit) != 0 {
                            final_addr |= 1 << to_bit;
                        } else {
                            final_addr &= !(1 << to_bit);
                        }
                    }

                    mem.insert(final_addr, val);
                }
            }
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let mask: Instr = "mask = 000000000000000000000000000000X1001X"
            .try_into()
            .unwrap();

        assert_eq!(mask, Instr::Mask("000000000000000000000000000000X1001X"));

        let mem: Instr = "mem[42] = 100".try_into().unwrap();
        assert_eq!(mem, Instr::Mem(42, 100));
    }

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
