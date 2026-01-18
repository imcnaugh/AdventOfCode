use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let matcher = Regex::new(r"(.) (\d+)").unwrap();
    let mut max_height = 0;
    let mut min_height = 0;
    let mut max_left = 0;
    let mut max_right = 0;

    let mut trench = input
        .split('\n')
        .map(|line| {
            let captures = matcher.captures(line).unwrap();
            let dir = captures.get(1).unwrap().as_str();
            let steps = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (dir, steps)
        })
        .fold(
            (HashSet::<(i32, i32)>::new(), (0, 0)),
            |mut acc, (dir, steps)| {
                let mut current = acc.1;
                let step_math = match dir {
                    "U" => (0, 1),
                    "D" => (-0, -1),
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    _ => (0, 0),
                };
                (0..steps).for_each(|_| {
                    current = (current.0 + step_math.0, current.1 + step_math.1);
                    acc.0.insert(current);
                });

                if dir == "U" && current.1 > max_height {
                    max_height = current.1;
                }
                if dir == "D" && current.1 < min_height {
                    min_height = current.1;
                }
                if dir == "L" && current.0 < max_left {
                    max_left = current.0;
                }
                if dir == "R" && current.0 > max_right {
                    max_right = current.0;
                }

                (acc.0, current)
            },
        )
        .0;

    flood_fill(&mut trench, (0, 0));

    trench.len()
}

fn flood_fill(trench: &mut HashSet<(i32, i32)>, current: (i32, i32)) {
    trench.insert(current);
    if !trench.contains(&(current.0 - 1, current.1)) {
        flood_fill(trench, (current.0 - 1, current.1));
    }
    if !trench.contains(&(current.0 + 1, current.1)) {
        flood_fill(trench, (current.0 + 1, current.1));
    }
    if !trench.contains(&(current.0, current.1 - 1)) {
        flood_fill(trench, (current.0, current.1 - 1));
    }
    if !trench.contains(&(current.0, current.1 + 1)) {
        flood_fill(trench, (current.0, current.1 + 1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(part_1(input), 62);
    }
}
