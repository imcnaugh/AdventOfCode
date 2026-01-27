use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_2(input: &str) -> usize {
    let (mut replacements, polymer) = parse_input_part_2(input);
    replacements.sort_by(|a, b| {
        let i = b.0.len().cmp(&a.0.len());
        if i == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            i
        }
    });

    reduce(polymer, &replacements)
}

fn reduce(polymer: String, replacements: &Vec<(String, String)>) -> usize {
    if polymer == "e" {
        return 0;
    }

    replacements
        .iter()
        .map(|(a, b)| {
            polymer
                .match_indices(a)
                .map(|(i, _)| {
                    let reduced_string =
                        format!("{}{}{}", &polymer[..i], b, &polymer[i + a.len()..]);
                    reduce(reduced_string, replacements).saturating_add(1)
                })
                .min()
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap()
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

fn parse_input_part_2(input: &str) -> (Vec<(String, String)>, String) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let vec = parts[0]
        .lines()
        .map(|line| {
            let replacements = line.split(" => ").collect::<Vec<_>>();
            (replacements[1].to_string(), replacements[0].to_string())
        })
        .collect::<Vec<_>>();
    (vec, parts[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOH";
        assert_eq!(part_2(&input), 3);
    }
}
