use regex::Regex;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    parse_inputs(input)
        .iter()
        .map(|(speed, run_time, rest_time)| get_distance(*speed, *run_time, *rest_time, 2503))
        .max()
        .unwrap()
}

fn get_distance(speed: u8, run_time: u8, rest_time: u8, total_time: usize) -> usize {
    let completed_loops = total_time / (run_time + rest_time) as usize;
    let distance_per_loop = speed as usize * run_time as usize;
    let remaining_time = (total_time % (run_time + rest_time) as usize).min(run_time as usize);
    (completed_loops * distance_per_loop) + (remaining_time * speed as usize)
}

fn parse_inputs(input: &str) -> Vec<(u8, u8, u8)> {
    let re = Regex::new(r#"fly (\d+).*for (\d+).*for (\d+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comet() {
        assert_eq!(get_distance(14, 10, 127, 1000), 1120);
    }

    #[test]
    fn test_dancer() {
        assert_eq!(get_distance(16, 11, 162, 1000), 1056);
    }
}
