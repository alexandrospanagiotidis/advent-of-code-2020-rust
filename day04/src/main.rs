use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::iter::FromIterator;
use std::ops::Add;

use regex::Regex;

fn main() {
    let mut inputs: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    // Add empty line so the last entry is always processed (as we expect empty lines to trigger the validation)
    inputs.push(String::from(""));

    let inputs = inputs;

    // println!("{0:?}", inputs);

    println!("part1: number of valid passports = {0}", part1(&inputs));
    println!("part2: number of valid passports = {0}", part2(&inputs));
}

fn part1(lines: &Vec<String>) -> i32 {
    // "cid" is optional
    let expected_fields: HashSet<&'static str> = HashSet::from_iter(
        vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid").iter().cloned()
    );

    let mut valid_passports = 0;
    let mut missing_fields = expected_fields.clone();
    let mut full_line = String::new();

    for line in lines {
        let tokens = line.trim();

        full_line = full_line.add(" ").add(&tokens);

        // println!("tokens = {0:?}, empty={1}", tokens, tokens.is_empty());

        if tokens.is_empty() {
            // println!("found newline, input was={0:?}", full_line);

            if missing_fields.is_empty() {
                valid_passports = valid_passports + 1;
            } else {
                // println!("missing_fields={0:?}", missing_fields);
            }

            // println!("valid_passports = {0}", valid_passports);
            missing_fields = expected_fields.clone();
            full_line.clear();
        } else {
            tokens.split_whitespace().into_iter()
                // .map(|tokens| {
                //     println!("tokens={0:?}", tokens);
                //     tokens
                // })
                .map(|token| token.split(":").nth(0).unwrap())
                // .map(|token| {
                //     println!("token={0:?}", token);
                //     token
                // })
                .for_each(|field| {
                    missing_fields.remove(field);
                    ()
                });
        }
    }

    valid_passports
}

fn part2(lines: &Vec<String>) -> i32 {
    let expected_fields: HashSet<&'static str> = HashSet::from_iter(
        vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid").iter().cloned()
    );

    let mut valid_passports = 0;
    let mut missing_fields = expected_fields.clone();
    let mut full_line = String::new();

    for line in lines {
        let tokens = line.trim();

        full_line = full_line.add(" ").add(&tokens);

        if tokens.is_empty() {
            if missing_fields.is_empty() {
                valid_passports = valid_passports + 1;
            }

            missing_fields = expected_fields.clone();
            full_line.clear();
        } else {
            tokens.split_whitespace().into_iter()
                .map(|token| token.split(":").take(2).collect::<Vec<&str>>())
                .for_each(|field| {
                    let (name, value) = (field[0], field[1]);
                    let valid = is_valid(name, value);
                    if valid {
                        missing_fields.remove(name);
                    } else {
                        // println!("line={0:?} name={1} value={2} valid={3}", full_line, name, value, valid);
                    }
                    ()
                });
        }
    }

    valid_passports
}

fn is_valid(name: &str, value: &str) -> bool {
    match name {
        "byr" => is_valid_year(value, 1920, 2002),
        "iyr" => is_valid_year(value, 2010, 2020),
        "eyr" => is_valid_year(value, 2020, 2030),
        "hgt" => is_valid_height(value),
        "hcl" => is_valid_hair_color(value),
        "ecl" => is_valid_eye_color(value),
        "pid" => is_valid_passport_id(value),
        "cid" => true,
        _ => false
    }
}

fn is_valid_passport_id(passport_id: &str) -> bool {
    let re = Regex::new("^[0-9]{9}$").unwrap();
    re.is_match(passport_id)
}

fn as_i32(year: &str) -> i32 {
    year.parse::<i32>()
        // If the value cannot be parsed, the assume it is invalid
        .unwrap_or(-1)
}

fn is_valid_year(year: &str, lower_bound: i32, upper_bound: i32) -> bool {
    let year = as_i32(year);
    year >= lower_bound && year <= upper_bound
}

fn is_valid_height(height: &str) -> bool {
    let length = height.len();
    let value = &height[..length - 2];
    let unit = &height[length - 2..];
    let height = as_i32(value);

    match unit {
        "cm" => height >= 150 && height <= 193,
        "in" => height >= 59 && height <= 76,
        _ => false,
    }
}

fn is_valid_hair_color(hair_color: &str) -> bool {
    let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
    re.is_match(hair_color)
}

fn is_valid_eye_color(eye_color: &str) -> bool {
    let valid_eye_colors = vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth");
    valid_eye_colors.contains(&eye_color)
}

