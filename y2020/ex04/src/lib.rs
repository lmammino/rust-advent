use regex::Regex;
use std::collections::{HashMap, HashSet};

mod validators;

pub fn part1(lines: Vec<&str>) -> u32 {
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
        let captures = re
            .captures_iter(&line)
            .map(|x| x.get(1).unwrap().as_str().to_string());

        let passport: HashSet<String> = captures.into_iter().collect();
        let expected_fields_present: HashSet<&String> =
            expected_fields.intersection(&passport).collect();

        if expected_fields_present.len() == expected_fields.len() {
            valid_passports += 1;
        }
    }

    valid_passports
}

pub fn part2(lines: Vec<&str>) -> u32 {
    let mut validators: HashMap<&str, Box<dyn validators::Validator>> = HashMap::new();
    validators.insert("byr", Box::new(validators::create_byr_validator()));
    validators.insert("iyr", Box::new(validators::create_iyr_validator()));
    validators.insert("eyr", Box::new(validators::create_eyr_validator()));
    validators.insert("hgt", Box::new(validators::create_hgt_validator()));
    validators.insert("hcl", Box::new(validators::create_hcl_validator()));
    validators.insert("ecl", Box::new(validators::create_ecl_validator()));
    validators.insert("pid", Box::new(validators::create_pid_validator()));
    // cid (Country ID) - ignored, missing or not.
    // This one is optional so we are not writing a validator for it

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

    let re = Regex::new(r"\b(\w+):(\S+)").unwrap();
    for line in raw_passports {
        let passport: HashMap<String, String> = re
            .captures_iter(&line)
            .map(|x| {
                let key = x.get(1).unwrap().as_str().to_string();
                let value = x.get(2).unwrap().as_str().to_string();
                (key, value)
            })
            .collect();

        let is_valid = validators.iter().all(|(field, validator)| {
            if let Some(field_value) = passport.get(*field) {
                return validator.validate(field_value);
            }
            false
        });

        if is_valid {
            valid_passports += 1;
        }
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
