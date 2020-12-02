use std::io::{BufRead, stdin};
use std::ops::Add;

#[derive(Debug)]
struct PasswordPolicy {
    min_occurrences: usize,
    max_occurrences: usize,
    character: char,
}

impl PasswordPolicy {
    fn is_valid_sled_rental(&self, password: &String) -> bool {
        let occurrences = password.matches(self.character).count();

        occurrences >= self.min_occurrences && occurrences <= self.max_occurrences
    }

    fn is_valid_toboggan(&self, password: &String) -> bool {
        let chars = password.chars();

        // println!("password={0} policy={1:?}", password, self);

        // Indices are 1-based at Toboggan
        let first_occurrence_matches = chars.clone().nth(self.min_occurrences - 1).unwrap() == self.character;
        let second_occurrence_matches = chars.clone().nth(self.max_occurrences - 1).unwrap() == self.character;

        // Only one occurrence allowed
        first_occurrence_matches != second_occurrence_matches
    }
}

fn main() {
    let test_cases: Vec<(PasswordPolicy, String)> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .map(|line| parse_line(line))
        .collect();

    // println!("input = {0:?}", test_cases);

    day02_part1(&test_cases);
    day02_part2(&test_cases);
}

fn day02_part1(test_cases: &Vec<(PasswordPolicy, String)>) {
    let valid_password_count = test_cases.iter()
        .map(|(password_policy, password)| password_policy.is_valid_sled_rental(password))
        .filter(|valid| *valid == true)
        .count();

    println!("part1: number of valid passwords = {0}", valid_password_count);
}

fn day02_part2(test_cases: &Vec<(PasswordPolicy, String)>) {
    let valid_password_count = test_cases.iter()
        .map(|(password_policy, password)| password_policy.is_valid_toboggan(password))
        .filter(|valid| *valid == true)
        .count();

    println!("part2: number of valid passwords = {0}", valid_password_count);
}

fn parse_line(line: String) -> (PasswordPolicy, String) {
    let mut tokens = line.split_whitespace();

    let (min_occurrences, max_occurrences) = tokens.next()
        .map(|range| range.split("-").take(2).collect::<Vec<&str>>())
        .map(|parts| (
            parts[0].parse::<usize>().expect("Could not convert"),
            parts[1].parse::<usize>().expect("Could not convert")
        ))
        .expect(String::from("Failed to parse range from line").add(&line).as_str());

    let character: char = tokens.next()
        .map(|probe_string| probe_string.chars().nth(0).unwrap())
        .expect(String::from("Failed to parse character from line").add(&line).as_str());

    let password = tokens.next()
        .map(String::from)
        .expect(String::from("Failed to parse prove from line").add(&line).as_str());

    return (PasswordPolicy {
        min_occurrences,
        max_occurrences,
        character,
    }, password)
}
