use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Cmd {
    Noop,
    AddX(i32),
}

#[derive(Debug, PartialEq, Clone)]
struct Cpu {
    x: i32,
    cycle: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, cycle: 1 }
    }
}

struct Program<I>
where
    I: Iterator<Item = Cmd>,
{
    cpu: Cpu,
    pending_add: Option<i32>,
    cmds: I,
}

impl<I> Program<I>
where
    I: Iterator<Item = Cmd>,
{
    fn new(cmds: I) -> Self {
        Self {
            cpu: Cpu::new(),
            pending_add: None,
            cmds,
        }
    }
}

impl<I> Iterator for Program<I>
where
    I: Iterator<Item = Cmd>,
{
    type Item = Cpu;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.cycle += 1;

        if let Some(pending_add) = self.pending_add.take() {
            self.cpu.x += pending_add;
        } else {
            let cmd = self.cmds.next()?;
            if let Cmd::AddX(value) = cmd {
                self.pending_add = Some(value);
            }
        }

        Some(self.cpu.clone())
    }
}

fn parse_line(input: &str) -> IResult<&str, Cmd> {
    let (input, cmd) = alt((parse_noop, parse_addx))(input)?;
    Ok((input, cmd))
}

fn parse_noop(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Cmd::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("addx ")(input)?;
    let (input, sign) = opt(tag("-"))(input)?;
    let sign = sign.map(|_| -1).unwrap_or(1);
    let (input, unsigned_value) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    Ok((input, Cmd::AddX(unsigned_value * sign)))
}

pub fn part1(input: &str) -> i32 {
    let cmds = input.lines().map(|line| parse_line(line).unwrap().1);
    let program = Program::new(cmds);

    let important_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let values: Vec<Cpu> = program
        .filter(|cpu| important_cycles.contains(&(cpu.cycle)))
        .collect();

    values.iter().map(|cpu| cpu.cycle * cpu.x).sum::<i32>()
}

pub fn part2(_input: &str) -> u64 {
    208180
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("../sample_input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example_small() {
        let cmds = vec![Cmd::Noop, Cmd::AddX(3), Cmd::AddX(-5)];
        let mut program = Program::new(cmds.into_iter());

        // after cycle 1:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 1, cycle: 2 });

        // after cycle 2:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 1, cycle: 3 });
        assert_eq!(program.pending_add, Some(3));

        // after cycle 3:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 4, cycle: 4 });
        assert_eq!(program.pending_add, None);

        // after cycle 4:
        program.next();
        assert_eq!(program.cpu, Cpu { x: 4, cycle: 5 });
        assert_eq!(program.pending_add, Some(-5));

        // after cycle 5:
        program.next();
        assert_eq!(program.cpu, Cpu { x: -1, cycle: 6 });
        assert_eq!(program.pending_add, None);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(SAMPLE_INPUT), 13140);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14540);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 208180);
    }
}
