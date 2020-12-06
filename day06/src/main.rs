use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() {
    let mut groups_answers: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    // Add empty line so the last entry is always processed (as we expect empty lines to trigger the validation)
    groups_answers.push(String::from(""));

    let groups_answers = groups_answers;

    println!("groups_answers = {0:?}", groups_answers);

    part1(&groups_answers);
}

fn part1(answers: &Vec<String>) -> usize {
    let mut questions_answered_yes = 0;

    let mut group_answers = HashSet::new();

    for answer in answers {
        let answer = answer.trim();

        if answer.is_empty() {
            questions_answered_yes = questions_answered_yes + group_answers.len();
            group_answers.clear();
        } else {
            for answer in answer.chars() {
                group_answers.insert(answer);
            }
        }
    }

    println!("part1: questions_answered_yes = {0}", questions_answered_yes);

    questions_answered_yes
}
