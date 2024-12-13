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
        let p_x: usize = format!("10000000000000{}", &captures["p_x"]).parse().unwrap();
        let p_y: usize = format!("10000000000000{}", &captures["p_y"]).parse().unwrap();

        Machine {
            a_movement: (captures["a_x"].parse::<usize>().unwrap(), captures["a_y"].parse::<usize>().unwrap()),
            b_movement: (captures["b_x"].parse::<usize>().unwrap(), captures["b_y"].parse::<usize>().unwrap()),
            prize: (p_x, p_y)
        }
    }).collect()
}

fn part_1(input: String) -> usize {
    parse_input(input).iter().map(|m| -> usize {
        get_minimum_to_prize_part_2(m).unwrap_or_else(|| 0)
    }).sum()
}

fn get_minimum_to_prize(machine: &Machine) -> Option<usize> {
    let mut b_presses: HashMap<(usize, usize), usize> = HashMap::new();

    for presses in 0..100 {
        let dist = (machine.b_movement.0 * presses, machine.b_movement.1 * presses);
        b_presses.insert(dist, presses);
    }

    let mut minimum_cost: Option<usize> = None;
    for a_pres_count in 0..100 {
        let dist = (machine.a_movement.0 * a_pres_count, machine.a_movement.1 * a_pres_count);
        if dist.0 > machine.prize.0 || dist.1 > machine.prize.1 {
            continue;
        }
        let dist_to_prize = (machine.prize.0 - dist.0, machine.prize.1 - dist.1);
        if let Some(b_press_count) = b_presses.get(&dist_to_prize){
            let cost = (a_pres_count * 3) + b_press_count;
            match minimum_cost {
                Some(cur_min) => {
                    if cur_min > cost {
                        minimum_cost = Some(cost);
                    }
                },
                None => minimum_cost = Some(cost),
            }
        }
    }

    minimum_cost
}


fn get_minimum_to_prize_part_2(machine: &Machine) -> Option<usize> {
    let mut minimum_cost: Option<usize> = None;
    let eq = |b: usize| -> bool {
        let top: usize = machine.prize.0 - (machine.b_movement.0 * b);
        (top % machine.a_movement.0) == 0
    };

    let mut cur_b = 0;
    loop {
        if (machine.b_movement.0 * cur_b) > machine.prize.0 {
            break;
        }

        if eq(cur_b) {
            let a_press = (machine.prize.0 - (machine.b_movement.0 * cur_b)) / machine.a_movement.0;
            if machine.prize.1 == (machine.a_movement.1 * a_press) + (machine.b_movement.1 * cur_b) {
                let cost = (a_press * 3) + cur_b;
                match minimum_cost {
                    None => minimum_cost = Some(cost),
                    Some(cur_min) => {
                        if cur_min > cost {
                            minimum_cost = Some(cost)
                        }
                    }
                }
            }
        }

        cur_b += 1;
    }
    minimum_cost
}

fn part_2(input: String) -> usize {
    parse_input_part_2(input).iter().map(|m| -> usize {
        println!("next");
        get_minimum_to_prize_part_2(m).unwrap_or_else(|| 0)
    }).sum()
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
            a_movement: (94, 34),
            b_movement: (22, 67),
            prize: (8400, 5400)
        };
        let min_cost = get_minimum_to_prize_part_2(&machine);

        assert_eq!(280, min_cost.unwrap())
    }
}