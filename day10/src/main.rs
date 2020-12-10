use std::collections::HashMap;
use std::io::{BufRead, stdin};

type Joltage = u32;

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    println!("input={0:?}", input);

    let mut adapters: Vec<Joltage> = input
        .iter()
        .map(|adapter_string| adapter_string.parse::<Joltage>()
            .expect(format!("Could not parse as number: {0}", adapter_string).as_str())
        )
        .collect();

    println!("adapters={0:?}", adapters);

    let differences = part1(&adapters);

    println!("differences={0:?}", differences);

    println!("part1: product={0}", differences.get(&1).unwrap() * differences.get(&3).unwrap());
}

fn part1(adapters: &Vec<u32>) -> HashMap<u32, u32> {
    let mut adapters = adapters.clone();
    adapters.sort();

    let mut differences: HashMap<u32, u32> = HashMap::new();

    // The charging outlet has an effective rating of 0 jolts
    let mut last_adapter = 0;

    for current_adapter in adapters {
        println!("last={0}  current={1}  diff={2}",
            last_adapter, current_adapter, current_adapter-last_adapter
        );

        *differences.entry(current_adapter - last_adapter).or_insert(0) += 1;

        last_adapter = current_adapter;
    }

    // your device's built-in adapter is always 3 higher than the highest adapter
    *differences.entry(3).or_insert(0) += 1;

    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let adapters: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let differences = part1(&adapters);

        assert_eq!(differences.get(&1), Some(&7u32));
        assert_eq!(differences.get(&3), Some(&5u32));
    }
}
