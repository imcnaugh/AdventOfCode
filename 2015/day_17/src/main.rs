use std::collections::HashMap;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
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

fn part_2(input: &str) -> usize {
    let mut containers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let volume_to_fill = 150usize;
    let mut cache: Vec<Vec<usize>> = Vec::new();
    ways_to_fill_container_tracking_containers(volume_to_fill, &containers, vec![], &mut cache);

    let min = cache.iter().map(|c| c.len()).min().unwrap();
    cache.iter().filter(|c| c.len() == min).count()
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

fn ways_to_fill_container_tracking_containers(remaining_volume: usize, containers: &[usize], current_in_use: Vec<usize>, cache: &mut Vec<Vec<usize>>) -> usize {
    if remaining_volume == 0 {
        cache.push(current_in_use.to_vec());
        return 1;
    }

    containers
        .iter()
        .enumerate()
        .filter(|(index, c)| *c <= &remaining_volume)
        .fold(0usize, |acc, (index, con)| {
            let new_remaining_volume = remaining_volume - *con;
            let next_containers = &containers[index + 1..];
            let mut used_containers = current_in_use.clone();
            used_containers.push(*con);

            acc + ways_to_fill_container_tracking_containers(new_remaining_volume, &next_containers, used_containers, cache)
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
