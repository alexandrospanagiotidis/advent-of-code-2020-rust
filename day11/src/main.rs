use std::io::{BufRead, stdin};

#[derive(Clone, Debug, PartialEq)]
enum SeatType {
    None,
    Empty,
    Occupied,
}

type GridDimensionType = i32;

#[derive(Clone, Debug)]
struct SeatGrid {
    width: GridDimensionType,
    height: GridDimensionType,
    seats: Vec<SeatType>,
}

type SeatStateMapper = dyn Fn(&SeatGrid, GridDimensionType, GridDimensionType) -> Option<SeatType>;

impl SeatGrid {
    fn from(lines: &Vec<String>) -> Self {
        let width = lines.first()
            .expect(format!("Cannot convert to SeatGrid from invalid/empty input: {0:?}", lines).as_str())
            .len() as GridDimensionType;

        let height = lines.len() as GridDimensionType;

        let seats: Vec<SeatType> = lines.into_iter()
            .flat_map(|line| line.chars())
            .map(|char| SeatGrid::char_to_seat_type(char))
            .collect();

        Self {
            width,
            height,
            seats,
        }
    }

    fn char_to_seat_type(char: char) -> SeatType {
        match char {
            '.' => SeatType::None,
            'L' => SeatType::Empty,
            '#' => SeatType::Occupied,
            _ => panic!(format!("Invalid seat type: {0}", char))
        }
    }

    fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn seat_index(&self, x: GridDimensionType, y: GridDimensionType) -> Option<GridDimensionType> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = x + y * self.width;
            Some(index)
        } else {
            None
        }
    }

    fn seat_at(&self, x: GridDimensionType, y: GridDimensionType) -> Option<&SeatType> {
        let result = self.seat_index(x, y)
            .map(|index| self.seats.get(index as usize).unwrap());

        // println!("seat_at({0}, {1})={2:?}", x, y, result);

        result
    }

    fn is_occupied(&self, x: GridDimensionType, y: GridDimensionType) -> Option<bool> {
        let result = self.seat_at(x, y)
            .map(|tile| *tile == SeatType::Occupied);

        // println!("is_occupied({0}, {1})={2:?}", x, y, result);

        result
    }

    fn num_adjacent_occupied(&self, x: GridDimensionType, y: GridDimensionType) -> usize {
        let adjacent_seats = vec![
            self.is_occupied(x - 1, y - 1),
            self.is_occupied(x, y - 1),
            self.is_occupied(x + 1, y - 1),
            self.is_occupied(x - 1, y),
            self.is_occupied(x + 1, y),
            self.is_occupied(x - 1, y + 1),
            self.is_occupied(x, y + 1),
            self.is_occupied(x + 1, y + 1),
        ];

        adjacent_seats.into_iter()
            .filter(|maybe_seat| maybe_seat.is_some())
            .map(|maybe_seat| maybe_seat.unwrap())
            .filter(|&seat_occupied| seat_occupied)
            .count()
    }

    fn apply_rules(&self, seat_state_mapper: &SeatStateMapper) -> SeatGrid {
        let mut after: Vec<SeatType> = Vec::new();
        after.reserve_exact(self.len());

        for y in 0..self.height {
            for x in 0..self.width {
                after.push(seat_state_mapper(self, x, y).unwrap());
            }
        }

        SeatGrid {
            width: self.width,
            height: self.height,
            seats: after,
        }
    }
}

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    // println!("input={0:?}", input);

    let seat_grid = SeatGrid::from(&input);

    let (part1_grid, part1_occupied_seats) = part1(&seat_grid);
    println!("part1_occupied_seats={0:?} -> {1:?}", part1_occupied_seats, part1_grid);

    let (part2_grid, part2_occupied_seats) = part2(&seat_grid);
    println!("part2_occupied_seats={0:?} -> {1:?}", part2_occupied_seats, part2_grid);
}

fn part1_seat_state_mapper(seat_grid: &SeatGrid, x: GridDimensionType, y: GridDimensionType) -> Option<SeatType> {
    seat_grid.seat_at(x, y)
        .map(|current_seat| {
            // println!("seats={3:?} seat({0}, {1})={2:?}", x, y, current_seat, self.seats);
            match current_seat {
                SeatType::Empty => {
                    let adjacent_occupied_seats = seat_grid.num_adjacent_occupied(x, y);
                    if adjacent_occupied_seats == 0 {
                        SeatType::Occupied
                    } else {
                        current_seat.clone()
                    }
                }
                SeatType::Occupied => {
                    let adjacent_occupied_seats = seat_grid.num_adjacent_occupied(x, y);
                    if adjacent_occupied_seats >= 4 {
                        SeatType::Empty
                    } else {
                        current_seat.clone()
                    }
                }
                SeatType::None => SeatType::None,
            }
        })
}

