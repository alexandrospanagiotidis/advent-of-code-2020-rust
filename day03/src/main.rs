use std::io::{BufRead, stdin};

use crate::Thing::{OpenSquare, Tree, Unknown};

#[derive(Debug)]
enum Thing {
    OpenSquare,
    Tree,
    Unknown,
}

type Map = Vec<Vec<Thing>>;

impl Thing {
    fn from(line: &String) -> Vec<Thing> {
        line.to_uppercase()
            .chars()
            .into_iter()
            .map(|character| match character {
                '.' => OpenSquare,
                '#' => Tree,
                _ => Unknown
            })
            .collect()
    }
}

fn main() {
    let map: Map = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .map(|row_string| Thing::from(&row_string))
        .collect();

    // println!("{0:?}", map);

    println!("part 1: encountered trees = {0}", part1(&map, 3, 1));
}

fn part1(map: &Map, dx: usize, dy: usize) -> u32 {
    let mut column: usize = 0;
    let mut row: usize = 0;
    let mut tree_count: u32 = 0;

    let width = map.first().unwrap().len();
    let height = map.len();

    // println!("grid dim = {0}x{1}", width, height);

    loop {
        if row >= height {
            break;
        }

        let current_row = &map[row];
        let current_thing = &current_row[column];

        match current_thing {
            Tree => tree_count = tree_count + 1,
            _ => ()
        }

        // Wrap around due to "arboreal genetics and biome stability"
        column = (column + dx) % width;
        row = row + dy;
    }

    tree_count as u32
}
