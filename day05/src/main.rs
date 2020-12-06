use std::io::{BufRead, stdin};
use std::collections::HashSet;

fn main() {
    let seat_codes: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    part1(&seat_codes);
    part2(&seat_codes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determine_seat_id_returns_357_for_FBFBBFFRLR() {
        let seat_code = String::from("FBFBBFFRLR");
        assert_eq!(determine_seat_id(&seat_code), 357);
    }

    #[test]
    fn determine_seat_id_returns_567_for_BFFFBBFRRR() {
        let seat_code = String::from("BFFFBBFRRR");
        assert_eq!(determine_seat_id(&seat_code), 567);
    }

    #[test]
    fn determine_seat_id_returns_567_for_FFFBBBFRRR() {
        let seat_code = String::from("FFFBBBFRRR");
        assert_eq!(determine_seat_id(&seat_code), 119);
    }

    #[test]
    fn determine_seat_id_returns_820_for_BBFFBBFRLL() {
        let seat_code = String::from("BBFFBBFRLL");
        assert_eq!(determine_seat_id(&seat_code), 820);
    }
}

fn determine_seat_id(seat_code: &String) -> u32 {
    let row_partitions = &seat_code[..7];
    let column_partitions = &seat_code[7..];

    let mut lower_bound = 0;
    let mut upper_bound = 127;
    let mut row = 0;

    for row_partition in row_partitions.chars() {
        // println!("before: l={0} u={1} p={2}", lower_bound, upper_bound, row_partition);
        match row_partition {
            'F' => {
                upper_bound = (upper_bound + lower_bound) / 2;
                row = lower_bound;
            }
            'B' => {
                // Need to add 1 as the row indices are 0-based
                lower_bound = (upper_bound + lower_bound + 1) / 2;
                row = upper_bound;
            }
            _ => panic!("Invalid row partition '{0}'", row_partition)
        }
        // println!("after: l={0} u={1} p={2}", lower_bound, upper_bound, row_partition);
    }

    // println!("row={0}", row);

    let mut lower_bound = 0;
    let mut upper_bound = 7;
    let mut column = 0;

    for column_partition in column_partitions.chars() {
        // println!("before: l={0} u={1} p={2}", lower_bound, upper_bound, column_partition);
        match column_partition {
            'L' => {
                upper_bound = (upper_bound + lower_bound) / 2;
                column = lower_bound;
            }
            'R' => {
                // Need to add 1 as the column indices are 0-based
                lower_bound = (upper_bound + lower_bound + 1) / 2;
                column = upper_bound;
            }
            _ => panic!("Invalid column partition '{0}'", column_partition)
        }
        // println!("after: l={0} u={1} p={2}", lower_bound, upper_bound, column_partition);
    }

    // println!("column={0}", column);

    row * 8 + column
}

fn part1(seat_codes: &Vec<String>) -> u32 {
    let highest_seat_id = seat_codes.into_iter()
        .map(|seat_code| determine_seat_id(&seat_code))
        .max()
        .expect("Could not determine max seat id");

    println!("part1: highest seat id = {0}", highest_seat_id);

    highest_seat_id
}

fn part2(seat_codes: &Vec<String>) {
    let mut possible_seats: HashSet<u32> = HashSet::new();
    for row in 0..127 {
        for column in 0..7 {
            possible_seats.insert(row * 8 + column);
        }
    }

    let seat_codes = seat_codes.into_iter()
        .map(|seat_code| determine_seat_id(&seat_code))
        .for_each(|seat_id| {
            possible_seats.remove(&seat_id);
        });

    let mut possible_seats = possible_seats.into_iter()
        .collect::<Vec<u32>>();

    possible_seats.sort();

    println!("part2: your seat id = {0:?}", possible_seats);

    // Didn't actually solve this part; looked at the output:
    // part2: your seat id = [0, 1, 2, 3, 4, 5, 6, 619, 909, 910, 912, 913, 914, 915, 916, 917, 918, 920, 921, 922, 923, 924, 925, 926, 928, 929, 930, 931, 932, 933, 934, 936, 937, 938, 939, 940, 941, 942, 944, 945, 946, 947, 948, 949, 950, 952, 953, 954, 955, 956, 957, 958, 960, 961, 962, 963, 964, 965, 966, 968, 969, 970, 971, 972, 973, 974, 976, 977, 978, 979, 980, 981, 982, 984, 985, 986, 987, 988, 989, 990, 992, 993, 994, 995, 996, 997, 998, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1008, 1009, 1010, 1011, 1012, 1013, 1014]
    // and guessed "619" :)
}
