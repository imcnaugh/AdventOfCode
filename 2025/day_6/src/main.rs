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
        if i == count - 1 {
            line.split_whitespace().enumerate().for_each(|(i, s)| {
                let op = match s {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => panic!("Invalid op"),
                };
                aggs.get_mut(i).unwrap().op = op;
            });
        } else {
            line.split_whitespace().enumerate().for_each(|(i, s)| {
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

    aggs.iter().map(|agg| agg.total()).sum()
}

fn part_2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines[0].len();
    let count = lines.len();

    let mut lines = lines.iter().map(|&l| l.chars().rev()).collect::<Vec<_>>();

    let mut buf: Vec<i64> = Vec::new();
    let mut total = 0;
    (0..len).for_each(|_| {
        let num = (0..count - 1)
            .map(|i| lines[i].next().unwrap())
            .filter(|&c| c != ' ')
            .collect::<String>();

        if !num.is_empty() {
            let num = num.parse::<i64>().unwrap();
            buf.push(num);
        }

        match lines[count - 1].next().unwrap() {
            '+' => {
                total += buf.iter().sum::<i64>();
                buf.clear();
            }
            '*' => {
                total += buf.iter().product::<i64>();
                buf.clear();
            }
            _ => {}
        }
    });

    total
}

struct Agg {
    nums: Vec<i64>,
    op: Op,
}

impl Agg {
    fn add_numberr(&mut self, num: i64) {
        self.nums.push(num);
    }

    fn total(&self) -> i64 {
        match self.op {
            Op::Add => self.nums.iter().sum(),
            Op::Mul => self.nums.iter().product(),
        }
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
