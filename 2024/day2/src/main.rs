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
    'outter: while let Some(initial_row) = rows.next() {
        let initial_row: Vec<usize> = initial_row.split(" ").map(|e| e.parse().unwrap()).collect();
        let r = check_row(&initial_row);
        if r == 0 {
            let idk: Vec<usize> = initial_row.clone();
            for i in 0..idk.len() {
                let mut tmp_row = idk.clone();
                tmp_row.remove(i);

                let r = check_row(&tmp_row);
                if r == 1 {
                    result += 1;
                    continue 'outter;
                }
            }
        }
        result += r;
    }

    result
}

fn check_row(row: &Vec<usize>) -> usize {
    let row = row.clone();
    let mut elements = row.into_iter();
    let first: usize = elements.next().unwrap();
    let second: usize = elements.next().unwrap();

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
        assert_eq!(4, result);
    }
}