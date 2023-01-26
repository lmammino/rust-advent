use std::borrow::Cow; // 🐮
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Instr {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

impl Instr {
    fn change(&self) -> (Self, Self) {
        match self {
            Instr::Nop(x) => (Instr::Jmp(*x), self.to_owned()),
            Instr::Jmp(x) => (Instr::Nop(*x), self.to_owned()),
            Instr::Acc(x) => (Instr::Acc(*x), self.to_owned()),
        }
    }
}

fn parse_line(line: &str) -> Instr {
    let mut parts = line.split(' ');
    let instr = parts.next().expect("Instruction not found");
    let val: i32 = parts
        .next()
        .expect("Value not found")
        .parse()
        .expect("Cannot convert value to i32");

    match instr {
        "nop" => Instr::Nop(val),
        "jmp" => Instr::Jmp(val),
        "acc" => Instr::Acc(val),
        x => panic!("Unknown instruction: {}", x),
    }
}

fn execute_code(code: &[Instr]) -> (i32, bool) {
    let mut acc: i32 = 0;
    let mut i: usize = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&i) || i >= code.len() {
            break;
        }

        let instr = code
            .get(i)
            .unwrap_or_else(|| panic!("Cannot find instruction at index {}", i));

        visited.insert(i);

        match instr {
            Instr::Nop(_) => i += 1,
            Instr::Acc(x) => {
                acc += *x;
                i += 1;
            }
            Instr::Jmp(x) => {
                if *x < 0 {
                    i = i.wrapping_sub(x.unsigned_abs() as usize);
                } else {
                    i = i.wrapping_add(x.unsigned_abs() as usize);
                }
            }
        }
    }

    (acc, i >= code.len())
}

pub fn part1(input: &str) -> i32 {
    let code: Vec<Instr> = input.lines().map(parse_line).collect();
    let (acc, _) = execute_code(&code);
    acc
}

pub fn part2(input: &str) -> i32 {
    let code: Vec<Instr> = input.lines().map(parse_line).collect();
    let mut code_variations = Cow::from(code);

    for i in 0..code_variations.len() {
        // changes the current instruction
        let (new_instr, orig_instr) = code_variations.get(i).unwrap().change();

        if let Instr::Acc(_) = new_instr {
            continue; // does not run simulation if the current line is an Acc instr (no change)
        }

        code_variations.to_mut()[i] = new_instr;

        let (acc, completed) = execute_code(&code_variations);

        if completed {
            return acc;
        }

        // restores latest change (there will be only one change at the time)
        code_variations.to_mut()[i] = orig_instr;
    }

    panic!("Could not find the instruction to swap");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1816);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1149);
    }
}
