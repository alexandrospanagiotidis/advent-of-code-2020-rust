use std::io::{BufRead, stdin};

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
struct NavigationState {
    north: i32,
    east: i32,
    facing: Direction,
}

impl NavigationState {
    fn new() -> NavigationState {
        NavigationState {
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

    {
        let ship_state = part1(&input);
        let manhattan_distance = manhattan_distance(ship_state);
        println!("part1: {0}", manhattan_distance);
    }
    {
        let waypoint = NavigationState {
            north: 1,
            east: 10,
            facing: Direction::North,
        };

        let ship_state = part2(&input, &NavigationState::new(), &waypoint);
        let manhattan_distance = manhattan_distance(ship_state);
        println!("part2: {0}", manhattan_distance);
    }
}

fn manhattan_distance(ship_state: NavigationState) -> i32 {
    ship_state.north.abs() + ship_state.east.abs()
}

fn part1(input: &Vec<String>) -> NavigationState {
    let mut ship_state = NavigationState::new();

    for instruction in input {
        let instruction = instruction.trim();

        if instruction.is_empty() {
            continue;
        }

        let command = &instruction[..1];
        let argument = instruction[1..].parse::<i32>()
            .expect(format!("Could not convert to i32: {0}", instruction).as_str());

        println!("before {0}: ship={1:?}", instruction, ship_state);
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

fn part2(input: &Vec<String>, ship_state: &NavigationState, waypoint: &NavigationState) -> NavigationState {
    let mut ship_state = (*ship_state).clone();
    let mut waypoint = (*waypoint).clone();

    for instruction in input {
        let instruction = instruction.trim();

        if instruction.is_empty() {
            continue;
        }

        let command = &instruction[..1];
        let argument = instruction[1..].parse::<i32>()
            .expect(format!("Could not convert to i32: {0}", instruction).as_str());

        println!("before {0}: ship={1:?} waypoint={2:?}", instruction, ship_state, waypoint);
        match command {
            "F" => {
                let dx = waypoint.east * argument;
                let dy = waypoint.north * argument;
                println!("Delta=({0}, {1})", dx, dy);
                ship_state.north += dy;
                ship_state.east += dx;
            }
            "N" => waypoint.north += argument,
            "E" => waypoint.east += argument,
            "S" => waypoint.north -= argument,
            "W" => waypoint.east -= argument,
            "L" => {
                let dx = waypoint.east;
                let dy = waypoint.north;
                let (x, y) = rotate(dx, dy, argument);
                waypoint.north = y;
                waypoint.east = x;
            },
            "R" => {
                let dx = waypoint.east;
                let dy = waypoint.north;
                let (x, y) = rotate(dx, dy, -argument);
                waypoint.north = y;
                waypoint.east = x;
            }
            _ => panic!(format!("Invalid instruction: {0}", instruction)),
        }
        println!("after {0}: ship={1:?} waypoint={2:?}", instruction, ship_state, waypoint);
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

fn rotate(x: i32, y: i32, degrees: i32) -> (i32, i32) {
    print!("Turning {0},{1} for {2} degrees", x, y, degrees);

    let x = x as f32;
    let y = y as f32;

    let degrees: f32 = degrees as f32 * std::f32::consts::PI / 180.0;

    let d_cos = degrees.cos();
    let d_sin = degrees.sin();

    let new_x = x * d_cos - y * d_sin;
    let new_y = x * d_sin + y * d_cos;

    println!(" = {0},{1}", new_x, new_y);

    (
        new_x.round() as i32,
        new_y.round() as i32,
    )
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

        let ship_state = part1(input, &NavigationState::new());
        let result = manhattan_distance(ship_state);

        assert_eq!(result, 25);
    }

    #[test]
    fn part2_example1() {
        let input: Vec<String> = r"F10
N3
F7
R90
F11
".split('\n')
            .into_iter()
            .map(|s| String::from(s))
            .collect();

        let ship_state = NavigationState::new();
        let waypoint = NavigationState {
            north: 1,
            east: 10,
            facing: Direction::North,
        };

        let ship_state = part2(input, &ship_state, &waypoint);
        let result = manhattan_distance(ship_state);

        assert_eq!(result, 286);
    }
}
