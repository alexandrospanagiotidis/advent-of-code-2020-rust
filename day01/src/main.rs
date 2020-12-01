use std::io::{BufRead, stdin};
use std::collections::HashSet;
use std::ops::Add;

fn main() {
    let numbers: HashSet<i32> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Could not read line"))
        .map(|line| line.parse::<i32>().expect(String::from("Could not convert to number: ").add(&line).as_str()))
        .collect();

    for number in numbers.iter() {
        let remainder = 2020 - number;

        if numbers.contains(&remainder) {
            println!("{0}*{1}={2}", number, remainder, number*remainder);
            return ()
        }
    }
}
