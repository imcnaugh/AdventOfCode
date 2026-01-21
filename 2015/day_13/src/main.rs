use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("{}", part_1(input));
}

fn part_1(input: &str) -> i64 {
    let mut people = HashSet::<String>::new();
    let mut happiness_map = HashMap::<String, HashMap<String, i64>>::new();
    input.lines().for_each(|line| {
        let rx = Regex::new(r#"(.*) would (.*) happiness units by sitting next to (.*)."#).unwrap();
        let captures = rx.captures(line).unwrap();
        let person = captures.get(1).unwrap().as_str().to_string();
        let happiness_change: Vec<&str> = captures.get(2).unwrap().as_str().split(' ').collect();
        let happiness = match happiness_change[0] {
            "gain" => happiness_change[1].parse::<i64>().unwrap(),
            "lose" => -happiness_change[1].parse::<i64>().unwrap(),
            _ => panic!("Invalid happiness change: {}", happiness_change[1]),
        };
        let other_person = captures.get(3).unwrap().as_str().to_string();
        people.insert(person.clone());
        people.insert(other_person.clone());
        happiness_map.entry(person).or_insert(HashMap::new()).insert(other_person, happiness);
    });
    seat_people(vec![], people, &happiness_map)
}

fn seat_people(current_setting: Vec<String>, to_be_seated: HashSet<String>, happyness_map: &HashMap<String, HashMap<String, i64>>) -> i64 {
    if to_be_seated.is_empty() {
        let mut total = 0;
        for i in 0..current_setting.len() {
            let persson = current_setting[i].clone();
            let next_person = current_setting[(i+1) % current_setting.len()].clone();
            let previous_person = current_setting[(i + current_setting.len() - 1) % current_setting.len()].clone();
            total += happyness_map.get(&persson).unwrap().get(&next_person).unwrap_or(&0) + happyness_map.get(&persson).unwrap().get(&previous_person).unwrap_or(&0);
        }
        return total;
    }

    to_be_seated.iter().map(|person| {
        let mut remaining = to_be_seated.clone();
        remaining.remove(person);
        let mut next_seating = current_setting.clone();
        next_seating.push(person.clone());
        seat_people(next_seating, remaining, happyness_map)
    }).max().unwrap()
}

fn part_2(input: &str) -> i64 {
    let mut people = HashSet::<String>::new();
    let mut happiness_map = HashMap::<String, HashMap<String, i64>>::new();
    input.lines().for_each(|line| {
        let rx = Regex::new(r#"(.*) would (.*) happiness units by sitting next to (.*)."#).unwrap();
        let captures = rx.captures(line).unwrap();
        let person = captures.get(1).unwrap().as_str().to_string();
        let happiness_change: Vec<&str> = captures.get(2).unwrap().as_str().split(' ').collect();
        let happiness = match happiness_change[0] {
            "gain" => happiness_change[1].parse::<i64>().unwrap(),
            "lose" => -happiness_change[1].parse::<i64>().unwrap(),
            _ => panic!("Invalid happiness change: {}", happiness_change[1]),
        };
        let other_person = captures.get(3).unwrap().as_str().to_string();
        people.insert(person.clone());
        people.insert(other_person.clone());
        happiness_map.entry(person).or_insert(HashMap::new()).insert(other_person, happiness);
    });
    seat_people(vec![], people, &happiness_map)
}
