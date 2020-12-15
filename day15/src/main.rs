use std::borrow::Borrow;
use std::collections::HashMap;

fn main() {
    let starting_numbers: Vec<usize> = vec![2, 1, 10, 11, 0, 6];

    {
        let result = rambunctious_recitation(&starting_numbers, 2020);
        println!("part1: result={0}", result);
    }
    {
        let result = rambunctious_recitation(&starting_numbers, 30000000);
        println!("part2: result={0}", result);
    }
}

fn rambunctious_recitation(starting_numbers: &Vec<usize>, max_rounds: usize) -> usize {
    let mut round = 0;
    let mut last_number = 0;
    let mut last_spoken: HashMap<usize, Vec<usize>> = HashMap::new();

    for &number in starting_numbers {
        round += 1;
        last_spoken.entry(number).or_insert(Vec::new()).push(round);
        last_number = number;
    }

    while round < max_rounds {
        round += 1;

        // println!("round={0}", round);
        // println!("last_number={0}", last_number);
        // println!("last_spoken={0:#?}", last_spoken);

        let current: &mut Vec<usize> = last_spoken.entry(last_number).or_insert(Vec::new());

        last_number = if current.len() <= 1 {
            0
        } else {
            let values: &[usize] = &current[current.len() - 2 ..];

            values[1] - values[0]
        };

        // println!("last_number={0:#?}", what_is_spoken);
        last_spoken.entry(last_number).or_insert(Vec::new()).push(round);
        // println!("---");
    }

    // println!("round={0} last_number={1}", round, last_number);
    // println!("last_spoken={0:#?}", last_spoken);

    last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = vec![0,3,6];

        let result = rambunctious_recitation(&input, 2020);

        assert_eq!(result, 436);
    }
}
