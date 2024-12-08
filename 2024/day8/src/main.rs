use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = get_file_as_string("resource/input.txt");
    let result = part_2(input);
    println!("{}", result);
}

fn get_file_as_string(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn part_1(input: String) -> usize {
    let mut antenna_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        height += 1;
        for (col, c) in line.chars().enumerate() {
            width = max(width, col as i32);
            if c != '.' {
                if antenna_map.contains_key(&c) {
                    let current = antenna_map.get_mut(&c).unwrap();
                    current.push((row as i32, col as i32));
                } else {
                    antenna_map.insert(c, vec![(row as i32, col as i32)]);
                }
                // antenna_map.entry(c).or_insert_with(Vec::new).push((row as i32, col as i32));
            }
        }
    }

    let mut total = 0;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for antennas in antenna_map.keys() {
        println!("{:?}", antennas);
        let positions = antenna_map.get(antennas).unwrap();
        if positions.len() == 1 {
            continue;
        }

        for (index, position) in positions.iter().enumerate() {
            for other_index in index + 1..positions.len() {
                let other = positions.get(other_index).unwrap();
                let node_a = position;
                let node_b = other;

                let get_anti_node = |a: &(i32, i32), b: &(i32, i32)| -> (i32, i32) {
                    let x_dif = a.0 - b.0;
                    let y_dif = a.1 - b.1;
                    let x_pos = a.0 - (x_dif * 2);
                    let y_pos = a.1 - (y_dif * 2);
                    return (x_pos, y_pos);
                };

                let anti_node_a = get_anti_node(node_a, node_b);
                let anti_node_b = get_anti_node(node_b, node_a);

                let is_node_out_of_bounds = |(y, x)| -> bool {
                    x < 0 || y < 0 || x > width || y >= height
                };

                if !is_node_out_of_bounds(anti_node_a) {
                    antinodes.insert(anti_node_a);
                    total += 1;
                }

                if !is_node_out_of_bounds(anti_node_b) {
                    antinodes.insert(anti_node_b);
                    total += 1;
                }

            }
        }
    }

    // for antennas in antenna_map.keys() {
    //     antenna_map.get(antennas).unwrap().iter().for_each(|(row, col)| {
    //         antinodes.remove(&(*row, *col));
    //     })
    // }
    antinodes.len()
}

fn part_2(input: String) -> usize {    let mut antenna_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        height += 1;
        for (col, c) in line.chars().enumerate() {
            width = max(width, col as i32);
            if c != '.' {
                if antenna_map.contains_key(&c) {
                    let current = antenna_map.get_mut(&c).unwrap();
                    current.push((row as i32, col as i32));
                } else {
                    antenna_map.insert(c, vec![(row as i32, col as i32)]);
                }
                // antenna_map.entry(c).or_insert_with(Vec::new).push((row as i32, col as i32));
            }
        }
    }

    let mut total = 0;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for antennas in antenna_map.keys() {
        println!("{:?}", antennas);
        let positions = antenna_map.get(antennas).unwrap();
        if positions.len() == 1 {
            continue;
        }

        for (index, position) in positions.iter().enumerate() {
            for other_index in index + 1..positions.len() {
                let other = positions.get(other_index).unwrap();
                let node_a = position;
                let node_b = other;

                let get_anti_node = |a: &(i32, i32), b: &(i32, i32)| -> (i32, i32) {
                    let x_dif = a.0 - b.0;
                    let y_dif = a.1 - b.1;
                    let x_pos = a.0 - (x_dif * 2);
                    let y_pos = a.1 - (y_dif * 2);
                    return (x_pos, y_pos);
                };

                let is_node_out_of_bounds = |(y, x)| -> bool {
                    x < 0 || y < 0 || x > width || y >= height
                };

                let mut loop_it = |a: &(i32, i32), b: &(i32, i32)| {
                    let mut a = *a;
                    let mut b = *b;

                    loop {
                        let new = get_anti_node(&a, &b);
                        if is_node_out_of_bounds(new) {
                            break;
                        }
                        antinodes.insert(new);

                        a = b.clone();
                        b = new.clone();
                    }
                };

                loop_it(node_a, node_b);
                loop_it(node_b, node_a);

                antinodes.insert(*node_a);
                antinodes.insert(*node_b);

                // let anti_node_a = get_anti_node(node_a, node_b);
                // let anti_node_b = get_anti_node(node_b, node_a);
                //
                //
                //
                // if !is_node_out_of_bounds(anti_node_a) {
                //     antinodes.insert(anti_node_a);
                //     total += 1;
                // }
                //
                // if !is_node_out_of_bounds(anti_node_b) {
                //     antinodes.insert(anti_node_b);
                //     total += 1;
                // }

            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file_as_string("resource/test.txt");
        let result = part_1(input);
        assert_eq!(14, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_file_as_string("resource/test.txt");
        let result = part_2(input);
        assert_eq!(34, result);
    }
}