fn part1(seat_grid: &SeatGrid) -> (SeatGrid, usize) {
    let mut previous_grid: SeatGrid = seat_grid.clone();

    let mut _rounds = 0;

    'mutate: loop {
        _rounds += 1;

        let next_grid = previous_grid.apply_rules(&part1_seat_state_mapper);

        if previous_grid.seats == next_grid.seats {
            break 'mutate;
        }

        previous_grid = next_grid;
    }

    println!("part1 finished after {0} rounds of shuffling", _rounds);

    let occupied_seats = previous_grid.seats.iter()
        .filter(|&seat| *seat == SeatType::Occupied)
        .count();

    (previous_grid, occupied_seats)
}

fn part2_walk(seat_grid: &SeatGrid, x: GridDimensionType, y: GridDimensionType, dx: GridDimensionType, dy: GridDimensionType) -> usize {
    let mut column = x + dx;
    let mut row = y + dy;

    while column >= 0 && column < seat_grid.width
        && row >= 0 && row < seat_grid.height {
        match seat_grid.seat_at(column, row).unwrap() {
            SeatType::None => (),
            SeatType::Empty => return 0,
            SeatType::Occupied => return 1,
        }

        column += dx;
        row += dy;
    }

    0
}

fn part2_adjacency_count(seat_grid: &SeatGrid, x: GridDimensionType, y: GridDimensionType) -> usize {
    part2_walk(seat_grid, x, y, -1, 0) // left
        + part2_walk(seat_grid, x, y, 1, 0) // right
        + part2_walk(seat_grid, x, y, 0, -1) // top
        + part2_walk(seat_grid, x, y, 0, 1) // bottom
        + part2_walk(seat_grid, x, y, -1, -1) // top-left
        + part2_walk(seat_grid, x, y, 1, -1) // top-right
        + part2_walk(seat_grid, x, y, -1, 1) // bottom-left
        + part2_walk(seat_grid, x, y, 1, 1) // bottom-right
}

fn part2_seat_state_mapper(seat_grid: &SeatGrid, x: GridDimensionType, y: GridDimensionType) -> Option<SeatType> {
    seat_grid.seat_at(x, y)
        .map(|current_seat| {
            // println!("seats={3:?} seat({0}, {1})={2:?}", x, y, current_seat, self.seats);
            match current_seat {
                SeatType::Empty => {
                    let adjacent_occupied_seats = part2_adjacency_count(seat_grid, x, y);
                    if adjacent_occupied_seats == 0 {
                        SeatType::Occupied
                    } else {
                        current_seat.clone()
                    }
                }
                SeatType::Occupied => {
                    let adjacent_occupied_seats = part2_adjacency_count(seat_grid, x, y);
                    if adjacent_occupied_seats >= 5 {
                        SeatType::Empty
                    } else {
                        current_seat.clone()
                    }
                }
                SeatType::None => SeatType::None,
            }
        })
}

fn part2(seat_grid: &SeatGrid) -> (SeatGrid, usize) {
    let mut previous_grid: SeatGrid = seat_grid.clone();

    let mut _rounds = 0;

    'mutate: loop {
        _rounds += 1;

        let next_grid = previous_grid.apply_rules(&part2_seat_state_mapper);

        if previous_grid.seats == next_grid.seats {
            break 'mutate;
        }

        previous_grid = next_grid;
    }

    println!("part2 finished after {0} rounds of shuffling", _rounds);

    let occupied_seats = previous_grid.seats.iter()
        .filter(|&seat| *seat == SeatType::Occupied)
        .count();

    (previous_grid, occupied_seats)
}

