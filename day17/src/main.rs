use std::collections::HashSet;

type Coordinate = (i32, i32, i32);

#[derive(Debug)]
struct CubeGrid {
    active: HashSet<Coordinate>,
}

impl CubeGrid {
    fn new() -> Self {
        CubeGrid {
            active: HashSet::new(),
        }
    }

    fn from(other: &Self) -> Self {
        CubeGrid {
            active: other.active.clone(),
        }
    }

    fn add_active(&mut self, coordinate: Coordinate) {
        self.active.insert(coordinate);
    }

    fn is_active(&self, coordinate: Coordinate) -> bool {
        self.active.contains(&coordinate)
    }

    fn adjacent_coordinates(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let mut coordinates = Vec::with_capacity(26);

        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dz == 0 && dy == 0 && dx == 0 {
                        continue;
                    }

                    coordinates.push((
                        coordinate.0 + dx,
                        coordinate.1 + dy,
                        coordinate.2 + dz
                    ));
                }
            }
        }

        coordinates
    }

    fn active_neighbors(&self, coordinate: Coordinate) -> usize {
        let mut active = 0;

        for neighbor_coordinate in self.adjacent_coordinates(coordinate) {
            if self.is_active(neighbor_coordinate) {
                active += 1;
            }
        }

        active
    }

    fn count_active(self: &Self) -> usize {
        self.active.len()
    }
}

fn main() {
    let input = r"
.#.####.
.#...##.
..###.##
#..#.#.#
#..#....
#.####..
##.##..#
#.#.#..#
    ";

    let mut cube_grid = read_input(input);

    // println!("cube_Grid={0:#?}", cube_grid);

    let part1_grid = simulate(&mut cube_grid, 6);
    println!("part1: num_active={0}", part1_grid.count_active());
}

fn read_input(input: &str) -> CubeGrid {
    let mut x;
    let mut y = 0;
    let z = 0;

    let mut cube_grid = CubeGrid::new();

    for line in input.split_whitespace() {
        x = 0;
        for cube in line.chars() {
            match cube {
                '.' => (),
                '#' => cube_grid.add_active((x, y, z)),
                _ => panic!(format!("Invalid state '{3}' at ({0}, {1}, {2}) (line={4})", x, y, z, cube, line)),
            };

            x += 1;
        }
        y += 1;
    }

    cube_grid
}

fn simulate(cube_grid: &CubeGrid, rounds: usize) -> CubeGrid {
    let mut cube_grid = CubeGrid::from(cube_grid);

    for _i in 0..rounds {
        cube_grid = simulate_once(&mut cube_grid);
    }

    cube_grid
}

fn simulate_once(cube_grid: &CubeGrid) -> CubeGrid {
    let mut next_grid = CubeGrid::new();

    let mut coordinates: Vec<Coordinate> = Vec::new();

    for &coordinate in &cube_grid.active {
        coordinates.extend(cube_grid.adjacent_coordinates(coordinate));
    }

    coordinates.extend(cube_grid.active.clone());

    for cube in &coordinates {
        let active_neighbors = &cube_grid.active_neighbors(*cube);

        // print!("{0:?} active_neighbors={1}", cube, active_neighbors);

        if cube_grid.is_active(*cube) {
            // print!(" is active");
            if *active_neighbors == 2 || *active_neighbors == 3 {
                // print!(" -> active");
                next_grid.add_active(*cube);
            }
        } else {
            // print!(" is INactive");
            if *active_neighbors == 3 {
                // print!(" -> active");
                next_grid.add_active(*cube);
            }
        }

        println!();
    }

    // println!("next_grid={0:?}", next_grid);
    next_grid
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = r"
.#.
..#
###
";

        let mut cube_grid = read_input(input);

        let cube_grid = simulate(&mut cube_grid, 6);

        // println!("cube_grid={0:#?}", cube_grid);

        let num_active = cube_grid.count_active();

        assert_eq!(num_active, 112)
    }
}
