use std::io::{BufRead, stdin};
use std::ops::Add;

#[derive(Debug)]
struct PasswordPolicy {
    min_occurrences: usize,
    max_occurrences: usize,
    character: String,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &String) -> bool {
        let occurrences = password.matches(&self.character).count();

        occurrences >= self.min_occurrences && occurrences <= self.max_occurrences
    }
}

fn main() {
    let test_cases: Vec<(PasswordPolicy, String)> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .map(|line| parse_line(line))
        .collect();

    // println!("input = {0:?}", test_cases);

    let valid_password_count = test_cases.iter()
        .map(|(password_policy, password)| password_policy.is_valid(password))
        .filter(|valid| *valid == true)
        .count();

    println!("number of valid passwords = {0}", valid_password_count);
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

    let character = tokens.next()
        .map(|character| character.split(":")
            .take(1)
            .nth(0)
            .unwrap()
        )
        .map(String::from)
        .expect(String::from("Failed to parse character from line").add(&line).as_str());

    let probe = tokens.next()
        .map(String::from)
        .expect(String::from("Failed to parse prove from line").add(&line).as_str());

    return (PasswordPolicy {
        min_occurrences,
        max_occurrences,
        character,
    }, probe)
}
