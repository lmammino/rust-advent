use regex::Regex;
use std::ops::RangeInclusive;

pub trait Validator {
    fn validate(&self, value: &str) -> bool;
}

pub struct U16RangeValidator {
    range: RangeInclusive<u16>,
}

impl U16RangeValidator {
    pub fn new(range: RangeInclusive<u16>) -> Self {
        U16RangeValidator { range }
    }
}

impl Validator for U16RangeValidator {
    fn validate(&self, value: &str) -> bool {
        if let Ok(year) = value.parse::<u16>() {
            return self.range.contains(&year);
        }
        false
    }
}

pub struct RegexValidator {
    regex: Regex,
}

impl RegexValidator {
    pub fn new(regex: Regex) -> Self {
        RegexValidator { regex }
    }
}

impl Validator for RegexValidator {
    fn validate(&self, value: &str) -> bool {
        self.regex.is_match(value)
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
pub fn create_byr_validator() -> U16RangeValidator {
    U16RangeValidator::new(1920..=2002)
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
pub fn create_iyr_validator() -> U16RangeValidator {
    U16RangeValidator::new(2010..=2020)
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
pub fn create_eyr_validator() -> U16RangeValidator {
    U16RangeValidator::new(2020..=2030)
}

// hgt (Height) - a number followed by either cm or in.
//   If cm, the number must be at least 150 and at most 193.
//   If in, the number must be at least 59 and at most 76.
pub struct HgtValidator {
    regex: Regex,
}

impl HgtValidator {
    pub fn new() -> Self {
        let regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        HgtValidator { regex }
    }
}

impl Validator for HgtValidator {
    fn validate(&self, value: &str) -> bool {
        if let Some(captures) = self.regex.captures(value) {
            let unit = captures.get(2).unwrap().as_str();

            if let Ok(num) = captures.get(1).unwrap().as_str().parse::<u16>() {
                return (unit == "in" && (59..=76).contains(&num))
                    || (unit == "cm" && (150..=193).contains(&num));
            }
        }
        false
    }
}

pub fn create_hgt_validator() -> HgtValidator {
    HgtValidator::new()
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
pub fn create_hcl_validator() -> RegexValidator {
    let regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    RegexValidator::new(regex)
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pub fn create_ecl_validator() -> RegexValidator {
    let regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    RegexValidator::new(regex)
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
pub fn create_pid_validator() -> RegexValidator {
    let regex = Regex::new(r"^\d{9}$").unwrap();
    RegexValidator::new(regex)
}
