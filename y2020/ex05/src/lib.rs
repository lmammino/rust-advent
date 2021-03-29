use std::collections::HashSet;

fn seatid_to_int(input: &str) -> u16 {
    let bin_str = input
        .chars()
        .map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!("Unexpected character"),
        })
        .collect::<String>();
    u16::from_str_radix(&bin_str, 2).expect("Cannot convert bin_str to u16")
}

pub fn part1(input: &str) -> u16 {
    // The problem is basically asking us to convert a binary number to integer.
    // If we take an example input such as `FBFBBFFRLR` we can convert F and L to 0 and B and R to 1 and we get `0101100101`.
    // If we then get the integer representation of that binary we get the integer ID of a given seat.
    // This calculation is done in `seatid_to_int`, so we map every input line to that and then we take the max.
    let seat_ids = input.lines().map(seatid_to_int).collect::<Vec<u16>>();
    *seat_ids.iter().max().expect("List of ids cannot be empty")
}

pub fn part2(input: &str) -> u16 {
    let seat_ids = input.lines().map(seatid_to_int).collect::<HashSet<u16>>();

    for current_seat in &seat_ids {
        let next_seat = current_seat + 1;
        let next_next_seat = &next_seat + 1;
        if !seat_ids.contains(&next_seat) && seat_ids.contains(&next_next_seat)  {
            return next_seat
        }
    }

    panic!("Couldn't find our seat!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 835);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 649);
    }
}
