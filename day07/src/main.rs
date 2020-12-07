use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::iter::FromIterator;
use std::ops::{Add, Deref};

use regex::Regex;

type Color = String;
type Contents = HashSet<Content>;
type Bags = HashMap<Color, Contents>;

#[derive(Debug, Eq, Hash, Clone)]
struct Content {
    color: Color,
    count: usize,
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.count == other.count
    }
}

fn main() {
    let bag_rules: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let bag_contents_re = Regex::new(r"([0-9]+) ([a-zA-z ]+) bags?[.,]?")
        .expect("Could not compile regex");

    let bags: Bags = bag_rules.into_iter()
        .map(|line| split_rule(&line, &bag_contents_re))
        .collect();

    println!("bags = {0:?}", bags);

    let shiny_gold_name = Color::from("shiny gold");

    let mut reverse_bags: HashMap<Color, HashSet<Color>> = HashMap::new();

    for (color, contents) in bags.clone().into_iter() {
        contents.into_iter()
            .for_each(|c| {
                (*reverse_bags.entry(c.color).or_insert(HashSet::new())).insert(color.to_string());
            });
    }

    // All bags that can contain "shiny gold" directly
    let mut can_contain: HashSet<Color> = reverse_bags.get(&shiny_gold_name).unwrap().clone();

    // A deque that holds bags that contain bags that contain "shiny gold" in some way
    let mut might_contain: VecDeque<Color> = VecDeque::from_iter(can_contain.clone());

    while let Some(color) = might_contain.pop_front() {
        if let Some(contents) = reverse_bags.get(&color) {
            // println!("can_contain = {0:?}", can_contain);
            // println!("might_contain = {0:?}", might_contain);
            // println!("trying {0} = {1:?}", color, contents);

            for x in contents {
                if !can_contain.contains(x) {
                    might_contain.push_back(x.clone());
                    can_contain.insert(x.clone());
                }
            }
        }
    }

    println!("can_contain = {0} -> {1:?}", can_contain.len(), can_contain);

    let mut sum_bags = count_bags(&bags, &shiny_gold_name);

    println!("sum_bags = {0}", sum_bags);
}

fn split_rule(line: &str, re: &Regex) -> (Color, Contents) {
    let mut tokens = line.split(" bags contain ");

    (
        String::from(tokens.next()
            .expect(String::from("Could not extract bag color from: ").add(&line).as_str())
        ),
        tokens.next()
            .map(|contents| split_contents(&contents, &re))
            .expect(String::from("Could not extract bag contents from: ").add(&line).as_str()),
    )
}

fn split_contents(line: &str, re: &Regex) -> Contents {
    re.captures_iter(line.trim())
        // .map(|capture| {
        //     println!("captured = {0:?}", capture);
        //     capture
        // })
        .map(|capture| (capture.get(1).unwrap(), capture.get(2).unwrap()))
        .map(|(capacity, color)| Content {
            color: color.as_str().to_string(),
            count: capacity.as_str().parse::<usize>().unwrap(),
        })
        .collect()
}

fn count_bags(bags: &Bags, color: &String) -> usize {
    let mut sum = 0;

    for inner_bag in bags.get(color).unwrap() {
        sum += inner_bag.count + inner_bag.count * count_bags(bags, &inner_bag.color);
    }

    sum
}
