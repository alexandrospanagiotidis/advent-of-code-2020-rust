use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::ops::Add;

fn main() {
    let numbers: Vec<i32> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Could not read line"))
        .map(|line| line.parse::<i32>().expect(String::from("Could not convert to number: ").add(&line).as_str()))
        .collect();

    match elements_with_sum(2020, numbers, &part2) {
        Some(result) => println!("product({0:?}) = {1} ", result.clone(), elements_product(result)),
        None => ()
    }
}

/// Find elements in `numbers` that sum up to `sum` when checked with `checker`.
fn elements_with_sum(sum: i32, numbers: Vec<i32>, checker: &dyn Fn(i32, Vec<i32>) -> Option<Vec<i32>>) -> Option<Vec<i32>> {
    for index in 1..numbers.len() {
        let head = numbers[index];
        let tail = numbers[index + 1..].to_vec();
        let remainder = sum - head;

        // println!("index={0}, head={1}, tail={2:?}, remainder={3}", index, head, tail, remainder);

        if let Some(elements) = checker(remainder, tail) {
            return Some(vec![head]
                .into_iter()
                .chain(elements.into_iter())
                .collect())
        };
    }

    None
}

/// Return `Some([remainder])` iff `tail.contains(remainder)`
fn part1(remainder: i32, tail: Vec<i32>) -> Option<Vec<i32>> {
    if tail.contains(&remainder) {
        // println!("tail={0:?} contains {1}", tail, remainder);

        Some(vec![remainder]
            .into_iter()
            .collect()
        )
    } else {
        None
    }
}

/// Returns `Some([sum, x])` iff `tail.contains(sum - x)`
fn part2(sum: i32, tail: Vec<i32>) -> Option<Vec<i32>> {
    elements_with_sum(sum, tail, &part1)
}

/// Calculates the product of all elements in `elements`.
/// If `elements` is empty, then `0` is returned.
fn elements_product(elements: Vec<i32>) -> i64 {
    let init = if elements.is_empty() { 0 } else { 1 };

    elements.into_iter()
        .fold(init, |product, element| product * i64::from(element))
}

