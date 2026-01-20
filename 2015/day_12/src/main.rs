use regex::Regex;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"(-?\d+)").unwrap();
    let captures = regex.captures_iter(input);
    captures.fold(0, |acc, cap| acc + cap[1].parse::<i32>().unwrap_or(0))
}

fn part_2(input: &str) -> i32 {
    
}