use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64,
    combinator::{complete, eof, opt},
    multi::{many_till, separated_list1},
    IResult,
};
use std::ops::Range;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Mapping,
    soil_to_fertilizer_map: Mapping,
    fertilizer_to_water_map: Mapping,
    water_to_light_map: Mapping,
    light_to_temperature_map: Mapping,
    temperature_to_humidity_map: Mapping,
    humidity_to_location_map: Mapping,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let value = self.seed_to_soil_map.map(seed);
        let value = self.soil_to_fertilizer_map.map(value);
        let value = self.fertilizer_to_water_map.map(value);
        let value = self.water_to_light_map.map(value);
        let value = self.light_to_temperature_map.map(value);
        let value = self.temperature_to_humidity_map.map(value);
        let value = self.humidity_to_location_map.map(value);
        value
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MappingEntry {
    range: Range<u64>,
    delta: i64,
}

impl MappingEntry {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        let range = source_range_start..(source_range_start + range_length);
        let delta = destination_range_start as i64 - source_range_start as i64;
        Self { range, delta }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    entries: Vec<MappingEntry>,
}

impl Mapping {
    fn map(&self, value: u64) -> u64 {
        for entry in &self.entries {
            if entry.range.contains(&value) {
                return (value as i64 + entry.delta) as u64;
            }
        }
        value
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    // seeds: 3943078016 158366385 481035699 103909769 3553279107 15651230 3322093486 189601966 2957349913 359478652 924423181 691197498 2578953067 27362630 124747783 108079254 1992340665 437203822 2681092979 110901631

    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), u64)(input)?;

    Ok((input, seeds))
}

fn parse_mapping_entry(input: &str) -> IResult<&str, MappingEntry> {
    // 2702707184 1771488746 32408643
    let (input, destination_range_start) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, source_range_start) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, range_length) = u64(input)?;
    // add an optional new line consumption
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((
        input,
        MappingEntry::new(destination_range_start, source_range_start, range_length),
    ))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, (entries, _)) = many_till(parse_mapping_entry, alt((tag("\n"), eof)))(input)?;
    Ok((input, Mapping { entries }))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, _) = tag("seed-to-soil map:\n")(input)?;
    let (input, seed_to_soil_map) = parse_mapping(input)?;
    let (input, _) = tag("soil-to-fertilizer map:\n")(input)?;
    let (input, soil_to_fertilizer_map) = parse_mapping(input)?;
    let (input, _) = tag("fertilizer-to-water map:\n")(input)?;
    let (input, fertilizer_to_water_map) = parse_mapping(input)?;
    let (input, _) = tag("water-to-light map:\n")(input)?;
    let (input, water_to_light_map) = parse_mapping(input)?;
    let (input, _) = tag("light-to-temperature map:\n")(input)?;
    let (input, light_to_temperature_map) = parse_mapping(input)?;
    let (input, _) = tag("temperature-to-humidity map:\n")(input)?;
    let (input, temperature_to_humidity_map) = parse_mapping(input)?;
    let (input, _) = tag("humidity-to-location map:\n")(input)?;
    let (input, humidity_to_location_map) = complete(parse_mapping)(input)?;

    let almanac = Almanac {
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    };
    Ok((input, almanac))
}

pub fn part1(input: &str) -> u64 {
    let (_, almanac) = parse_almanac(input).unwrap();
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    // TODO: this solution is currently very slow (100+ seconds in --release mode)
    //   figure out how to refactor it to be faster
    // ideas:
    //  - merge all the transformation layers into one
    //  - find a way to index the ranges more efficiently (so that we don't have to scan them all for every seed)
    let (_, almanac) = parse_almanac(input).unwrap();
    almanac
        .seeds
        .as_slice()
        .chunks(2)
        .flat_map(|r| {
            let range_start = r[0];
            let range_len = r[1];
            let range = range_start..(range_start + range_len);
            range.map(|seed| almanac.seed_to_location(seed))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_mapping() {
        let entries = vec![MappingEntry::new(50, 98, 2), MappingEntry::new(52, 50, 48)];
        let mapper = Mapping { entries };

        assert_eq!(mapper.map(98), 50);
        assert_eq!(mapper.map(99), 51);
        assert_eq!(mapper.map(53), 55);
        // not mapped
        assert_eq!(mapper.map(10), 10);
    }

    #[test]
    fn test_parse_almanac() {
        let (_, almanac) = parse_almanac(EXAMPLE_INPUT).unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            almanac.seed_to_soil_map.entries,
            vec![MappingEntry::new(50, 98, 2), MappingEntry::new(52, 50, 48)]
        );
        assert_eq!(
            almanac.soil_to_fertilizer_map.entries,
            vec![
                MappingEntry::new(0, 15, 37),
                MappingEntry::new(37, 52, 2),
                MappingEntry::new(39, 0, 15)
            ]
        );
        assert_eq!(
            almanac.fertilizer_to_water_map.entries,
            vec![
                MappingEntry::new(49, 53, 8),
                MappingEntry::new(0, 11, 42),
                MappingEntry::new(42, 0, 7),
                MappingEntry::new(57, 7, 4)
            ]
        );
        assert_eq!(
            almanac.water_to_light_map.entries,
            vec![MappingEntry::new(88, 18, 7), MappingEntry::new(18, 25, 70)]
        );
        assert_eq!(
            almanac.light_to_temperature_map.entries,
            vec![
                MappingEntry::new(45, 77, 23),
                MappingEntry::new(81, 45, 19),
                MappingEntry::new(68, 64, 13)
            ]
        );
        assert_eq!(
            almanac.temperature_to_humidity_map.entries,
            vec![MappingEntry::new(0, 69, 1), MappingEntry::new(1, 0, 69)]
        );
        assert_eq!(
            almanac.humidity_to_location_map.entries,
            vec![MappingEntry::new(60, 56, 37), MappingEntry::new(56, 93, 4)]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 424490994);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 15290096);
    }
}
