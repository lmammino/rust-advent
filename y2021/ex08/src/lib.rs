use std::collections::BTreeSet;

pub fn part1(input: &str) -> usize {
    // just needs to count how many entries are in output sizes that have length of 2,3,4 or 7
    // Note: in this case a BTreeSet is faster than an HashSet
    let unique_output_sizes: BTreeSet<usize> = BTreeSet::from([2, 3, 4, 7]);
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(_, raw_output_values)| {
            raw_output_values
                .split_whitespace()
                .filter(|s| unique_output_sizes.contains(&s.len()))
                .count()
        })
        .sum::<usize>()
}

pub fn parse_line(line: &str) -> usize {
    let (raw_readings, raw_output_values) = line.split_once(" | ").unwrap();
    let readings = raw_readings
        .split_whitespace()
        .map(|s| s.chars().collect::<BTreeSet<char>>());
    let output_values = raw_output_values
        .split_whitespace()
        .map(|s| s.chars().collect::<BTreeSet<char>>());

    let mut readings_by_parts: [Vec<BTreeSet<char>>; 8] = Default::default();
    for reading in readings {
        readings_by_parts[reading.len()].push(reading);
    }
    let mut mappings: [BTreeSet<char>; 10] = Default::default();
    // number 1: 2 parts
    mappings[1] = readings_by_parts[2].pop().unwrap();
    // number 7: 3 parts
    mappings[7] = readings_by_parts[3].pop().unwrap();
    // number 4: 4 parts
    mappings[4] = readings_by_parts[4].pop().unwrap();
    // number 8: 7 parts
    mappings[8] = readings_by_parts[7].pop().unwrap();

    // number 9: 6 parts, contains the parts in 4
    let pos = readings_by_parts[6]
        .iter()
        .position(|s| s.is_superset(&mappings[4]))
        .unwrap();
    mappings[9] = readings_by_parts[6].remove(pos);

    // number 0: 6 parts, contains the parts in 7
    let pos = readings_by_parts[6]
        .iter()
        .position(|s| s.is_superset(&mappings[7]))
        .unwrap();
    mappings[0] = readings_by_parts[6].remove(pos);

    // number 6: 6 parts (remaining item with 6 parts)
    mappings[6] = readings_by_parts[6].pop().unwrap();

    // number 3: 5 parts, contains the parts in 1
    let pos = readings_by_parts[5]
        .iter()
        .position(|s| s.is_superset(&mappings[1]))
        .unwrap();
    mappings[3] = readings_by_parts[5].remove(pos);

    // number 5: 5 parts, all its parts are contained in 6
    let pos = readings_by_parts[5]
        .iter()
        .position(|s| s.is_subset(&mappings[6]))
        .unwrap();
    mappings[5] = readings_by_parts[5].remove(pos);

    // number 2: 5 parts (remaining item with 5 parts)
    mappings[2] = readings_by_parts[5].pop().unwrap();

    output_values
        .rev()
        .enumerate()
        .map(|(i, v)| mappings.iter().position(|s| s.eq(&v)).unwrap() * 10_usize.pow(i as u32))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.lines().map(parse_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 355);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 983030);
    }

    #[test]
    fn test_parse_line() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(parse_line(line), 5353);
    }
}
