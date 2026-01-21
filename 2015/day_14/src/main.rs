use regex::Regex;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    parse_inputs(input)
        .iter()
        .map(|(speed, run_time, rest_time)| get_distance(*speed, *run_time, *rest_time, 2503))
        .max()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let parsed_inputs = parse_inputs(input);
    let race_length = 2503usize;
    get_max_points_after_race(parsed_inputs, race_length)
}

fn get_max_points_after_race(parsed_inputs: Vec<(u8, u8, u8)>, race_length: usize) -> usize {
    let mut distance: Vec<usize> = vec![0; parsed_inputs.len()];
    let mut points: Vec<usize> = vec![0; parsed_inputs.len()];
    (0..=race_length).for_each(|time| {
        parsed_inputs
            .iter()
            .zip(distance.iter_mut())
            .for_each(|(deer, distance)| {
                *distance += if is_deer_running(deer, time) {
                    deer.0 as usize
                } else {
                    0
                };
            });

        let max_distance = distance.iter().max().unwrap();
        distance
            .iter()
            .zip(points.iter_mut())
            .for_each(|(distance, points)| {
                if distance == max_distance {
                    *points += 1;
                }
            });
    });
    points.into_iter().max().unwrap()
}

fn is_deer_running(deer: &(u8, u8, u8), time: usize) -> bool {
    let i = time % (deer.1 + deer.2) as usize;
    (i as u8) < deer.1
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

    #[test]
    fn is_deer_running_test() {
        (0..10).for_each(|time| assert!(is_deer_running(&(1, 10, 10), time), "time: {}", time));
        (10..20).for_each(|time| assert!(!is_deer_running(&(1, 10, 10), time), "time: {}", time));
    }

    #[test]
    fn part_2() {
        let deer = vec![(14, 10, 127), (16, 11, 162)];
        let time = 1000;
        assert_eq!(get_max_points_after_race(deer, time), 689);
    }
}
