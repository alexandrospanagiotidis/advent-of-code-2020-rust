use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::ops::RangeInclusive;

type ValueRangeType = u16;
type ValueRange = RangeInclusive<ValueRangeType>;
type ValueRanges = Vec<ValueRange>;
type Ticket = Vec<ValueRangeType>;

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let (ranges, _my_ticket, tickets) = parse_input(&lines);

    {
        let part1_result = part1(&ranges, &tickets);
        let ticket_scanning_error_rate: u16 = part1_result.iter().sum();
        println!("part1: ticket_scanning_error_rate={0}", ticket_scanning_error_rate);
    }
}

fn parse_input(lines: &Vec<String>) -> (HashMap<&str, ValueRanges>, Ticket, Vec<Ticket>) {
    let mut lines = lines.iter();

    let mut ranges: HashMap<&str, ValueRanges> = HashMap::new();

    'read_ranges: while let Some(line) = lines.next() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("your ticket:") {
            break 'read_ranges;
        }

        //departure location: 37-479 or 485-954
        let mut tokens = line.split(": ");

        let range_name = tokens.next()
            .expect("Could not get range_name");

        let tokens = tokens.next().unwrap();
        let mut tokens = tokens.split(" or ");

        let mut range1_str = tokens.next().unwrap().split("-");
        let mut range2_str = tokens.next().unwrap().split("-");

        let range1 = ValueRange::new(
            range1_str.next().unwrap().parse::<ValueRangeType>().unwrap(),
            range1_str.next().unwrap().parse::<ValueRangeType>().unwrap(),
        );
        let range2 = ValueRange::new(
            range2_str.next().unwrap().parse::<ValueRangeType>().unwrap(),
            range2_str.next().unwrap().parse::<ValueRangeType>()
                .expect(format!("Could not read end of range 2: {0}", line).as_str()),
        );

        ranges.insert(range_name, vec![range1, range2]);
    }

    // println!("ranges={0:#?}", ranges);

    let my_ticket: Ticket = parse_ticket(lines.next().unwrap());

    // println!("my_ticket={0:#?}", my_ticket);

    lines.next();
    lines.next();

    let mut tickets: Vec<Ticket> = Vec::with_capacity(lines.len());

    while let Some(line) = lines.next() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let ticket = parse_ticket(line);

        tickets.push(ticket);
    }

    // println!("tickets={0:#?}", tickets);

    (
        ranges,
        my_ticket,
        tickets,
    )
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(',')
        .map(|value| value.parse::<ValueRangeType>().unwrap())
        .collect()
}

fn part1(ranges: &HashMap<&str, ValueRanges>, tickets: &Vec<Ticket>) -> Vec<ValueRangeType> {
    let mut invalid_fields: Vec<ValueRangeType> = Vec::with_capacity(tickets.len());

    let mut current_fields: HashMap<ValueRangeType, usize> = HashMap::with_capacity(20);

    for ticket in tickets {
        current_fields.clear();

        for number in ticket {
            let entry = current_fields.entry(*number).or_insert(0);

            for (&_field, field_ranges) in ranges {
                for range in field_ranges {
                    if range.contains(&number) {
                        *entry += 1;
                    }
                }
            }
        }

        // println!("current_fields={0:#?}", current_fields);

        let mut invalid_field: Vec<ValueRangeType> = current_fields.iter()
            .filter(|&(_value, count)| *count == 0)
            .map(|(value, _count)| *value)
            .collect();

        invalid_fields.append(&mut invalid_field);
    }

    invalid_fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = r"
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
        ".split('\n')
            .map(|as_str| String::from(as_str))
            .collect();

        let (ranges, my_ticket, tickets) = parse_input(&input);

        let result = part1(&ranges, &tickets);

        // println!("part1_example1={0:#?}", result);

        assert_eq!(result, vec![4, 55, 12]);
    }
}