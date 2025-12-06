use std::cmp::PartialEq;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> i64 {
    let count = input.lines().count();
    let mut aggs: Vec<Agg> = Vec::new();

    input.lines().enumerate().for_each(|(i, line)| {
        let is_last = i == count - 1;
        if is_last {
            let chars = line.split_whitespace().enumerate().for_each(|(i, s)| {
                let op = match s {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => panic!("Invalid op"),
                };
                aggs.get_mut(i).unwrap().op = op;
            });
        } else {
            let nums = line.split_whitespace().enumerate().for_each(|(i, s)| {
                let num = s.parse::<i64>().unwrap();
                if let Some(agg) = aggs.get_mut(i) {
                    agg.add_numberr(num);
                } else {
                    aggs.push(Agg {
                        nums: vec![num],
                        op: Op::Add,
                    });
                }
            });
        }
    });

    aggs.iter()
        .map(|agg| {
            let total = agg.total();
            total
        })
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut aggs: Vec<Agg2> = Vec::new();
    let mut itt = input.lines().rev();
    let mut indexes: Vec<usize> = Vec::new();

    let first_line = itt.next().unwrap();
    first_line.chars().enumerate().for_each(|(i, c)| match c {
        '+' => {
            indexes.push(i);
            aggs.push(Agg2 {
                nums: Vec::new(),
                op: Op::Add,
            });
        }
        '*' => {
            indexes.push(i);
            aggs.push(Agg2 {
                nums: Vec::new(),
                op: Op::Mul,
            });
        }
        _ => {}
    });

    while let Some(line) = itt.next() {
        let mut index_itt = indexes.iter();
        let mut previous_index = index_itt.next().unwrap().clone();
        let line_len = line.len();
        for agg in aggs.iter_mut() {
            let current_index = if let Some(next_index) = index_itt.next() {
                next_index - 1
            } else {
                line_len
            };
            let s = line[(previous_index)..(current_index)].to_string();
            agg.add(s);
            previous_index = current_index + 1;
        }
    }

    aggs.iter()
        .map(|agg| {
            let total = agg.total_part_2();
            total
        })
        .sum()
}

struct Agg {
    nums: Vec<i64>,
    op: Op,
}

struct Agg2 {
    nums: Vec<String>,
    op: Op,
}

impl Agg2 {
    fn add(&mut self, num: String) {
        self.nums.push(num);
    }

    fn total_part_2(&self) -> i64 {
        let max = self.nums.iter().map(|n| n.chars().count()).max().unwrap();

        let mut new_vec: Vec<String> = vec![String::new(); max];

        self.nums.iter().for_each(|num| {
            let num_as_str = num.to_string();
            let num_as_str = format!("{:width$}", num_as_str, width = max);
            num_as_str.chars().rev().enumerate().for_each(|(i, c)| {
                if c != ' ' {
                    new_vec[i] = format!("{}{}", c, new_vec.get(i).unwrap_or(&String::new()));
                }
            })
        });

        let total = new_vec
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .fold(1, |acc, num| match self.op {
                Op::Add => acc + num,
                Op::Mul => acc * num,
            });

        if self.op == Op::Add { total - 1 } else { total }
    }
}

impl Agg {
    fn add_numberr(&mut self, num: i64) {
        self.nums.push(num);
    }

    fn total(&self) -> i64 {
        let total = self.nums.iter().fold(1, |acc, num| match self.op {
            Op::Add => acc + num,
            Op::Mul => acc * num,
        });
        if self.op == Op::Add { total - 1 } else { total }
    }

    fn total_part_2(&self) -> i64 {
        let max = self.nums.iter().max().unwrap();
        let max_as_str = max.to_string();
        let max_chars = max_as_str.chars().count();

        let mut new_vec: Vec<String> = vec![String::new(); max_chars];

        self.nums.iter().for_each(|num| {
            let num_as_str = num.to_string();
            num_as_str.chars().rev().enumerate().for_each(|(i, c)| {
                new_vec[i] = format!("{}{}", new_vec.get(i).unwrap_or(&String::new()), c);
            })
        });

        let total =
            new_vec
                .iter()
                .map(|s| s.parse::<i64>().unwrap())
                .fold(1, |acc, num| match self.op {
                    Op::Add => acc + num,
                    Op::Mul => acc * num,
                });

        if self.op == Op::Add { total - 1 } else { total }
    }
}

#[derive(PartialEq)]
enum Op {
    Add,
    Mul,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 3263827);
    }
}
