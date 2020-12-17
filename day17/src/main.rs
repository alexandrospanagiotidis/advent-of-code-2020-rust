trait Coordinate: Copy + PartialEq + Sized {
    fn new2d(x: i32, y: i32) -> Self;
    fn adjacent_coordinates(&self) -> Vec<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate3d(i32, i32, i32);

impl Coordinate3d {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coordinate3d {
            0: x,
            1: y,
            2: z,
        }
    }
}

impl Coordinate for Coordinate3d {
    fn new2d(x: i32, y: i32) -> Self {
        Coordinate3d::new(x, y, 0)
    }

    fn adjacent_coordinates(&self) -> Vec<Self> {
        let mut coordinates = Vec::with_capacity(3 * 3 * 3 - 1);

        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dz == 0 && dy == 0 && dx == 0 {
                        continue;
                    }

                    coordinates.push(Self::new(
                        self.0 + dx,
                        self.1 + dy,
                        self.2 + dz,
                    ));
                }
            }
        }

        coordinates
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate4d(i32, i32, i32, i32);

impl Coordinate4d {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Coordinate4d {
            0: x,
            1: y,
            2: z,
            3: w,
        }
    }
}

impl Coordinate for Coordinate4d {
    fn new2d(x: i32, y: i32) -> Self {
        Coordinate4d(x, y, 0, 0)
    }

    fn adjacent_coordinates(&self) -> Vec<Self> {
        let mut coordinates = Vec::with_capacity(4 * 4 * 4 * 4 - 1);

        for dw in -1..=1 {
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dw == 0 && dz == 0 && dy == 0 && dx == 0 {
                            continue;
                        }

                        coordinates.push(Self::new(
                            self.0 + dx,
                            self.1 + dy,
                            self.2 + dz,
                            self.3 + dw,
                        ));
                    }
                }
            }
        }

        coordinates
    }
}

#[derive(Debug, PartialEq)]
struct CubeGrid<CoordinateType: Coordinate> {
    active: Vec<CoordinateType>,
}

impl<CoordinateType: Coordinate> Clone for CubeGrid<CoordinateType> {
    fn clone(&self) -> Self {
        let vec = self.active.clone();

        CubeGrid {
            active: vec,
        }
    }
}

impl<CoordinateType: Coordinate> CubeGrid<CoordinateType> {
    fn new() -> Self {
        CubeGrid {
            active: Vec::new(),
        }
    }

    fn add_active(&mut self, coordinate: CoordinateType) {
        if !self.active.contains(&coordinate) {
            self.active.push(coordinate);
        }
    }

    fn is_active(&self, coordinate: CoordinateType) -> bool {
        self.active.contains(&coordinate)
    }

    fn active_neighbors(&self, coordinate: CoordinateType) -> usize {
        let mut active = 0;

        for neighbor_coordinate in coordinate.adjacent_coordinates() {
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

    {
        let mut cube_grid: CubeGrid<Coordinate3d> = read_input(input);

        let part1_grid = simulate(&mut cube_grid, 6);
        let num_active = part1_grid.count_active();

        println!("part1: num_active={0}", num_active);
        assert_eq!(num_active, 276)
    }
    {
        let mut cube_grid: CubeGrid<Coordinate4d> = read_input(input);

        let part2_grid = simulate(&mut cube_grid, 6);
        let num_active = part2_grid.count_active();

        println!("part2: num_active={0}", num_active);
        assert_eq!(num_active, 2136)
    }
}

fn read_input<CoordinateType: Coordinate>(input: &str) -> CubeGrid<CoordinateType> {
    let mut x;
    let mut y = 0;
    let z = 0;

    let mut cube_grid = CubeGrid::new();

    for line in input.split_whitespace() {
        x = 0;
        for cube in line.chars() {
            match cube {
                '.' => (),
                '#' => cube_grid.add_active(CoordinateType::new2d(x, y)),
                _ => panic!(format!("Invalid state '{3}' at ({0}, {1}, {2}) (line={4})", x, y, z, cube, line)),
            };

            x += 1;
        }
        y += 1;
    }

    cube_grid
}

fn simulate<CoordinateType: Coordinate>(cube_grid: &CubeGrid<CoordinateType>, rounds: usize) -> CubeGrid<CoordinateType> {
    let mut cube_grid = cube_grid.clone();

    for _i in 0..rounds {
        cube_grid = simulate_once(&mut cube_grid);
    }

    cube_grid
}

fn simulate_once<CoordinateType: Coordinate>(cube_grid: &CubeGrid<CoordinateType>) -> CubeGrid<CoordinateType> {
    let mut next_grid = CubeGrid::new();

    let mut coordinates: Vec<CoordinateType> = Vec::new();

    for &coordinate in &cube_grid.active {
        coordinates.extend(coordinate.adjacent_coordinates());
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

        // println!();
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

        let mut cube_grid: CubeGrid<Coordinate3d> = read_input(input);

        let cube_grid = simulate(&mut cube_grid, 6);

        // println!("cube_grid={0:#?}", cube_grid);

        let num_active = cube_grid.count_active();

        assert_eq!(num_active, 112)
    }

    // Kinda does not terminate
    #[test]
    fn part2_example1() {
        let input = r"
.#.
..#
###
";

        let mut cube_grid: CubeGrid<Coordinate4d> = read_input(input);

        let cube_grid = simulate(&mut cube_grid, 6);

        // println!("cube_grid={0:#?}", cube_grid);

        let num_active = cube_grid.count_active();

        assert_eq!(num_active, 848)
    }
}
