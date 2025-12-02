fn main() {
    let input = include_str!("../resource/input.txt");
    let result = part_2(&input);
    println!("Part 2: {}", result);
}

fn part_1(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| Instruction::build(line))
        .collect::<Vec<Instruction>>();
    let max = 100;
    let min = 0;

    let mut current = 50;
    let mut total_zero = 0;

    instructions.iter().for_each(|instruction| {
        current = match instruction.direction {
            Direction::Left => current - instruction.distance,
            Direction::Right => current + instruction.distance,
        };

        current %= max;
        if current < min {
            current += max;
        }

        if current == 0 {
            total_zero += 1;
        }
    });

    format!("{total_zero}")
}

fn part_2(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| Instruction::build(line))
        .collect::<Vec<Instruction>>();
    let max = 100;
    let min = 0;

    let mut current = 50;
    let mut total_zero = 0;

    instructions.iter().for_each(|instruction| {
        let total_spins = instruction.distance / max;
        total_zero += total_spins;

        let simplified_distance = instruction.distance % max;

        current = match instruction.direction {
            Direction::Left => {
                let tmp = current - simplified_distance;
                if tmp < min {
                    if current != min {
                        total_zero += 1;
                    }
                    tmp + max
                } else {
                    tmp
                }
            }
            Direction::Right => {
                let tmp = current + simplified_distance;
                if tmp > max {
                    total_zero += 1;
                }
                tmp % max
            }
        };

        if current == min {
            total_zero += 1;
        }
    });

    format!("{total_zero}")
}

enum Direction {
    Left,
    Right,
}

struct Instruction {
    pub direction: Direction,
    pub distance: i32,
}

impl Instruction {
    fn build(input: &str) -> Self {
        let direction = &input[0..1];
        let distance = &input[1..];
        Self {
            direction: match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            distance: distance.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        let result = part_1(&input);
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test.txt");
        let result = part_2(&input);
        assert_eq!(result, "6");
    }
}
