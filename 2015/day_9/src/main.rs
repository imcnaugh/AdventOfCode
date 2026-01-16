use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("resource/input.txt").expect("Failed to read input.txt");
    println!("{}", part_1(input.clone()));
    println!("{}", part_2(input));
}

fn part_1(input: String) -> usize {
    let mut cities: HashSet<String> = HashSet::new();
    let mut routes_from_city: HashMap<String, HashMap<String, usize>> = HashMap::new();
    input.lines().for_each(|line| {
        let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        routes_from_city
            .entry(caps[1].to_string())
            .or_insert_with(HashMap::new)
            .insert(caps[2].to_string(), caps[3].parse().unwrap());
        routes_from_city
            .entry(caps[2].to_string())
            .or_insert_with(HashMap::new)
            .insert(caps[1].to_string(), caps[3].parse().unwrap());
        cities.insert(caps[1].to_string());
        cities.insert(caps[2].to_string());
    });
    cities
        .iter()
        .map(|city| {
            dfs(
                cities.clone().into_iter().filter(|c| c != city).collect(),
                &routes_from_city,
                0,
                city,
            )
        })
        .min()
        .unwrap()
}

fn dfs(
    unvisited_cities: HashSet<String>,
    routes_from_city: &HashMap<String, HashMap<String, usize>>,
    current_distance: usize,
    current_city: &str,
) -> usize {
    if unvisited_cities.is_empty() {
        return current_distance;
    }

    unvisited_cities
        .iter()
        .map(|next_city| {
            let new_unvisited_cities = unvisited_cities
                .clone()
                .into_iter()
                .filter(|city| city != next_city)
                .collect();

            match routes_from_city.get(current_city) {
                Some(routes) => match routes.get(next_city) {
                    None => usize::MAX,
                    Some(distance) => dfs(
                        new_unvisited_cities,
                        routes_from_city,
                        current_distance + distance,
                        next_city,
                    ),
                },
                None => usize::MAX,
            }
        })
        .min()
        .unwrap()
}

fn part_2(input: String) -> usize {
    let mut cities: HashSet<String> = HashSet::new();
    let mut routes_from_city: HashMap<String, HashMap<String, usize>> = HashMap::new();
    input.lines().for_each(|line| {
        let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        routes_from_city
            .entry(caps[1].to_string())
            .or_insert_with(HashMap::new)
            .insert(caps[2].to_string(), caps[3].parse().unwrap());
        routes_from_city
            .entry(caps[2].to_string())
            .or_insert_with(HashMap::new)
            .insert(caps[1].to_string(), caps[3].parse().unwrap());
        cities.insert(caps[1].to_string());
        cities.insert(caps[2].to_string());
    });
    cities
        .iter()
        .map(|city| {
            dfs_max(
                cities.clone().into_iter().filter(|c| c != city).collect(),
                &routes_from_city,
                0,
                city,
            )
        })
        .max()
        .unwrap()
}
fn dfs_max(
    unvisited_cities: HashSet<String>,
    routes_from_city: &HashMap<String, HashMap<String, usize>>,
    current_distance: usize,
    current_city: &str,
) -> usize {
    if unvisited_cities.is_empty() {
        return current_distance;
    }

    unvisited_cities
        .iter()
        .map(|next_city| {
            let new_unvisited_cities = unvisited_cities
                .clone()
                .into_iter()
                .filter(|city| city != next_city)
                .collect();

            match routes_from_city.get(current_city) {
                Some(routes) => match routes.get(next_city) {
                    None => usize::MAX,
                    Some(distance) => dfs_max(
                        new_unvisited_cities,
                        routes_from_city,
                        current_distance + distance,
                        next_city,
                    ),
                },
                None => usize::MAX,
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
            .to_string();
        assert_eq!(part_1(input), 605);
    }

    #[test]
    fn test_part_2() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
            .to_string();
        assert_eq!(part_2(input), 982);
    }
}
