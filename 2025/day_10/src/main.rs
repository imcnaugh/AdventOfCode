use std::collections::HashSet;
use arrayvec::ArrayVec;
use microlp::{LinearExpr, OptimizationDirection, Problem};

fn main() {
    let input = include_str!("../resource/input.txt");
    // println!("Part 1: {}", part_1(&input));
    part_2();
}

fn part_1(input: &str) -> usize {
    let machines = input
        .lines()
        .map(|line| Machine::from(line))
        .collect::<Vec<Machine>>();

    machines
        .iter()
        .map(|machine| {
            let current_lights = vec![false; machine.lights.len()];
            let mut current_lights_set = HashSet::new();
            current_lights_set.insert(current_lights);
            part_1_bfs(machine, current_lights_set.clone(), current_lights_set)
        })
        .sum()
}

fn part_1_bfs(
    machine: &Machine,
    current_lights: HashSet<Vec<bool>>,
    previously_seen_lights: HashSet<Vec<bool>>,
) -> usize {
    let options = &machine.buttons;
    let mut next_lights: HashSet<Vec<bool>> = HashSet::new();
    let mut next_seen_lights: HashSet<Vec<bool>> = HashSet::new();

    for light in current_lights {
        for button in options {
            let new_lights = press_button_for_lights(&light, &button);
            if new_lights == machine.lights {
                return 1;
            }
            if !previously_seen_lights.contains(&new_lights) {
                next_lights.insert(new_lights.clone());
                next_seen_lights.insert(new_lights);
            }
        }
    }

    part_1_bfs(machine, next_lights, next_seen_lights) + 1
}

fn press_button_for_lights(current_lights: &Vec<bool>, button: &Vec<u8>) -> Vec<bool> {
    let mut new_lights = current_lights.clone();
    for p in button {
        new_lights[*p as usize] = !new_lights[*p as usize];
    }
    new_lights
}

const BTNS_SIZE: usize = 13;
const JOLT_SIZE: usize = 10;

fn part_2() {
    let presses = include_bytes!("../resource/input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let (first, last) = line.split_at(line.iter().position(|&b| b == b'{').unwrap());

            let btns = first[1..]
                .split(|&b| b == b' ')
                .skip(1)
                .filter(|btns| !btns.is_empty())
                .map(|btns| {
                    btns[1..]
                        .split(|&b| b == b',')
                        .map(|n| 1 << (n[0] - b'0'))
                        .sum()
                })
                .collect::<ArrayVec<u16, BTNS_SIZE>>();
            let jolts = last[1..]
                .split(|&b| b == b',')
                .map(|b| atoi::atoi::<u16>(b).unwrap())
                .collect::<ArrayVec<u16, JOLT_SIZE>>();

            let mut problem = Problem::new(OptimizationDirection::Minimize);
            let max = jolts.iter().copied().max().unwrap();
            let vars = (0..btns.len())
                .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
                .collect::<ArrayVec<_, BTNS_SIZE>>();
            for (i, &n) in jolts.iter().enumerate() {
                problem.add_constraint(
                    btns.iter()
                        .zip(&vars)
                        .filter(|&(mask, _)| mask & (1 << i) != 0)
                        .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                            ex.add(var, 1.0);
                            ex
                        }),
                    microlp::ComparisonOp::Eq,
                    n as f64,
                );
            }
            problem.solve().unwrap().objective().round() as usize
        })
        .sum::<usize>();

    println!("{presses}");
}

fn press_button_for_joltages(current_joltages: &Vec<u16>, button: &Vec<u8>) -> Vec<u16> {
    let mut new_joltages = current_joltages.clone();
    for p in button {
        new_joltages[*p as usize] += 1;
    }
    new_joltages
}

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u8>>,
    joltages: Vec<u16>,

    joltage_sum: usize,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let lights_str_end_index = value.find(']').unwrap();

        let lights = (1..lights_str_end_index)
            .map(|i| match value.chars().nth(i).unwrap() {
                '#' => true,
                _ => false,
            })
            .collect();

        let joltage_str_start_index = value.find('{').unwrap();
        let joltage_str = value[joltage_str_start_index..value.len()].to_string();

        let joltages: Vec<u16> = value[joltage_str_start_index + 1..value.len() - 1]
            .split(',')
            .map(|s| s.parse::<u16>().unwrap())
            .collect();

        let buttons = value[lights_str_end_index + 2..joltage_str_start_index - 1]
            .split(' ')
            .map(|button_string| {
                let real_str = &button_string[1..button_string.len() - 1];
                real_str
                    .split(',')
                    .map(|p| p.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let joltage_sum = joltages.iter().map(|&j| j as usize).sum();

        Self {
            lights,
            buttons,
            joltages,
            joltage_sum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 7);
    }

    #[test]
    fn test_part_1_bfs() {
        let machine = Machine::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        let current_lights = vec![false; machine.lights.len()];
        let mut current_lights_set = HashSet::new();
        current_lights_set.insert(current_lights);
        assert_eq!(
            part_1_bfs(&machine, current_lights_set.clone(), current_lights_set),
            3
        );
    }

    #[test]
    fn test_parse() {
        let machine = Machine::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let expected_lights = vec![false, true, true, false];
        let expected_joltages = vec![3, 5, 4, 7];
        let expected_buttons = vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ];
        assert_eq!(machine.lights.len(), 4);
        assert_eq!(machine.lights, expected_lights);

        assert_eq!(machine.buttons.len(), 6);
        assert_eq!(machine.buttons, expected_buttons);

        assert_eq!(machine.joltages.len(), 4);
        assert_eq!(machine.joltages, expected_joltages);
    }
}
