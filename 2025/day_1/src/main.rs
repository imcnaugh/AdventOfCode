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
        match instruction.direction {
            Direction::Left => {
                let next = current - instruction.distance;
                if next < min {
                    current = max + next;
                } else {
                    current = next;
                }
            }
            Direction::Right => {
                let next = current + instruction.distance;
                if next >= max {
                    current = (max + next) % max;
                } else {
                    current = next;
                }
            }
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
        .map(|line| Instruction::build_part_2(line))
        .collect::<Vec<Instruction>>();
    let max = 100;
    let min = 0;

    let mut current = 50;
    let mut total_zero = 0;

    instructions.iter().for_each(|instruction| {
        let total_spins = instruction.distance / max;
        let distance = instruction.distance % max;
        total_zero += total_spins;

        match instruction.direction {
            Direction::Left => {
                let mut next = current - distance;
                if next < min {
                    if current != 0 {
                        total_zero += 1;
                    }
                    next = max + next;
                }
                current = next;
            }
            Direction::Right => {
                let next = current + distance;
                if next > max {
                    total_zero += 1;
                }
                let next = next % max;
                current = next;
            }
        }
        if current == 0 {
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
        let i = distance.len() as i32 - 2;
        let i = if i <= 0 { 0 } else { i };
        let distance = &distance[i as usize..];
        Self {
            direction: match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            distance: distance.parse().unwrap(),
        }
    }

    fn build_part_2(input: &str) -> Self {
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
