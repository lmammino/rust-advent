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
    let mut sorted_seat_ids = input.lines().map(seatid_to_int).collect::<Vec<u16>>();
    sorted_seat_ids.sort_unstable();

    for (i, seat_id) in sorted_seat_ids.iter().enumerate() {
        if sorted_seat_ids.get(i + 1).unwrap() - seat_id > 1 {
            return seat_id + 1;
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
