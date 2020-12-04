use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::iter::FromIterator;
use std::ops::Add;

fn main() {
    let mut inputs: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    // Add empty line so the last entry is always processed (as we expect empty lines to trigger the validation)
    inputs.push(String::from(""));

    let inputs = inputs;

    // println!("{0:?}", inputs);

    println!("part1: number of valid passports = {0}", part1(&inputs));
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
