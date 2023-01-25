mod display;
mod parser;
mod program;
use display::*;
use parser::*;
use program::*;

pub fn part1(input: &str) -> i32 {
    let cmds = input.lines().map(|line| parse_line(line).unwrap().1);
    let program = Program::new(cmds);

    let important_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let values: Vec<Cpu> = program
        .filter(|cpu| important_cycles.contains(&(cpu.cycle)))
        .collect();

    values.iter().map(|cpu| cpu.cycle * cpu.x).sum::<i32>()
}

pub fn part2(input: &str) -> String {
    let mut display = Display::new();
    display.set_pixel(Pixel::On); // first pixel is always on (cycle 1)
    let cmds = input.lines().map(|line| parse_line(line).unwrap().1);
    let program = Program::new(cmds);

    for cpu_state in program {
        let pixel = if display.sprite_pos().contains(&(cpu_state.x)) {
            Pixel::On
        } else {
            Pixel::Off
        };
        display.set_pixel(pixel);
    }

    display.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("../sample_input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(SAMPLE_INPUT), 13140);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14540);
    }

    #[test]
    fn test_part2_example() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(part2(SAMPLE_INPUT), expected.to_string());
    }

    #[test]
    fn test_part2() {
        let expected = "####.#..#.####.####.####.#..#..##..####.
#....#..#....#.#.......#.#..#.#..#....#.
###..####...#..###....#..####.#......#..
#....#..#..#...#.....#...#..#.#.....#...
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.####.#....####.#..#..##..####.
";
        assert_eq!(part2(INPUT), expected.to_string());
    }
}
