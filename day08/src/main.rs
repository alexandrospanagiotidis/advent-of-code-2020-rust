use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};
use std::ops::Add;
use std::str::FromStr;

use crate::OpCode::{ACC, JMP, NOP};

type InstructionIndex = usize;
type InstructionDelta = i32;

#[derive(Debug, Clone)]
enum OpCode {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: OpCode,
    target: InstructionDelta,
}

impl Instruction {
    fn new(opcode: OpCode, target: InstructionDelta) -> Instruction {
        Instruction {
            opcode,
            target,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let instruction = tokens.next();

        if instruction.is_none() {
            return Err(String::from("Could not extract instruction from: ").add(s));
        }

        let target = tokens.next()
            .map(|target_string| target_string.parse::<InstructionDelta>()
                .expect(String::from("Could not parse target into number: ").add(s).as_str())
            );

        if instruction.is_none() {
            return Err(String::from("Could not extract target from: ").add(s));
        }

        let instruction = instruction.unwrap();
        let target = target.unwrap();

        match instruction.to_lowercase().as_str() {
            "nop" => Ok(Instruction::new(NOP, 0)),
            "acc" => Ok(Instruction::new(ACC, target)),
            "jmp" => Ok(Instruction::new(JMP, target)),
            _ => Err(String::from("Unknown instruction: ").add(s))
        }
    }
}

type InstructionTable = HashMap<InstructionIndex, Instruction>;
type CallStack = Vec<InstructionIndex>;
type Coverage = HashMap<InstructionIndex, u16>;
type Accumulator = i64;

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let mut line_number = 0;

    let instructions: InstructionTable = input.into_iter()
        .map(|line| Instruction::from_str(&line).unwrap())
        .map(|operation| {
            let current_line = line_number;
            line_number += 1;

            (current_line, operation)
        })
        .collect();

    // println!("instructions = {0:?}", instructions);

    part1(&instructions);
    part2(&instructions);
}

fn update_instruction_counter(current_instruction_counter: InstructionIndex, delta: InstructionDelta) -> InstructionIndex {
    if delta.is_negative() {
        current_instruction_counter - delta.wrapping_abs() as u32 as usize
    } else {
        current_instruction_counter + delta as usize
    }
}

fn update_accumulator(accumulator: Accumulator, delta: InstructionDelta) -> Accumulator {
    accumulator + delta as i64
}

fn run_instructions(instructions: &InstructionTable) -> (bool, Accumulator, CallStack, Coverage) {
    let mut call_stack = CallStack::new();
    let mut coverage = Coverage::new();
    let mut instruction_counter: InstructionIndex = call_stack.last().unwrap_or(&0).clone();

    let mut accumulator = 0;
    let successful;

    'run: loop {
        // println!("before: instruction_counter={0}  accumulator={1}", instruction_counter, accumulator);

        if instruction_counter >= instructions.len() {
            successful = true;
            break 'run;
        }

        call_stack.push(instruction_counter);
        *coverage.entry(instruction_counter).or_insert(0) += 1;

        if coverage.get(&instruction_counter).unwrap() > &1 {
            successful = false;
            break 'run;
        }

        let next_instruction = instructions.get(&instruction_counter)
            .expect(String::from("Invalid instruction counter: ").add(&instruction_counter.to_string()).as_str());

        // println!("instruction={:?}", next_instruction);

        match next_instruction.opcode {
            NOP => {
                instruction_counter += 1;
            },
            ACC => {
                accumulator = update_accumulator(accumulator, next_instruction.target);
                instruction_counter += 1;
            },
            JMP => {
                instruction_counter = update_instruction_counter(instruction_counter, next_instruction.target);
            },
        }

        // println!("after: instruction_counter={0}  accumulator={1}", instruction_counter, accumulator);
    }

    // println!("run finished: successful={0} instruction_counter={1} accumulator={2}", successful, instruction_counter, accumulator);
    // println!("call_stack={0:?}", call_stack);
    // println!("coverage={0:?}", coverage);

    (successful, accumulator, call_stack, coverage)
}

fn part1(instructions: &InstructionTable) {
    let (successful, accumulator, _call_stack, _coverage) = run_instructions(&instructions);

    println!("part1: successful={0} accumulator={1}", successful, accumulator);
    println!("part1: call_stack={0:?}", _call_stack);
    println!("part1: coverage={0:?}", _coverage);
}

fn part2(original_instructions: &InstructionTable) {
    let mut instructions = original_instructions.clone();
    let mut accumulator: Accumulator = 0;
    let mut successful = false;

    let mut already_switched: HashSet<InstructionIndex> = HashSet::new();

    let mut _call_stack = CallStack::new();
    let mut _coverage = Coverage::new();

    while !successful {
        let current_execution = run_instructions(&instructions);

        successful = current_execution.0;
        accumulator = current_execution.1;
        _call_stack = current_execution.2;
        _coverage = current_execution.3;

        // println!("part2: successful={0} accumulator={1}", successful, accumulator);
        // println!("part2: call_stack={0:?}", _call_stack);
        // println!("part2: coverage={0:?}", _coverage);

        if !successful {
            instructions = original_instructions.clone();

            if let Some((&instruction_counter, instruction)) = &original_instructions.into_iter()
                .filter(|(instruction_counter, _)| !already_switched.contains(&instruction_counter))
                .nth(0) {
                // println!("changing {0}={1:?}", instruction_counter, instruction);

                let entry = instructions.entry(instruction_counter);

                match instruction.opcode {
                    NOP => entry.and_modify(|i| i.opcode = JMP),
                    JMP => entry.and_modify(|i| i.opcode = NOP),
                    _ => entry,
                };

                already_switched.insert(instruction_counter.to_owned());
            }
        }
    }

    println!("part2: successful={0} accumulator={1}", successful, accumulator);
    println!("part2: call_stack={0:?}", _call_stack);
    println!("part2: coverage={0:?}", _coverage);
}
