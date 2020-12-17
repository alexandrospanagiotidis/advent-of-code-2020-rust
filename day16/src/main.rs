use std::collections::{HashMap};
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

    let (ranges, my_ticket, tickets) = parse_input(&lines);

    let part1_result = part1(&ranges, &tickets);
    let ticket_scanning_error_rate: u16 = part1_result.values().cloned().sum();
    println!("part1: ticket_scanning_error_rate={0}", ticket_scanning_error_rate);
    assert_eq!(ticket_scanning_error_rate, 20013);

    // println!("#tickets={0}", tickets.len());
    let mut valid_tickets: Vec<Ticket> = tickets.into_iter()
        .enumerate()
        .filter(|&(index, _)| !part1_result.contains_key(&index))
        .map(|(_, ticket)| ticket)
        .collect();

    // println!("#valid_tickets={0}", valid_tickets.len());
    // println!("valid_tickets={0:#?}", valid_tickets);

    valid_tickets.push(my_ticket.clone());
    let field_order = part2(&ranges, &valid_tickets);

    let departure_product = field_order.iter()
        .filter(|&(field, _index)| field.starts_with("departure "))
        .map(|(_field, index)| *index)
        .fold(1u64, |product, index| product * my_ticket[index].clone() as u64);

    println!("part2: departure_product={0}", departure_product);
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

    println!("ranges={0:#?}", ranges);

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

fn part1(ranges: &HashMap<&str, ValueRanges>, tickets: &Vec<Ticket>) -> HashMap<usize, ValueRangeType> {
    let mut invalid_fields: HashMap<usize, ValueRangeType> = HashMap::with_capacity(tickets.len());

    let mut current_fields: HashMap<ValueRangeType, usize> = HashMap::with_capacity(20);

    for (index, ticket) in tickets.iter().enumerate() {
        current_fields.clear();

        for number in ticket {
            let entry = current_fields.entry(*number).or_insert(0);

            for (&_field, field_ranges) in ranges {
                if valid_field(&number, field_ranges) {
                    *entry += 1;
                }
            }
        }

        // println!("current_fields={0:#?}", current_fields);

        let invalid_entries: Vec<ValueRangeType> = current_fields.iter()
            .filter(|&(_value, count)| *count == 0)
            .map(|(value, _count)| *value)
            .collect();

        if !invalid_entries.is_empty() {
            // Let's hope it's always just one invalid entry per ticket
            invalid_fields.insert(index, *invalid_entries.get(0).unwrap());
        }
    }

    invalid_fields
}

fn valid_field(value: &ValueRangeType, field: &ValueRanges) -> bool {
    field.iter().any(|range| range.contains(value))
}

fn part2(ranges: &HashMap<&str, ValueRanges>, tickets: &Vec<Ticket>) -> HashMap<String, usize> {
    let number_of_fields = tickets.first().unwrap().len();

    let mut possible_fields_for_index: HashMap<&str, Vec<bool>> = HashMap::new();

    for ticket in tickets {
        for (index, number) in ticket.iter().enumerate() {
            for (field, range) in ranges {
                let entry = possible_fields_for_index.entry(field)
                    .or_insert(vec![true; number_of_fields]);

                if !valid_field(&number, &range) {
                    entry[index] = false;
                }
            }
        }
    }

    // println!("possible_fields_for_index={0:#?}", possible_fields_for_index);

    let mut order: HashMap<String, usize> = HashMap::new();
    let mut valid_index = Vec::with_capacity(number_of_fields);

    while order.len() != number_of_fields {
        for (field, indices) in &possible_fields_for_index {
            // println!("trying field={0} indices={1:?} valid_index={2:?}", field, indices, valid_index);

            let mut possible_indices = Vec::with_capacity(number_of_fields);

            for (index, &can_be_used) in indices.iter().enumerate() {
                if !valid_index.contains(&index) && can_be_used {
                    possible_indices.push(index);
                }
            }

            if possible_indices.len() == 1 {
                let index = possible_indices[0];
                order.insert(field.to_string(), index);
                valid_index.push(index);
            }
        }
    }

    // println!("order={0:?}", order);

    order
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

        let result: Vec<ValueRangeType> = result.values().cloned().collect();

        assert_eq!(result.len(), 3);
        assert!(result.contains(&4));
        assert!(result.contains(&55));
        assert!(result.contains(&12));
    }
}