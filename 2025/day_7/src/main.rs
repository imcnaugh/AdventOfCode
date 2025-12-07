use std::collections::{HashMap, HashSet};
use std::str::Lines;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    find_splits(&mut lines, HashSet::from([first_line.find('S').unwrap()]))
}

fn find_splits(lines_iter: &mut Lines, incoming_beams: HashSet<usize>) -> usize {
    if let Some(line) = lines_iter.next() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut next_beams = HashSet::new();
        let mut splits = 0;

        for i in incoming_beams {
            if chars[i] == '^' {
                splits += 1;
                next_beams.insert(i - 1);
                next_beams.insert(i + 1);
            } else {
                next_beams.insert(i);
            }
        }
        splits + find_splits(lines_iter, next_beams)
    } else {
        0
    }
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    find_all_paths(
        &mut lines,
        HashMap::from([(first_line.find('S').unwrap(), 1)]),
    )
}

fn find_all_paths(lines_iter: &mut Lines, incoming_beams: HashMap<usize, usize>) -> usize {
    if let Some(line) = lines_iter.next() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut next_beams = HashMap::new();
        for (k, v) in incoming_beams {
            if chars[k] == '^' {
                *next_beams.entry(k - 1).or_insert(0) += v;
                *next_beams.entry(k + 1).or_insert(0) += v;
            } else {
                *next_beams.entry(k).or_insert(0) += v;
            }
        }
        find_all_paths(lines_iter, next_beams)
    } else {
        incoming_beams.values().sum::<usize>()
    }
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
