use std::collections::HashSet;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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

fn part_2(input: &str) -> usize {
    let machines = input
        .lines()
        .map(|line| Machine::from(line))
        .collect::<Vec<Machine>>();

    machines
        .iter()
        .enumerate()
        .map(|(index, machine)| {
            let current_joltages = vec![0; machine.joltages.len()];
            let mut current_joltages_set = HashSet::new();
            current_joltages_set.insert(current_joltages);
            println!("working on machine {index}");
            part_2_bfs(machine, current_joltages_set.clone(), 0)
        })
        .sum()
}

fn part_2_bfs(machine: &Machine, current_joltages: HashSet<Vec<u16>>, depth: usize) -> usize {
    if depth > machine.joltage_sum {
        return 0;
    }

    let options = &machine.buttons;
    let mut next_joltages: HashSet<Vec<u16>> = HashSet::new();

    for joltage in current_joltages {
        for option in options {
            let new_joltages = press_button_for_joltages(&joltage, &option);
            if new_joltages
                .iter()
                .zip(machine.joltages.iter())
                .all(|(a, b)| a == b)
            {
                return depth + 1;
            }

            if machine
                .joltages
                .iter()
                .zip(&new_joltages)
                .all(|(m_j, n_j)| *n_j <= *m_j)
            {
                next_joltages.insert(new_joltages.clone());
            }
        }
    }

    part_2_bfs(machine, next_joltages, depth + 1)
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

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 33);
    }

    #[test]
    fn test_part_2_bfs() {
        let machine = Machine::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

        let current_joltages = vec![0; machine.joltages.len()];
        let mut current_joltages_set = HashSet::new();
        current_joltages_set.insert(current_joltages);

        assert_eq!(part_2_bfs(&machine, current_joltages_set, 0), 10);
    }
}