fn seat_grid_to_input_format(seat_grid: &SeatGrid) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for y in 0..seat_grid.height {
        let mut row = String::new();

        for x in 0..seat_grid.width {
            row.push(match seat_grid.seat_at(x, y).unwrap() {
                SeatType::None => '.',
                SeatType::Empty => 'L',
                SeatType::Occupied => '#',
            });
        }

        result.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example1_data() -> Vec<String> {
        vec!["L.LL.LL.LL", "LLLLLLL.LL", "L.L.L..L..", "LLLL.LL.LL", "L.LL.LL.LL", "L.LLLLL.LL", "..L.L.....", "LLLLLLLLLL", "L.LLLLLL.L", "L.LLLLL.LL"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>()
    }

    #[test]
    fn seat_grid_char_to_seat_type_returns_none_for_dot() {
        assert_eq!(SeatGrid::char_to_seat_type('.'), SeatType::None);
    }

    #[test]
    #[allow(non_snake_case)]
    fn seat_grid_char_to_seat_type_returns_empty_for_L() {
        assert_eq!(SeatGrid::char_to_seat_type('L'), SeatType::Empty);
    }

    #[warn(non_snake_case)]
    #[test]
    fn seat_grid_char_to_seat_type_returns_occupied_for_hashtag() {
        assert_eq!(SeatGrid::char_to_seat_type('#'), SeatType::Occupied);
    }

    #[test]
    fn seat_grid_to_input_format_works() {
        let seat_grid = SeatGrid::from(&example1_data());

        let input_data = seat_grid_to_input_format(&seat_grid);

        assert_eq!(input_data, example1_data());
    }

    #[test]
    fn seat_grid_num_adjacent_occupied_returns_0_when_all_neighbors_empty() {
        let lines = vec!["LLL", "L#L", "LLL"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let num_neighbors = seat_grid.num_adjacent_occupied(1, 1);

        assert_eq!(num_neighbors, 0);
    }

    #[test]
    fn seat_grid_num_adjacent_occupied_returns_0_when_all_neighbors_floor() {
        let lines = vec!["...", ".#.", "..."]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let num_neighbors = seat_grid.num_adjacent_occupied(1, 1);

        assert_eq!(num_neighbors, 0);
    }

    #[test]
    fn seat_grid_num_adjacent_occupied_returns_8_when_all_neighbors_occupied() {
        let lines = vec!["###", "###", "###"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let num_neighbors = seat_grid.num_adjacent_occupied(1, 1);

        assert_eq!(num_neighbors, 8);
    }

    #[test]
    fn seat_grid_num_adjacent_occupied_returns_4_when_4_neighbors_occupied() {
        let lines = vec!["L#L", "###", "L#L"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let num_neighbors = seat_grid.num_adjacent_occupied(1, 1);

        assert_eq!(num_neighbors, 4);
    }

    #[test]
    fn seat_grid_apply_rules_returns_occupied_when_no_neighbors() {
        let lines = vec!["LLL", "LLL", "LLL"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let next_seat = part1_seat_state_mapper(&seat_grid, 1, 1);

        assert_eq!(next_seat, Some(SeatType::Occupied));
    }

    #[test]
    fn seat_grid_apply_rules_returns_empty_for_at_least_4_occupied_neighbors() {
        let lines = vec!["#.#", ".#.", "#.#"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let next_seat = part1_seat_state_mapper(&seat_grid, 1, 1);

        assert_eq!(next_seat, Some(SeatType::Empty));
    }

    macro_rules! seat_grid_apply_rules_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;

            let input = input.into_iter().map(|str| String::from(str)).collect::<Vec<String>>();
            let expected = expected.into_iter().map(|str| String::from(str)).collect::<Vec<String>>();

            let seat_grid = SeatGrid::from(&input);
            let result = seat_grid.apply_rules(&part1_seat_state_mapper);

            let expected_grid = SeatGrid::from(&expected);
            // println!("orig={0:?}", seat_grid_to_input_format(&result));
            assert_eq!(seat_grid_to_input_format(&result), expected);

            assert_eq!(result.len(), expected_grid.len());
            assert_eq!(result.width, expected_grid.width);
            assert_eq!(result.height, expected_grid.height);
            assert_eq!(result.seats, expected_grid.seats);
        }
    )*
    }
}

    seat_grid_apply_rules_tests! {
        example1_round1: (
            vec!["L.LL.LL.LL", "LLLLLLL.LL", "L.L.L..L..", "LLLL.LL.LL", "L.LL.LL.LL", "L.LLLLL.LL", "..L.L.....", "LLLLLLLLLL", "L.LLLLLL.L", "L.LLLLL.LL"],
            vec!["#.##.##.##", "#######.##", "#.#.#..#..", "####.##.##", "#.##.##.##", "#.#####.##", "..#.#.....", "##########", "#.######.#", "#.#####.##"],
        ),
        example1_round2: (
            vec!["#.##.##.##", "#######.##", "#.#.#..#..", "####.##.##", "#.##.##.##", "#.#####.##", "..#.#.....", "##########", "#.######.#", "#.#####.##"],
            vec!["#.LL.L#.##", "#LLLLLL.L#", "L.L.L..L..", "#LLL.LL.L#", "#.LL.LL.LL", "#.LLLL#.##", "..L.L.....", "#LLLLLLLL#", "#.LLLLLL.L", "#.#LLLL.##"],
        ),
        example1_round3: (
            vec!["#.LL.L#.##", "#LLLLLL.L#", "L.L.L..L..", "#LLL.LL.L#", "#.LL.LL.LL", "#.LLLL#.##", "..L.L.....", "#LLLLLLLL#", "#.LLLLLL.L", "#.#LLLL.##"],
            vec!["#.##.L#.##", "#L###LL.L#", "L.#.#..#..", "#L##.##.L#", "#.##.LL.LL", "#.###L#.##", "..#.#.....", "#L######L#", "#.LL###L.L", "#.#L###.##"],
        ),
        example1_round4: (
            vec!["#.##.L#.##", "#L###LL.L#", "L.#.#..#..", "#L##.##.L#", "#.##.LL.LL", "#.###L#.##", "..#.#.....", "#L######L#", "#.LL###L.L", "#.#L###.##"],
            vec!["#.#L.L#.##", "#LLL#LL.L#", "L.L.L..#..", "#LLL.##.L#", "#.LL.LL.LL", "#.LL#L#.##", "..L.L.....", "#L#LLLL#L#", "#.LLLLLL.L", "#.#L#L#.##"],
        ),
        example1_round5: (
            vec!["#.#L.L#.##", "#LLL#LL.L#", "L.L.L..#..", "#LLL.##.L#", "#.LL.LL.LL", "#.LL#L#.##", "..L.L.....", "#L#LLLL#L#", "#.LLLLLL.L", "#.#L#L#.##"],
            vec!["#.#L.L#.##", "#LLL#LL.L#", "L.#.L..#..", "#L##.##.L#", "#.#L.LL.LL", "#.#L#L#.##", "..L.L.....", "#L#L##L#L#", "#.LLLLLL.L", "#.#L#L#.##"],
        ),
        example1_round6: (
            vec!["#.#L.L#.##", "#LLL#LL.L#", "L.#.L..#..", "#L##.##.L#", "#.#L.LL.LL", "#.#L#L#.##", "..L.L.....", "#L#L##L#L#", "#.LLLLLL.L", "#.#L#L#.##"],
            vec!["#.#L.L#.##", "#LLL#LL.L#", "L.#.L..#..", "#L##.##.L#", "#.#L.LL.LL", "#.#L#L#.##", "..L.L.....", "#L#L##L#L#", "#.LLLLLL.L", "#.#L#L#.##"],
        ),
    }

    #[test]
    fn part1_example1() {
        let seat_grid = SeatGrid::from(&example1_data());
        let (part1_result, _) = part1(&seat_grid);

        let expected = vec!["#.#L.L#.##", "#LLL#LL.L#", "L.#.L..#..", "#L##.##.L#", "#.#L.LL.LL", "#.#L#L#.##", "..L.L.....", "#L#L##L#L#", "#.LLLLLL.L", "#.#L#L#.##"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let expected_grid = SeatGrid::from(&expected);

        assert_eq!(part1_result.len(), expected_grid.len());
        assert_eq!(part1_result.width, expected_grid.width);
        assert_eq!(part1_result.height, expected_grid.height);
        assert_eq!(part1_result.seats, expected_grid.seats);
    }

    #[test]
    fn part1_works() {
        let input = vec![
            "LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLLLLLLLLL.LLLLL.LLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLLL.LLLLLL",
            "LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "L.L...L...LL.LL.......LL...LL.L...LL..LL..L.......LLLLL.....LL..LLLL.L....L..L...L.LL....LL...L",
            "LLLLL.LLLLL.LLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LLLLLLLLLLLLL.LL.LLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LL.LLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLL.LLLLLL",
            "LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLLLLLLL.LLL.LL.LLLLLLLLL.LLLL.LLLL.LLLL.L.LLLL",
            "LLLLL.LLLL.LLLL.LLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL",
            "LLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLLL.LLL.LL.LLLLLLLLLLLLLLLLLLL.LLLL.LLLLLL",
            ".L.....L.LL..LLL.L..L...L.LLL.L...L.L.L.L.....L..L.......L.LLL...L.......L.LLLL......L.L.L...LL",
            "LLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL..LLLLLL.LLLL.LLLL.L.LL.LLLLLLLLL.LLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LL.LLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            ".L..LLL.L...LL.....LL......LL...L...LL...L.L..L....L.L.L.LL.L........L....LL......L..LL..LL....",
            "LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLL",
            "LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLL..LLLL.LLLLLL.LLLLLLLLLLL.LL.LLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL..L.LLL.LLLLLLLLL.LLLL.LLLLLLLLLLLLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLL.LLLLLLLLLL.LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLLLLLLLLL",
            "L.LLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLL",
            "LLLL....L.......L..LLL...........L..L...LL..L.L.LLL...L.....LL..LL..L....L....L..LL..LL.L....L.",
            "LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.L.LLLLLLLLLLLL.LLLLLL",
            "LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLLLL",
            "LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLLL.LLLLLLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLL..LLL.LLLL.LLLL.LLLL.LLLLLLLLLLL",
            "LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLL.L.LLLL.LL.L.LL.LLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            ".LL........L.....L.L.L.......LL.L.L.......LLL.........L....LL........L.L..L......L.LL......L..L",
            "LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLL.L.LLLLLLLLLLLLLL",
            "LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLL.LLLLLLLLLLLLLLLL",
            "LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.L.LLLL.LLLLLL.LLLLLLLLL.LLLL.LLLLLLLLL.LLLLLL",
            "LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLL..LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLL.LL.L.LLLLLLLLLLLLLLLL.LLLL.LLLLLLLLL.LLLLLL",
            ".L..L...L........L.......L.L.LL.LLLL...LLL.L.L..L.L....L.................L.L.L.L....L...L...L..",
            "LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL.LLLL",
            "LLLLL.LL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLLLLLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLL...LLL.......LL..L.L.L.L...L........LL..............L.L......L.......L..LL....L....LL...LL.",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLL.LLLLLL.L.LLLLLLL.LLLLLLLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL..LLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.L.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLL.LLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLLL.L.LL.LLLLLL.LLLLLLLLLLL.LL.LLLLLLLLL.LLL.LL",
            "LLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLL.LLLLLLLLLLLLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LL.LL.LLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLL.LLLLLL",
            "LLLL.L...LL.......LL..L.....L.LL..L....L.L..L.......L..L......LLLLL..L.L..L......L...L.L...L.L.",
            "LLLLL.LLL..LLLL.LLLLLL.LLLLL.LLL.LLLLL.L.LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLL..LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLL.LLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL",
            "LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLL.LLLLLLLLLLLL.LLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "L.L...............L..L.LLLL...L..L...LLL.......LLL.LL........L..LL..L..L...L.L.L.LL..LLLL.L.LL.",
            "LLLLLLLL.L.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLLLLLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLLLLLL",
            "LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLLLLLLLLL",
            "LLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLL.LLLLLLLLLLLLLLLL",
            "L.LLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLL.LLLLLLL.LLLL.LLLL.LLLLLL",
            ".L..LL.L.L..L...L....L......LLL......L.LL..L....L.LLLL.LL.....L.L.LL.L.....L......L.LL.........",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLL..LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLL.LLLLLLLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLL",
            "LLLLL.LLL..LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLL..LLL.LLLLLLLLLLLLLLLL",
            ".LLL......L.L.L......L.....LL......L.LLL.LLL..LL...L.L.......L..L.......L....L.....L.......LL..",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLLLLLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LL...L...........L...L..L......LL...........L...L.LL..LL....L....LLL.LLLL....LLL...LL..L..L...L",
            "LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLL.LLLLLLLLLLLLL.LLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL",
            ".LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL",
            "LLLLL.LL.LLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLL.LLLLLLLLLLL.LLLLLLLL..LLLL.LLL..LLLLLLLLLLL",
            ".......L...LL.L.....L.L.....LLL.L.......L.....LL.......L..LLL.....L.LL.L..........LL...........",
            "LLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLLLL.LL.LL.LLL.LLLLLLLLLLLLLL.LLLL.LLLLLLLLLLL",
            "LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLL..LLLLLLLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLL.LLLL.LLLL.LLLL.LLLLLL",
            "LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLL",
        ]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&input);
        let (_part1_result, part1_occupied_seats) = part1(&seat_grid);

        assert_eq!(part1_occupied_seats, 2406);
    }

    #[test]
    fn part2_visibility_check1() {
        let input = vec![".......#.", "...#.....", ".#.......", ".........", "..#L....#", "....#....", ".........", "#........", "...#....."]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&input);
        let adjacency_count = part2_adjacency_count(&seat_grid, 3, 4);

        assert_eq!(adjacency_count, 8);
    }

    #[test]
    fn part2_visibility_check2() {
        let input = vec![".............", ".L.L.#.#.#.#.", "............."]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&input);
        let adjacency_count = part2_adjacency_count(&seat_grid, 1, 1);

        assert_eq!(adjacency_count, 0);
    }

    #[test]
    fn part2_visibility_check3() {
        let input = vec![".##.##.", "#.#.#.#", "##...##", "...L...", "##...##", "#.#.#.#", ".##.##."]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&input);
        let adjacency_count = part2_adjacency_count(&seat_grid, 3, 3);

        assert_eq!(adjacency_count, 0);
    }
}
