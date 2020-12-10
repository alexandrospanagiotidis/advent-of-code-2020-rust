use std::collections::HashMap;
use std::io::{BufRead, stdin};

type Joltage = u32;

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    // println!("input={0:?}", input);

    let adapters: Vec<Joltage> = input
        .iter()
        .map(|adapter_string| adapter_string.parse::<Joltage>()
            .expect(format!("Could not parse as number: {0}", adapter_string).as_str())
        )
        .collect();

    // println!("adapters={0:?}", adapters);

    let part1_result = part1(&adapters);

    if !part1_result.is_empty() {
        // println!("part1_result={0:?}", part1_result);

        let ones = part1_result.iter()
            .filter(|&difference| *difference == 1)
            .count();

        let threes = part1_result.iter()
            .filter(|&difference| *difference == 3)
            .count();

        println!("part1: ones={0} threes={1} product={2}", ones, threes, ones * threes);

        let number_of_combinations = part2(&part1_result);

        println!("part2: number_of_combinations={0}", number_of_combinations);
    }
}

fn part1(adapters: &Vec<u32>) -> Vec<u32> {
    let mut adapters = adapters.clone();
    adapters.sort();

    // println!("sorted adapters={0:?}", adapters);

    let mut differences: Vec<u32> = Vec::new();

    // The charging outlet has an effective rating of 0 jolts
    let mut last_adapter = 0;

    for current_adapter in adapters {
        // println!("last={0}  current={1}  diff={2}",
        //          last_adapter, current_adapter, current_adapter - last_adapter
        // );

        differences.push(current_adapter - last_adapter);
        last_adapter = current_adapter;
    }

    // your device's built-in adapter is always 3 higher than the highest adapter
    differences.push(3);

    differences
}

fn part2(differences: &Vec<u32>) -> u64 {
    // println!("#differences={0} -> {1:?}", differences.len(), differences);

    let mut combinations: u64 = 1;

    let mut left = 0;
    let mut cache: HashMap<u64, u64> = HashMap::new();

    while left < differences.len() {
        let mut right = left;

        while right < differences.len() && differences.get(right).unwrap() == &1 {
            right += 1;
        }

        let length_of_one_sequence = right - left;
        combinations *= tribonacci(length_of_one_sequence as u64, &mut cache);

        while right < differences.len() && differences.get(right).unwrap() == &3 {
            right += 1;
        }

        left = right;
    }

    combinations
}

// Truth be told, I got this from Reddit
fn tribonacci(number: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    // println!("number={0} cache={1:?}", number, cache);

    match cache.get(&number).map(|entry| entry.clone()) {
        Some(result) => result,
        None => {
            let result = match number {
                0 => 1,
                1 => 1,
                2 => 2,
                n => tribonacci(n - 1, cache)
                    + tribonacci(n - 2, cache)
                    + tribonacci(n - 3, cache),
            };

            cache.insert(number, result);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let adapters: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let differences = part1(&adapters);

        assert_eq!(differences, vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3]);
    }

    #[test]
    fn part2_example1() {
        let adapters: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let differences = part1(&adapters);

        let can_skip = part2(&differences);

        assert_eq!(can_skip, 3);
    }

    #[test]
    fn part2_example2() {
        let adapters: Vec<u32> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        let differences = part1(&adapters);

        let can_skip = part2(&differences);

        assert_eq!(can_skip, 19208);
    }
}
