use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let start = input.find('S').unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let starter_set = HashSet::from([start]);
    split(lines, 1, starter_set)
}

fn part_2(input: &str) -> usize {
    let start = input.find('S').unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut starter_set: HashMap<usize, usize> = HashMap::new();
    starter_set.insert(start, 1);
    split_part_2(lines, 1, starter_set)
}

fn split(input: Vec<&str>, current_line_index: usize, incoming: HashSet<usize>) -> usize {
    if current_line_index == input.len() {
        return 0;
    }
    let mut next_incoming = HashSet::new();
    let current_line = input[current_line_index].chars().collect::<Vec<char>>();
    let mut total_splits = 0;
    for i in incoming {
        if current_line[i] == '^' {
            next_incoming.insert(i - 1);
            next_incoming.insert(i + 1);
            total_splits += 1;
        } else {
            next_incoming.insert(i);
        }
    }
    split(input, current_line_index + 1, next_incoming) + total_splits
}

fn split_part_2(
    input: Vec<&str>,
    current_line_index: usize,
    incoming: HashMap<usize, usize>,
) -> usize {
    if current_line_index == input.len() {
        return incoming.values().sum::<usize>();
    }
    let mut next_incoming = HashMap::new();
    let current_line = input[current_line_index].chars().collect::<Vec<char>>();
    for (k, v) in incoming {
        if current_line[k] == '^' {
            let left = k - 1;
            let right = k + 1;
            if let Some(left_value) = next_incoming.get_mut(&left) {
                *left_value += v;
            } else {
                next_incoming.insert(left, v);
            }

            if let Some(right_value) = next_incoming.get_mut(&right) {
                *right_value += v;
            } else {
                next_incoming.insert(right, v);
            }
        } else {
            if let Some(value) = next_incoming.get_mut(&k) {
                *value += v;
            } else {
                next_incoming.insert(k, v);
            }
        }
    }
    split_part_2(input, current_line_index + 1, next_incoming)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 40);
    }
}
