use regex::Regex;
use std::collections::{HashMap, HashSet};

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

pub fn part2(lines: Vec<&str>) -> u32 {
    type Validator = fn(&String) -> bool;
    let mut validators: HashMap<String, Validator> = HashMap::new();
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    validators.insert(String::from("byr"), |byr: &String| {
        if let Ok(year) = byr.parse::<u16>() {
            return (1920..=2002).contains(&year);
        }
        false
    });
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    validators.insert(String::from("iyr"), |iyr: &String| {
        if let Ok(year) = iyr.parse::<u16>() {
            return (2010..=2020).contains(&year);
        }
        false
    });
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    validators.insert(String::from("eyr"), |eyr: &String| {
        if let Ok(year) = eyr.parse::<u16>() {
            return (2020..=2030).contains(&year);
        }
        false
    });
    // hgt (Height) - a number followed by either cm or in.
    //   If cm, the number must be at least 150 and at most 193.
    //   If in, the number must be at least 59 and at most 76.
    validators.insert(String::from("hgt"), |hgt: &String| {
        // TODO: see if we can move the regex outside the function
        // maybe check lazy_static!
        let hgt_regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        if let Some(captures) = hgt_regex.captures(hgt) {
            let unit = captures.get(2).unwrap().as_str();

            if let Ok(num) = captures.get(1).unwrap().as_str().parse::<u16>() {
                return (unit == "in" && (59..=76).contains(&num))
                    || (unit == "cm" && (150..=193).contains(&num));
            }
        }
        false
    });
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    validators.insert(String::from("hcl"), |hcl: &String| {
        let color_regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
        color_regex.is_match(hcl)
    });
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    validators.insert(String::from("ecl"), |ecl: &String| {
        let color_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        color_regex.is_match(ecl)
    });
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    validators.insert(String::from("pid"), |pid: &String| {
        let pid_regex = Regex::new(r"^\d{9}$").unwrap();
        pid_regex.is_match(pid)
    });
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
            if let Some(field_value) = passport.get(field) {
                if validator(field_value) {
                    return true;
                }
            }
            false
        });

        // // ALTERNATIVE more verbose
        // let mut is_valid = true;
        // for (field, validator) in &validators {
        //     if let Some(field_value) = passport.get(field) {
        //         if !validator(field_value) {
        //             is_valid = false;
        //             break;
        //         }
        //     } else {
        //         is_valid = false;
        //         break;
        //     }
        // }

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
