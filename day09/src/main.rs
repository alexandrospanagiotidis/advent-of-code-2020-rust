use std::io::{BufRead, stdin};
use std::ops::{Add, Range};

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

    if let Some(part1_result) = part1(&numbers) {
        let magic_number = part1_result.0;

        println!("Did not find a number pair that sums to {0}", magic_number);

        let windows = part2(&numbers, magic_number);

        for window in windows {
            let slice = &numbers[window.start..window.end];
            let min_value = slice.iter().min().unwrap();
            let max_value = slice.iter().max().unwrap();

            println!("part2: {0:?} sums to {1}; sum(min={2}, max={3})={4}",
                     slice, magic_number, min_value, max_value, min_value + max_value
            );
        }
    }
}

fn part1(numbers: &[NumberType]) -> Option<(NumberType, usize, usize)> {
    process_all_numbers(&numbers, 25, &sum_any_two)
}

fn part2(numbers: &[NumberType], magic_number: NumberType) -> Vec<Range<usize>> {
    let mut windows: Vec<Range<usize>> = Vec::new();

    let length = numbers.len();

    'search: for window_size in (2..=length / 2).rev() {
        println!("window_size={0}", window_size);
        for left in 0..length - window_size {
            let right = left + window_size;

            if right >= length {
                continue 'search;
            }

            let slice = &numbers[left..right];
            let sum: NumberType = slice.iter().sum();

            // println!("sum={0} numbers[{1}..{2}]={3:?}", sum, left, right, slice);

            if sum == magic_number {
                windows.push(left..right);
                // break 'search;
            }
        }
    }

    windows
}

fn process_all_numbers(numbers: &[u64], window_size: usize, condition: &dyn Fn(&[NumberType], NumberType) -> Vec<NumberType>) -> Option<(NumberType, usize, usize)> {
    let mut left = 0;
    let mut right = window_size;

    while right <= numbers.len() {
        let next_index = right + 1;

        if next_index >= numbers.len() {
            panic!("No more numbers at index {0}", next_index);
        }

        let candidates = &numbers[left..=right];
        let next_number = numbers[next_index];
        let operands = condition(candidates, next_number);

        if operands.is_empty() {
            // println!("numbers[{0}..{1}]={2:?}", left, right, candidates);

            return Some((next_number, left, right));
        }

        left += 1;
        right = left + window_size;
    }

    None
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
