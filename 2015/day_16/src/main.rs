use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", search(part_1_match));
    println!("Part 2: {}", search(part_2_match));
}

fn search(search_fn: fn(&HashMap<String, usize>, &HashMap<String, usize>) -> bool) -> usize {
    let input = include_str!("../resources/input.txt");
    let expected = get_expected();
    input
        .lines()
        .map(parse_line)
        .find(|(_, properties)| search_fn(&expected, properties))
        .unwrap()
        .0
}

fn part_1_match(expected: &HashMap<String, usize>, properties: &HashMap<String, usize>) -> bool {
    properties
        .iter()
        .all(|(key, value)| expected.get(key) == Some(value))
}

fn part_2_match(expected: &HashMap<String, usize>, properties: &HashMap<String, usize>) -> bool {
    properties.iter().all(|(key, value)| match key.as_str() {
        "cats" | "trees" => *value > expected[key],
        "pomeranians" | "goldfish" => *value < expected[key],
        _ => expected.get(key) == Some(value),
    })
}

fn parse_line(line: &str) -> (usize, HashMap<String, usize>) {
    let rx = Regex::new(r#"Sue (\d+): (.*)"#).unwrap();
    let captures = rx.captures(line).unwrap();
    let id = captures[1].parse::<usize>().unwrap();
    let properties = captures[2]
        .split(", ")
        .map(parse_list_of_items)
        .collect::<HashMap<String, usize>>();
    (id, properties)
}

fn get_expected() -> HashMap<String, usize> {
    include_str!("../resources/expected.txt")
        .lines()
        .map(parse_list_of_items)
        .collect::<HashMap<String, usize>>()
}

fn parse_list_of_items(line: &str) -> (String, usize) {
    let mut parts = line.split(": ");
    let (key, value) = (parts.next().unwrap().to_lowercase(), parts.next().unwrap());
    (key.to_string(), value.parse::<usize>().unwrap())
}
