use regex::Regex;
use std::collections::{HashMap, HashSet};

fn part1(lines: Vec<&str>) -> u32 {
    let expected_fields: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut raw_passports: Vec<String> = vec![];
    let temp: Vec<&str> = vec![];

    let last_passports = lines.iter().fold(temp, |mut acc, x| {
        if x.trim() == "" {
            raw_passports.push(acc.join(" "));
            return vec![];
        } else {
            acc.push(x);
            acc
        }
    });

    raw_passports.extend::<Vec<String>>(
        last_passports
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
    );

    let mut valid_passports = 0;

    let re = Regex::new(r"\b(\w+):").unwrap();
    for line in raw_passports {
        let captures: Vec<String> = re
            .captures_iter(&line)
            .map(|x| x.get(1).unwrap().as_str().to_string())
            .collect();

        let passport: HashSet<String> = captures.into_iter().collect();
        let expected_fields_present: HashSet<&String> =
            expected_fields.intersection(&passport).collect();

        if expected_fields_present.len() == expected_fields.len() {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn part2(lines: Vec<&str>) -> u32 {
    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    */
    type Validator = fn(&String) -> bool;
    let mut validators: HashMap<String, Validator> = HashMap::new();
    validators.insert(String::from("byr"), |_x: &String| {
        // TODO: implement business logic for first validator
        true
    });

    let mut raw_passports: Vec<String> = vec![];
    let temp: Vec<&str> = vec![];

    let last_passports = lines.iter().fold(temp, |mut acc, x| {
        if x.trim() == "" {
            raw_passports.push(acc.join(" "));
            return vec![];
        } else {
            acc.push(x);
            acc
        }
    });

    raw_passports.extend::<Vec<String>>(
        last_passports
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
    );

    let mut valid_passports = 0;

    let re = Regex::new(r"\b(\w+):(\w+)").unwrap();
    for line in raw_passports {
        let passport: HashMap<String, String> = re
            .captures_iter(&line)
            .map(|x| {
                let key = x.get(1).unwrap().as_str().to_string();
                let value = x.get(2).unwrap().as_str().to_string();
                (key, value)
            })
            .collect();

        println!("{:?}", passport);

        // let passport: HashSet<String> = captures.into_iter().collect();
        // let expected_fields_present: HashSet<&String> =
        //     expected_fields.intersection(&passport).collect();

        // if expected_fields_present.len() == expected_fields.len() {
        //     valid_passports += 1;
        // }
    }

    valid_passports
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        let values: Vec<&str> = input.lines().collect();
        assert_eq!(part1(values), 219);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        let values: Vec<&str> = input.lines().collect();
        assert_eq!(part2(values), 127);
    }
}
