use models::{parse_command, Boxes};

use crate::models::{CommandOperation, Lens};

mod models;

fn hash(s: &str) -> u64 {
    let mut h = 0;
    for c in s.chars() {
        let ascii_value = c as u64;
        h += ascii_value;
        h *= 17;
        h %= 256;
    }
    h
}

pub fn part1(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

pub fn part2(input: &str) -> u64 {
    let mut boxes = Boxes::new();
    input.split(',').for_each(|s| {
        let (_, command) = parse_command(s).unwrap();
        let box_id = hash(command.label) as usize;
        let box_ = boxes.get_mut(box_id).unwrap();
        let same_label_idx = box_
            .lenses
            .iter()
            .position(|lens| lens.name == command.label);
        match command.command {
            CommandOperation::Add(focal_length) => {
                let lens = Lens::new(command.label, focal_length as u64);
                if let Some(idx) = same_label_idx {
                    // replace the lens if it exists
                    box_.lenses[idx] = lens;
                } else {
                    // append it otherwise
                    box_.lenses.push(lens);
                }
            }
            CommandOperation::Remove => {
                if let Some(idx) = same_label_idx {
                    box_.lenses.remove(idx);
                }
            }
        }
    });

    boxes.focusing_power()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const INPUT_EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 519603);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(INPUT_EXAMPLE), 145);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 244342);
    }
}
