use std::io::{BufRead, stdin};

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct ShipState {
    north: i32,
    east: i32,
    facing: Direction,
}

impl ShipState {
    fn new() -> ShipState {
        ShipState {
            north: 0,
            east: 0,
            facing: Direction::East,
        }
    }
}

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    // println!("input={0:?}", input);

    let ship_state = navigate(input, &ShipState::new());

    let mut manhattan_distance = manhattan_distance(ship_state);
    println!("part1: {0}", manhattan_distance);
}

fn manhattan_distance(ship_state: ShipState) -> i32 {
    ship_state.north.abs() + ship_state.east.abs()
}

fn navigate(input: Vec<String>, ship_state: &ShipState) -> ShipState {
    let mut ship_state = ShipState::new();

    for instruction in input {
        let instruction = instruction.trim();

        if instruction.is_empty() {
            continue;
        }

        let command = &instruction[..1];
        let argument = instruction[1..].parse::<i32>()
            .expect(format!("Could not convert to i32: {0}", instruction).as_str());

        println!("before {0}: {1:?}", instruction, ship_state);
        match command {
            "F" => {
                ship_state.north += get_dy(&ship_state.facing) * argument;
                ship_state.east += get_dx(&ship_state.facing) * argument;
            }
            "N" => ship_state.north += argument,
            "E" => ship_state.east += argument,
            "S" => ship_state.north -= argument,
            "W" => ship_state.east -= argument,
            "L" => ship_state.facing = turn(&ship_state.facing, -argument),
            "R" => ship_state.facing = turn(&ship_state.facing, argument),
            _ => panic!(format!("Invalid instruction: {0}", instruction)),
        }
        println!("after {0}: {1:?}", instruction, ship_state);
    }

    ship_state
}

fn direction_to_degrees(facing: &Direction) -> i32 {
    match facing {
        Direction::North => 0,
        Direction::East => 90,
        Direction::South => 180,
        Direction::West => 270,
    }
}

fn degrees_to_direction(degrees: &i32) -> Direction {
    match degrees {
        0 | 360 => Direction::North,
        90 => Direction::East,
        180 => Direction::South,
        270 => Direction::West,
        _ => panic!(format!("Cannot convert {0} degrees to direction", degrees)),
    }
}

fn turn(facing: &Direction, degrees: i32) -> Direction {
    let mut current_degrees = direction_to_degrees(facing);

    print!("Turning {0:?}={1} for {2} degrees", facing, current_degrees, degrees);

    current_degrees = (degrees + current_degrees) % 360;

    while current_degrees.is_negative() {
        current_degrees += 360;
    }

    let facing = degrees_to_direction(&current_degrees);

    println!(" = {0:?}", facing);

    facing
}

fn get_dx(direction: &Direction) -> i32 {
    match direction {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 0,
        Direction::West => -1,
    }
}

fn get_dy(direction: &Direction) -> i32 {
    match direction {
        Direction::North => 1,
        Direction::East => 0,
        Direction::South => -1,
        Direction::West => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input: Vec<String> = r"F10
N3
F7
R90
F11
".split('\n')
            .into_iter()
            .map(|s| String::from(s))
            .collect();

        let ship_state = navigate(input, &ShipState::new());
        let result = manhattan_distance(ship_state);

        assert_eq!(result, 25);
    }
}
