use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let (map, polymer) = parse_input(input);

    let mut possible: HashSet<String> = HashSet::new();

    (0..polymer.len()).for_each(|i| {
        let prefix = &polymer[..i];
        let postfix = &polymer[i..].to_string();
        map.keys().for_each(|k| {
            let replacement = map.get(k).unwrap();
            let x = postfix.clone();

            if postfix.starts_with(k) {
                let x = &x[k.len()..];
                replacement.iter().for_each(|r| {
                    let new = format!("{}{}{}", prefix, r, x);
                    possible.insert(new);
                })
            }
        })
    });

    possible.len()
}

fn parse_input(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let map: HashMap<String, Vec<String>> =
        parts[0].lines().fold(HashMap::new(), |mut acc, line| {
            let replacements = line.split(" => ").collect::<Vec<_>>();
            let a = replacements[0];
            let b = replacements[1];
            acc.entry(a.to_string())
                .or_insert(vec![])
                .push(b.to_string());
            acc
        });
    (map, parts[1].to_string())
}
