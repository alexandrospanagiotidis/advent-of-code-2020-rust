use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::thread::current;

#[derive(Debug)]
enum CommandType {
    MASK,
    MEM,
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    mask_value: String,
    mem_address: usize,
    mem_value: u64,
}

type InstructionTable = Vec<Command>;

#[derive(Debug)]
struct State {
    current_mask: String,
    memory: HashMap<usize, u64>,
}

impl Command {
    fn new_mask(mask: &str) -> Self {
        Command {
            command_type: CommandType::MASK,
            mask_value: String::from(mask),
            mem_address: 0,
            mem_value: 0,
        }
    }

    fn new_mem(address: usize, value: u64) -> Self {
        Command {
            command_type: CommandType::MEM,
            mask_value: String::new(),
            mem_address: address,
            mem_value: value,
        }
    }
}

impl State {
    fn write(&mut self, address: usize, value: u64) -> &Self {
        let mut value = value;

        println!("value={0} current_mask={1}", value, self.current_mask);

        self.current_mask.chars()
            .rev()
            .enumerate()
            .filter(|(_, bit)| *bit != 'X')
            .map(|(index, bit)|
                (
                    index,
                    bit.to_digit(2)
                        .expect(format!("Could not convert to binary: {0}", bit).as_str())
                )
            )
            .for_each(|(index, bit)| {
                if bit == 0 {
                    print!("Clearing bit {0} in value={1}", index, value);
                    value = value & !(1 << index);
                } else {
                    println!("Setting bit {0} in value={1}", index, value);
                    value |= 1 << index;
                }
                println!(" -> value={0}", value);
            });

        println!(" -> shifted={0}", value);

        *self.memory.entry(address).or_insert(0) = value;

        self
    }
}

fn read_input(lines: &Vec<String>) -> InstructionTable {
    let instructions: InstructionTable = lines.iter()
        .map(|line| {
            let mut tokens = line.split(" = ");

            let first = tokens.next().unwrap();
            let second = tokens.next().unwrap();

            if first.starts_with("mask") {
                Command::new_mask(second)
            } else {
                let address_begin = first.find("[")
                    .expect(format!("Could not find [ in: {0}", line).as_str());

                let address_end = first.find("]")
                    .expect(format!("Could not find ] in: {0}", line).as_str());

                let address_value = &first[address_begin + 1..address_end];
                let address = address_value.parse::<usize>()
                    .expect(format!("Could not convert to usize: {0}", address_value).as_str());

                let value = second.parse::<u64>()
                    .expect(format!("Could not convert to u64: {0}", second).as_str());

                Command::new_mem(address, value)
            }
        })
        // .enumerate()
        .collect();

    instructions
}

fn main() {
    let lines = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let instructions = read_input(&lines);

    println!("instructions={0:?}", instructions);

    let mut state = State {
        current_mask: String::new(),
        memory: HashMap::new(),
    };

    process_instructions(instructions, &mut state);

    // println!("state={0:?}", state);

    let sum: u128 = state.memory.values()
        .map(|&v| v as u128)
        .sum();
    println!("part1: sum={0}", sum);
}

fn process_instructions(instructions: Vec<Command>, state: &mut State) {
    for command in instructions {
        match command.command_type {
            CommandType::MASK => state.current_mask = command.mask_value.clone(),
            CommandType::MEM => {
                state.write(command.mem_address, command.mem_value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = vec![
            r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            r"mem[8] = 11",
            r"mem[7] = 101",
            r"mem[8] = 0",
        ]
            .iter()
            .map(|&l| String::from(l))
            .collect();

        let instructions = read_input(&input);

        let mut state = State {
            current_mask: String::new(),
            memory: HashMap::new(),
        };

        process_instructions(instructions, &mut state);

        assert_eq!(state.memory.get(&7), Some(&101));
        assert_eq!(state.memory.get(&8), Some(&64));
    }
}
