use std::collections::HashSet;

#[derive(Debug)]
enum Instr {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

impl Instr {
    fn flip(&self) -> Self {
        match self {
            Instr::Nop(x) => Instr::Jmp(*x),
            Instr::Jmp(x) => Instr::Nop(*x),
            _ => panic!("Only Nop and Jmp can be flipped"),
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
        .expect("Cannot convert value to i16");

    match instr {
        "nop" => Instr::Nop(val),
        "jmp" => Instr::Jmp(val),
        "acc" => Instr::Acc(val),
        x => panic!("Unknown instruction: {}", x),
    }
}

fn execute_code(code: &[Instr], overwrite: Option<(usize, &Instr)>) -> (i32, usize) {
    let mut acc: i32 = 0;
    let mut i: usize = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&i) || i >= code.len() {
            break;
        }

        // original_instr is the natural next istr.
        // It might get overwritten if `overwrite` is set and the current line matches the overwrite line
        let original_instr = code
            .get(i)
            .expect(&*format!("Cannot find instruction at index {}", i));

        let instr = match overwrite {
            Some((pos, new_instr)) => {
                if pos == i {
                    new_instr
                } else {
                    original_instr
                }
            }
            _ => original_instr,
        };

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

    (acc, i)
}

pub fn part1(input: &str) -> i32 {
    let code: Vec<Instr> = input.lines().map(parse_line).collect();
    let (acc, _) = execute_code(&code, None);
    acc
}

pub fn part2(input: &str) -> i32 {
    let code: Vec<Instr> = input.lines().map(parse_line).collect();
    for i in 0..code.len() {
        let curr_instr = code.get(i).unwrap();

        if let Instr::Acc(_) = curr_instr {
            continue; // does not run simulation if the current line is an Acc instr (no mutation)
        }

        let (acc, last_line) = execute_code(&code, Some((i, &curr_instr.flip())));

        if last_line >= code.len() {
            // the code completed
            return acc;
        }
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
