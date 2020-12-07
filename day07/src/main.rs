use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::iter::FromIterator;
use std::ops::Add;

use regex::Regex;

type Color = String;
type Contents = HashSet<Color>;
type Bags = HashMap<Color, Contents>;

fn main() {
    let bag_rules: Vec<String> = stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect();

    let bag_contents_re = Regex::new(r"[0-9]+ ([a-zA-z ]+) bags?[.,]?")
        .expect("Could not compile regex");

    let bags: Bags = bag_rules.into_iter()
        .map(|line| split_rule(&line, &bag_contents_re))
        .collect();

    // println!("bags = {0:?}", bags);

    let shiny_gold_name = Color::from("shiny gold");

    let mut reverse_bags = Bags::new();

    for (color, contents) in bags.clone().into_iter() {
        contents.into_iter()
            .for_each(|c| {
                (*reverse_bags.entry(c).or_insert(Contents::new())).insert(color.to_string());
            });
    }

    println!("reverse_bags = {0:?}", reverse_bags);
    println!("reverse_bags = {0:?}", reverse_bags.get(&shiny_gold_name));

    // All bags that can contain "shiny gold" directly
    let mut can_contain: Contents = reverse_bags.get(&shiny_gold_name).unwrap().clone();

    // A deque that holds bags that contain bags that contain "shiny gold" in some way
    let mut might_contain: VecDeque<Color> = VecDeque::from_iter(can_contain.clone());

    let mut seen: Contents = Contents::new();

    while let Some(color) = might_contain.pop_front() {
        if let Some(contents) = reverse_bags.get(&color) {
            println!("can_contain = {0:?}", can_contain);
            println!("might_contain = {0:?}", might_contain);
            println!("trying {0} = {1:?}", color, contents);

            for x in contents {
                if !seen.contains(x) {
                    println!("{0} is new!", x);
                    // println!("boi = {0:?}", x);
                    might_contain.push_back(x.clone());
                    can_contain.insert(x.clone());
                } else {
                    println!("{0} already seen", x);
                }

                seen.insert(x.to_string());
            }
        }
    }

    println!("can_contain_gold = {0} -> {1:?}", can_contain.len(), can_contain);
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
    re.captures_iter(line)
        .map(|capture| capture.get(1)
            .expect(String::from("Could not get capture from: ").add(line).as_str()))
        .map(|x| String::from(x.as_str()))
        .collect()
}
