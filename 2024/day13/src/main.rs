use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    pub a_movement: (usize, usize),
    pub b_movement: (usize, usize),
    pub prize: (usize, usize)
}

fn main() {
    let input = get_file("resource/input.txt");
    let result = part_2(input);
    println!("{result}")
}

fn get_file(path: &str) -> String{
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: String) -> Vec<Machine> {
    let parts = input.split("\n\n");
    parts.map(|machine_params| -> Machine {
        let re = Regex::new(r"Button A: X\+(?<a_x>\d+), Y\+(?<a_y>\d+)\nButton B: X\+(?<b_x>\d+), Y\+(?<b_y>\d+)\nPrize: X=(?<p_x>\d+), Y=(?<p_y>\d+)").unwrap();
        let captures = re.captures(machine_params).unwrap();

        Machine {
            a_movement: (captures["a_x"].parse::<usize>().unwrap(), captures["a_y"].parse::<usize>().unwrap()),
            b_movement: (captures["b_x"].parse::<usize>().unwrap(), captures["b_y"].parse::<usize>().unwrap()),
            prize: (captures["p_x"].parse::<usize>().unwrap(), captures["p_y"].parse::<usize>().unwrap())
        }
    }).collect()
}

fn parse_input_part_2(input: String) -> Vec<Machine> {
    let parts = input.split("\n\n");
    parts.map(|machine_params| -> Machine {
        let re = Regex::new(r"Button A: X\+(?<a_x>\d+), Y\+(?<a_y>\d+)\nButton B: X\+(?<b_x>\d+), Y\+(?<b_y>\d+)\nPrize: X=(?<p_x>\d+), Y=(?<p_y>\d+)").unwrap();
        let captures = re.captures(machine_params).unwrap();
        let p_x: usize = &captures["p_x"].parse::<usize>().unwrap() + 10000000000000;
        let p_y: usize = &captures["p_y"].parse::<usize>().unwrap() + 10000000000000;

        Machine {
            a_movement: (captures["a_x"].parse::<usize>().unwrap(), captures["a_y"].parse::<usize>().unwrap()),
            b_movement: (captures["b_x"].parse::<usize>().unwrap(), captures["b_y"].parse::<usize>().unwrap()),
            prize: (p_x, p_y)
        }
    }).collect()
}

fn part_1(input: String) -> usize {
    parse_input(input).iter().map(|m| get_minimum_to_prize_part_2(m)).sum()
}

fn part_2(input: String) -> usize {
    parse_input_part_2(input).iter().map(|m| get_minimum_to_prize_part_2(m)).sum()
}


fn get_minimum_to_prize_part_2(machine: &Machine) -> usize {
    let a_slope = machine.a_movement.1 as f64 / machine.a_movement.0 as f64;
    let a_y_intercept = machine.prize.1 as f64 - (a_slope * machine.prize.0 as f64);

    let b_slope = machine.b_movement.1 as f64 / machine.b_movement.0 as f64;

    let prize_slope = machine.prize.1 as f64 / machine.prize.0 as f64;

    if prize_slope == a_slope {
        return (machine.prize.0 / machine.a_movement.0) * 3;
    }
    if prize_slope == b_slope {
        return machine.prize.0 / machine.b_movement.0;
    }

    let x_intercept = a_y_intercept / (b_slope - a_slope);
    let b_count = (x_intercept / machine.b_movement.0 as f64) as usize;
    let a_count = ((machine.prize.0 as f64 - x_intercept) / machine.a_movement.0 as f64) as usize;

    let x = (machine.a_movement.0 * a_count) + (machine.b_movement.0 * b_count);
    let y = (machine.a_movement.1 * a_count) + (machine.b_movement.1 * b_count);

    if machine.prize.0 == x && machine.prize.1 == y {
        (a_count * 3) + b_count
    } else {


        let x_ao = (machine.a_movement.0 * (a_count + 1)) + (machine.b_movement.0 * b_count);
        let y_ao = (machine.a_movement.1 * (a_count + 1)) + (machine.b_movement.1 * b_count);

        let x_bo = (machine.a_movement.0 * a_count) + (machine.b_movement.0 * (b_count + 1));
        let y_bo = (machine.a_movement.1 * a_count) + (machine.b_movement.1 * (b_count + 1));

        let x_b = (machine.a_movement.0 * (a_count + 1)) + (machine.b_movement.0 * (b_count + 1));
        let y_b = (machine.a_movement.1 * (a_count + 1)) + (machine.b_movement.1 * (b_count + 1));

        if machine.prize.0 == x_ao && machine.prize.1 == y_ao {
            return ((a_count + 1) * 3) + b_count
        }
        if machine.prize.0 == x_bo && machine.prize.1 == y_bo {
            return (a_count * 3) + b_count + 1
        }
        if machine.prize.0 == x_b && machine.prize.1 == y_b {
            return ((a_count + 1) * 3) + b_count + 1
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(480, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(480, result);
    }

    #[test]
    fn test_get_minimum_to_prize() {
        let machine = Machine{
            a_movement: (17, 86),
            b_movement: (84, 37),
            prize: (7870, 6450)
        };
        let min_cost = get_minimum_to_prize_part_2(&machine);

        assert_eq!(200, min_cost)
    }
}