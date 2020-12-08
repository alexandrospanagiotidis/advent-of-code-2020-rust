use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::ops::Add;
use std::str::FromStr;

use crate::Instruction::{ACC, JMP, NOP};

type InstructionIndex = i32;

#[derive(Debug)]
enum Instruction {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug)]
struct Operation {
    instruction: Instruction,
    target: InstructionIndex,
}

impl Operation {
    fn new(instruction: Instruction, target: InstructionIndex) -> Operation {
        Operation {
            instruction,
            target,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let instruction = tokens.next();

        if instruction.is_none() {
            return Err(String::from("Could not extract instruction from: ").add(s));
        }

        let target = tokens.next()
            .map(|target_string| target_string.parse::<InstructionIndex>()
                .expect(String::from("Could not parse target into numebr: ").add(s).as_str())
            );

        if instruction.is_none() {
            return Err(String::from("Could not extract target from: ").add(s));
        }

        let instruction = instruction.unwrap();
        let target = target.unwrap();

        match instruction.to_lowercase().as_str() {
            "nop" => Ok(Operation::new(NOP, 0)),
            "acc" => Ok(Operation::new(ACC, target)),
            "jmp" => Ok(Operation::new(JMP, target)),
            _ => Err(String::from("Unknown instruction: ").add(s))
        }
    }
}

type InstructionTable = HashMap<InstructionIndex, Operation>;

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let mut line_number = 0;

    let instructions: InstructionTable = input.into_iter()
        .map(|line| Operation::from_str(&line).unwrap())
        .map(|operation| {
            let current_line = line_number;
            line_number += 1;

            (current_line, operation)
        })
        .collect();

    println!("{0:?}", instructions);

    part1(&instructions);
}

fn part1(instructions: &InstructionTable) {
    let mut instruction_counter: InstructionIndex = 0;
    let mut accumulator = 0i64;

    let mut coverage: HashMap<InstructionIndex, u16> = HashMap::new();

    'run: loop {
        println!("before: instruction_counter={0}  accumulator={1}", instruction_counter, accumulator);

        *coverage.entry(instruction_counter).or_insert(0) += 1;

        if coverage.get(&instruction_counter).unwrap() > &1 {
            break 'run;
        }

        let next_instruction = instructions.get(&instruction_counter)
            .expect(String::from("Invalid instruction counter: ").add(&instruction_counter.to_string()).as_str());

        println!("instruction={:?}", next_instruction);

        match next_instruction.instruction {
            NOP => {
                instruction_counter += 1;
            },
            ACC => {
                accumulator += next_instruction.target as i64;
                instruction_counter += 1;
            },
            JMP => {
                instruction_counter += next_instruction.target;
            },
        }

        println!("after: instruction_counter={0}  accumulator={1}", instruction_counter, accumulator);
    }

    println!("part1: instruction_counter={0}  accumulator={1}  instruction={2:?}",
             instruction_counter, accumulator, instructions.get(&instruction_counter)
    );
}
