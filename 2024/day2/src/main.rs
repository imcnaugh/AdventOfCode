use std::fs;
use crate::Type::{Decreasing, Increasing};

fn main() {
    let result = part_1("resource/input.txt");
    println!("result: {result}");
}

enum Type {
    Increasing,
    Decreasing,
}

impl Type {
    fn get_diff(&self, a: usize, b: usize) -> usize {
        match self {
            Increasing => {
                if a > b { return 100 }
                b - a
            },
            Decreasing => {
                if b > a { return 100; }
                a - b
            }
        }
    }

    fn is_correct_order(&self, a: usize, b: usize) -> bool {
        match self {
            Increasing => b > a,
            Decreasing => a > b
        }
    }
}

fn part_1(input_file_path: &str) -> usize {
    let data = fs::read_to_string(input_file_path).unwrap();

    let mut result = 0;

    let mut rows = data.split("\n");
    while let Some(row) = rows.next() {
        let r = check_row(row);
        result += r;
    }

    result
}

fn check_row(row: &str) -> usize {
    let mut elements = row.split(" ");
    let first: usize = elements.next().unwrap().parse().unwrap();
    let second: usize = elements.next().unwrap().parse().unwrap();

    let direction = if first < second {
        Increasing
    } else {
        Decreasing
    };

    if !direction.is_correct_order(first, second) {
        return 0;
    }

    let diff = direction.get_diff(first, second);
    if diff > 3 {
        return 0;
    }

    let mut previous = second;
    while let Some(n) = elements.next() {
        let n: usize = n.parse().unwrap();
        if !direction.is_correct_order(previous, n) {
            return 0;
        }
        let diff = direction.get_diff(previous, n);
        if diff > 3 {
            return 0;
        }
        previous = n;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        let result = part_1("resource/test.txt");
        assert_eq!(2, result);
    }
}