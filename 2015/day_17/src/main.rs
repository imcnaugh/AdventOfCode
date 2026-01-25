use std::collections::HashMap;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut containers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    containers.sort();
    let volume_to_fill = 150usize;
    ways_to_fill_container(volume_to_fill, &containers)
}

fn ways_to_fill_container(remaining_volume: usize, containers: &[usize]) -> usize {
    if remaining_volume == 0 {
        return 1;
    }

    containers
        .iter()
        .enumerate()
        .filter(|(index, c)| *c <= &remaining_volume)
        .fold(0usize, |acc, (index, con)| {
            let new_remaining_volume = remaining_volume - *con;
            let next_containers = &containers[index + 1..];

            acc + ways_to_fill_container(new_remaining_volume, &next_containers)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let containers = vec![20, 15, 10, 5, 5];
        let fill_volume = 25;
        let ways_to_fill = ways_to_fill_container(fill_volume, &containers);
        assert_eq!(ways_to_fill, 4);
    }
}
