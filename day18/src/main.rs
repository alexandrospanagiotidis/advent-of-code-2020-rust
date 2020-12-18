use std::collections::VecDeque;
use std::io::{BufRead, stdin};

#[derive(Clone, Debug, PartialEq)]
enum OperationType {
    Plus,
    Mult,
}

type NumberType = u64;

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Operand(NumberType),
    Operation(OperationType),
    OpeningParens,
    ClosingParens,
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut result: NumberType = 0;

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut tokens = parse(&line);
        print!("tokens={0:?}", tokens);
        let res = evaluate(&mut tokens);
        result += res;
        println!("  res={0:?} -> result={1}", res, result);
    }

    println!("part1: result={0}", result);
}

fn parse(line: &str) -> Vec<Operation> {
    let mut operands: Vec<Operation> = Vec::new();

    let mut chars = line.chars().peekable();
    let mut number = String::with_capacity(10);

    loop {
        let current_char = chars.next();

        if current_char.is_none() {
            break;
        }

        let current_char = current_char.unwrap();

        // println!("current_char={0:?}", current_char);

        match current_char {
            ' ' => (),
            '(' => {
                operands.push(Operation::OpeningParens);
            }
            ')' => {
                operands.push(Operation::ClosingParens);
            }
            '+' => {
                operands.push(Operation::Operation(OperationType::Plus));
            }
            '*' => {
                operands.push(Operation::Operation(OperationType::Mult));
            }
            _ => {
                number.clear();
                number.push(current_char);

                'parse_number: loop {
                    let current_number = chars.peek();

                    match current_number {
                        None => break 'parse_number,
                        Some(digit) => {
                            if digit.is_digit(10) {
                                number.push(*digit);
                                chars.next();
                            } else {
                                break 'parse_number;
                            }
                        }
                    }
                }

                let number = number.parse::<NumberType>()
                    .expect(format!("Invalid number: {0}", number).as_str());

                operands.push(Operation::Operand(number));
            }
        }
    }

    operands
}

fn evaluate(operands: &mut Vec<Operation>) -> NumberType {
    // println!("-- operands={0:?}", operands);

    'resolve_parens: loop {
        // println!("operands={0:?}", operands);

        // Find next closing parens
        let next_closing_parens = operands.iter()
            .position(|op| *op == Operation::ClosingParens);

        match next_closing_parens {
            Some(closing_parens_index) => {
                // Find first opening parens relative to this closing one
                let mut matching_opening_parens = closing_parens_index - 1;
                while operands[matching_opening_parens] != Operation::OpeningParens {
                    matching_opening_parens -= 1;
                }

                // println!("removing parens from {0:?}", operands);
                let mut inner_op: Vec<Operation> = operands.drain(matching_opening_parens..=closing_parens_index).collect();

                // Remove the parens
                inner_op.drain(..1);
                inner_op.pop();

                let result = evaluate(&mut inner_op);
                operands.insert(matching_opening_parens, Operation::Operand(result));

                // println!("after {0:?}", operands);
            }
            None => break 'resolve_parens,
        }
    }

    // println!("No more parens -> {0:?}", operands);

    let mut operands: VecDeque<Operation> = operands.into_iter()
        .map(|o| o.clone())
        .collect();

    // No more parens in operands
    while operands.len() != 1 {
        let number1 = match operands.pop_front().unwrap() {
            Operation::Operand(number) => number.to_owned(),
            _ => panic!(format!("Expected number: {0:?}", operands)),
        };

        let operation = match operands.pop_front().unwrap() {
            Operation::Operation(what) => what,
            _ => panic!("Did not find operation after number"),
        };

        let number2 = match operands.pop_front().unwrap() {
            Operation::Operand(number) => number.to_owned(),
            _ => panic!(format!("Expected number: {0:?}", operands)),
        };

        let result = Operation::Operand(match operation {
            OperationType::Plus => number1 + number2,
            OperationType::Mult => number1 * number2,
        });

        operands.insert(0, result);
    }

    match operands[0] {
        Operation::Operand(number) => number,
        _ => panic!(format!("Last element not an operand: {0:?}", operands)),
    }
}
