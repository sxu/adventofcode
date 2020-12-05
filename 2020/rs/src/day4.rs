use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Data {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Data {
    fn new() -> Data {
        Data {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        let byr = self.birth_year.as_ref().unwrap().parse::<u32>().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr = self.issue_year.as_ref().unwrap().parse::<u32>().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr = self
            .expiration_year
            .as_ref()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt = self.height.as_ref().unwrap();
        let cm = hgt.ends_with("cm");
        let inch = hgt.ends_with("in");
        if !cm && !inch {
            return false;
        }
        let hgt_val = hgt[..(hgt.len() - 2)].parse::<u32>().unwrap();
        if cm && (hgt_val < 150 || hgt_val > 193) {
            return false;
        }
        if inch && (hgt_val < 59 || hgt_val > 76) {
            return false;
        }

        let hcl = self.hair_color.as_ref().unwrap();
        if hcl.len() != 7 {
            return false;
        }
        let mut iter = hcl.chars();
        if iter.next() != Some('#') {
            return false;
        }
        while let Some(c) = iter.next() {
            if !c.is_digit(10) && (c < 'a' || c > 'f') {
                return false;
            }
        }

        let ecl = self.eye_color.as_ref().unwrap();
        match ecl.as_str() {
            "amb" => (),
            "blu" => (),
            "brn" => (),
            "gry" => (),
            "grn" => (),
            "hzl" => (),
            "oth" => (),
            _ => return false,
        }

        let pid = self.passport_id.as_ref().unwrap();
        if pid.len() != 9 || !pid.chars().all(|c| c.is_digit(10)) {
            return false;
        }

        true
    }
}

fn parse_data(line: &str, data: &mut Data) {
    for entry in line.split(' ') {
        let (field, value) = entry.split(':').next_tuple().unwrap();
        match field {
            "byr" => data.birth_year = Some(value.to_string()),
            "iyr" => data.issue_year = Some(value.to_string()),
            "eyr" => data.expiration_year = Some(value.to_string()),
            "hgt" => data.height = Some(value.to_string()),
            "hcl" => data.hair_color = Some(value.to_string()),
            "ecl" => data.eye_color = Some(value.to_string()),
            "pid" => data.passport_id = Some(value.to_string()),
            "cid" => data.country_id = Some(value.to_string()),
            _ => panic!("Unknown field {}", field),
        }
    }
}

pub fn day4(input_path: &str) {
    let file =
        File::open(input_path).unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e));
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut data = Data::new();
    let mut num_has_required_fields = 0;
    let mut num_valid = 0;
    for line in lines {
        if line.is_empty() {
            if data.has_required_fields() {
                num_has_required_fields += 1;
                if data.is_valid() {
                    num_valid += 1;
                }
            }
            data = Data::new();
        } else {
            parse_data(&line, &mut data);
        }
    }
    if data.has_required_fields() {
        num_has_required_fields += 1;
        if data.is_valid() {
            num_valid += 1;
        }
    }
    assert_eq!(num_has_required_fields, 245);
    assert_eq!(num_valid, 133);
}
