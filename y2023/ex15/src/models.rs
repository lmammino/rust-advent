use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::complete,
    IResult,
};

#[derive(Debug, Default)]
pub struct Lens<'a> {
    pub name: &'a str,
    pub focal_length: u64,
}

impl<'a> Lens<'a> {
    pub fn new(name: &'a str, focal_length: u64) -> Self {
        Self { name, focal_length }
    }
}

impl<'a> Display for Lens<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.name, self.focal_length)
    }
}

#[derive(Debug, Default)]
pub struct Box<'a> {
    pub lenses: Vec<Lens<'a>>,
}

impl<'a> Display for Box<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for lens in &self.lenses {
            write!(f, " {}", lens)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Boxes<'a> {
    boxes: Vec<Box<'a>>,
}

impl<'a> Display for Boxes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, box_) in self.boxes.iter().enumerate() {
            if !box_.lenses.is_empty() {
                writeln!(f, "Box {}:{}", i, box_)?;
            }
        }
        Ok(())
    }
}

impl<'a> Deref for Boxes<'a> {
    type Target = Vec<Box<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.boxes
    }
}

impl<'a> DerefMut for Boxes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boxes
    }
}

impl<'a> Boxes<'a> {
    pub fn new() -> Self {
        let mut boxes: Vec<Box<'a>> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Box::default());
        }
        Self { boxes }
    }

    pub fn focusing_power(&self) -> u64 {
        self.iter()
            .enumerate()
            .flat_map(|(box_id, box_)| {
                box_.lenses
                    .iter()
                    .enumerate()
                    .map(move |(lens_slot, lens)| {
                        (
                            (box_id + 1) as u64,
                            (lens_slot + 1) as u64,
                            lens.focal_length,
                        )
                    })
            })
            // .inspect(|a| {
            //     println!("{:?}", a);
            // })
            .map(|(box_id, slot_id, focal_length)| box_id * slot_id * focal_length)
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandOperation {
    Remove,
    Add(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command<'a> {
    pub label: &'a str,
    pub command: CommandOperation,
}

fn parse_command_add<'a>(input: &'a str) -> IResult<&'a str, Command<'a>> {
    let (input, label) = alpha1(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, focal_length) = u32(input)?;

    Ok((
        input,
        Command {
            label,
            command: CommandOperation::Add(focal_length as u8),
        },
    ))
}

fn parse_command_remove<'a>(input: &'a str) -> IResult<&'a str, Command<'a>> {
    let (input, label) = alpha1(input)?;
    let (input, _) = tag("-")(input)?;

    Ok((
        input,
        Command {
            label,
            command: CommandOperation::Remove,
        },
    ))
}

pub fn parse_command<'a>(input: &'a str) -> IResult<&'a str, Command<'a>> {
    let (input, command) = complete(alt((parse_command_add, parse_command_remove)))(input)?;
    Ok((input, command))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let (_, command_add) = parse_command("rn=1").unwrap();
        assert_eq!(
            command_add,
            Command {
                label: "rn",
                command: CommandOperation::Add(1)
            }
        );
        let (_, command_remove) = parse_command("rn-").unwrap();
        assert_eq!(
            command_remove,
            Command {
                label: "rn",
                command: CommandOperation::Remove
            }
        );
    }
}
