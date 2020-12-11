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

    fn apply_rules_at(&self, x: GridDimensionType, y: GridDimensionType) -> Option<SeatType> {
        self.seat_at(x, y)
            .map(|current_seat| {
                // println!("seats={3:?} seat({0}, {1})={2:?}", x, y, current_seat, self.seats);
                match current_seat {
                    SeatType::Empty => {
                        let adjacent_occupied_seats = self.num_adjacent_occupied(x, y);
                        if adjacent_occupied_seats == 0 {
                            SeatType::Occupied
                        } else {
                            current_seat.clone()
                        }
                    }
                    SeatType::Occupied => {
                        let adjacent_occupied_seats = self.num_adjacent_occupied(x, y);
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

    fn apply_rules(&self) -> SeatGrid {
        let mut after: Vec<SeatType> = Vec::new();
        after.reserve_exact(self.len());

        for y in 0..self.height {
            for x in 0..self.width {
                after.push(self.apply_rules_at(x, y).unwrap());
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

    let part1_result = part1(&seat_grid);

    let part1_occupied_seats = part1_result.seats.iter()
        .filter(|&seat| *seat == SeatType::Occupied)
        .count();

    println!("part1_occupied_seats={0:?}", part1_occupied_seats);
}

fn part1(seat_grid: &SeatGrid) -> SeatGrid {
    let mut previous_grid: SeatGrid = seat_grid.clone();

    let mut _rounds = 0;

    'mutate: loop {
        _rounds += 1;

        let next_grid = previous_grid.apply_rules();

        if previous_grid.seats == next_grid.seats {
            break 'mutate;
        }

        previous_grid = next_grid;
    }

    println!("part1 finished after {0} rounds of shuffling", _rounds);

    previous_grid
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
    macro_rules! seat_grid_apply_rules_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;

            let input = input.into_iter().map(|str| String::from(str)).collect::<Vec<String>>();
            let expected = expected.into_iter().map(|str| String::from(str)).collect::<Vec<String>>();

            let seat_grid = SeatGrid::from(&input);
            let result = seat_grid.apply_rules();

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

        let next_seat = seat_grid.apply_rules_at(1, 1);

        assert_eq!(next_seat, Some(SeatType::Occupied));
    }

    #[test]
    fn seat_grid_apply_rules_returns_empty_for_at_least_4_occupied_neighbors() {
        let lines = vec!["#.#", ".#.", "#.#"]
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        let seat_grid = SeatGrid::from(&lines);

        let next_seat = seat_grid.apply_rules_at(1, 1);

        assert_eq!(next_seat, Some(SeatType::Empty));
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
        let part1_result = part1(&seat_grid);

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

}
