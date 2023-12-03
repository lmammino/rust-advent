use std::str::FromStr;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::complete,
    multi::separated_list1, IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn minimum_set(&self) -> CubeSet {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in &self.sets {
            if set.red > max_red {
                max_red = set.red;
            }
            if set.green > max_green {
                max_green = set.green;
            }
            if set.blue > max_blue {
                max_blue = set.blue;
            }
        }

        CubeSet {
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
struct Cubes {
    num: u32,
    color: Color,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = u32(input)?;

    Ok((input, id))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    let color = Color::from_str(color).unwrap();

    Ok((input, color))
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    let (input, num) = u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;

    Ok((input, Cubes { num, color }))
}

fn parse_cubeset(input: &str) -> IResult<&str, CubeSet> {
    let (input, cubes) = separated_list1(tag(", "), parse_cubes)(input)?;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube in cubes {
        match cube.color {
            Color::Red => red += cube.num,
            Color::Green => green += cube.num,
            Color::Blue => blue += cube.num,
        }
    }

    Ok((input, CubeSet { red, green, blue }))
}

fn parse_line(line: &str) -> IResult<&str, Game> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (input, game_id) = parse_game_id(line)?;
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = complete(separated_list1(tag("; "), parse_cubeset))(input)?;

    Ok((input, Game { id: game_id, sets }))
}

pub fn part1(input: &str) -> u32 {
    // assume you have "12 red cubes, 13 green cubes, and 14 blue cubes" in the bag
    input
        .lines()
        .filter_map(|line: &str| {
            let (_, game) = parse_line(line).unwrap();
            for set in &game.sets {
                if set.red > 12 || set.green > 13 || set.blue > 14 {
                    return None;
                }
            }

            Some(game)
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, game) = parse_line(line).unwrap();
            let minimum_set = game.minimum_set();
            minimum_set.power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (input, game) = parse_line(line).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            game,
            Game {
                id: 1,
                sets: vec![
                    CubeSet {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2505);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 70265);
    }
}
