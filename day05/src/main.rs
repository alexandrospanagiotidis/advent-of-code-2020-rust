use std::io::{BufRead, stdin};

fn main() {
    let seat_codes: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    part1(&seat_codes);
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
