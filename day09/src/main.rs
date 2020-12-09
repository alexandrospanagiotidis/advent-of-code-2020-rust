use std::io::{BufRead, stdin};
use std::ops::Add;

type NumberType = u64;

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let numbers: Vec<NumberType> = input.into_iter()
        .map(|line| line.parse::<NumberType>()
            .expect(String::from("Could not parse as number: ").add(&line).as_str())
        )
        .collect();

    part1(&numbers);
}

fn part1(numbers: &[NumberType]) {
    process_all_numbers(&numbers, 25, &sum_any_two);
}

fn process_all_numbers(numbers: &[u64], window_size: usize, condition: &dyn Fn(&[NumberType], NumberType) -> Vec<NumberType>) {
    let mut left = 0;
    let mut right = window_size;

    loop {
        let next_index = right + 1;

        if next_index >= numbers.len() {
            panic!("No more numbers at index {0}", next_index);
        }

        // +1 because upper bound is not inclusive
        let candidates = &numbers[left..right + 1];
        let next_number = numbers[next_index];
        let operands = condition(candidates, next_number);

        if operands.is_empty() {
            println!("Did not find a match for {0}", next_number);
            println!("numbers[{0}..{1}]={2:?}", left, right, candidates);
            break;
        }

        left += 1;
        right = left + window_size;
    }
}

fn sum_any_two(numbers: &[NumberType], needle: NumberType) -> Vec<NumberType> {
    for index in 0..numbers.len() {
        let left = numbers[index];

        if left >= needle {
            continue;
        }

        let needle = needle - left;

        if numbers[index..].contains(&needle) {
            return vec![left, needle];
        }
    }

    vec![]
}
