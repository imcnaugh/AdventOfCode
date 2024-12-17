use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;

enum Space {
    Wall,
    Tile {distance: Option<(usize, Dir)>},
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Wall => {
                write!(f, "wall")
            },
            Space::Tile { distance: Some(info) } => {
                write!(f, "space, dist: {}, dir: {:?}", info.0, info.1)
            }
            _ => {
                write!(f, "space but no dist info")
            }
        }
    }
}

fn try_update(pos: (usize, usize), grid: &mut Vec<Vec<Space>>, new_steps: usize, new_dir: Dir) -> bool {
    let old_space = &grid[pos.1][pos.0];
    let mut add_back_to_unvisited = false;
    let should_update = match old_space {
        Space::Wall => false,
        Space::Tile { distance } => {
            match distance {
                None => true,
                Some((old_size, old_dir)) => {
                    let mut mod_size = old_size + 1000;
                    if mod_size > new_steps{
                        add_back_to_unvisited = true;
                        true
                    } else {
                        false
                    }
                }
            }
        }
    };
    
    if should_update {
        grid[pos.1][pos.0] = Space::Tile { distance: Some((new_steps, new_dir)) }
    }
    let space = &grid[pos.1][pos.0];
    add_back_to_unvisited
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn main() {
    let input = read_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}");
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: String) -> (Vec<Vec<Space>>, (usize, usize)) {
    let mut end = (0,0);

    let grid = input.lines().enumerate().map(|(row_index, line)| -> Vec<Space> {
        line.chars().enumerate().map(|(col_index, c)| -> Space {
            match c {
                '.' => Space::Tile {distance: None},
                'S' => {
                    Space::Tile {distance: Some((0, Dir::E))}
                },
                'E' => {
                    end = (col_index, row_index);
                    Space::Tile {distance: None}
                }
                _ => Space::Wall
            }
        }).collect()
    }).collect();

    (grid, end)
}

fn get_min_in_un_visited(nodes: &HashSet<(usize, usize)>, grid: &Vec<Vec<Space>>) -> Option<(usize, usize)> {
    let mut min: Option<(usize, usize)> = None;
    for node in nodes {
        let space = &grid[node.1][node.0];
        match space {
            Space::Tile { distance: Some(info) } => {
                match min {
                    None => min = Some(node.clone()),
                    Some(cur_min_pos) => {
                        let cur_min = &grid[cur_min_pos.1][cur_min_pos.0];
                        match cur_min {
                            Space::Tile{distance: Some(cur_min_info)} => {
                                if cur_min_info.0 > info.0 {
                                    min = Some(node.clone());
                                }
                            },
                            _ => {min = Some(node.clone())},
                        }
                    }
                }
            },
            _ => (),
        }
    }
    min
}

fn part_1(input: String) -> usize {
    let (mut grid, end_pos) = parse_input(input);
    let mut un_visited_nodes: HashSet<(usize, usize)> = grid.iter().enumerate()
        .map(|(row_index, row)| -> Vec<(usize, usize)>{
            let mut r: Vec<(usize, usize)> = Vec::new();
            row.iter().enumerate().for_each(|(col_index, c)| {
                match c {
                    Space::Wall => (),
                    Space::Tile {..} => r.push((col_index, row_index))
                }
            });
            r
        }).flat_map(|i| i).collect();

    loop {
        if un_visited_nodes.is_empty() {
            break;
        }
        let next_min = get_min_in_un_visited(&mut un_visited_nodes, &grid).unwrap();
        un_visited_nodes.remove(&next_min);

        let info = &grid[next_min.1][next_min.0];
        match info {
            Space::Tile { distance: Some(info) } => {
                let (f, s_1, s_2) = match info.1 {
                    Dir::N => {
                        (((next_min.0, next_min.1-1), Dir::N), ((next_min.0-1, next_min.1), Dir::W), ((next_min.0+1, next_min.1), Dir::E))
                    }
                    Dir::S => {
                        (((next_min.0, next_min.1+1), Dir::S), ((next_min.0-1, next_min.1), Dir::W), ((next_min.0+1, next_min.1), Dir::E))
                    }
                    Dir::E => {
                        (((next_min.0+1, next_min.1), Dir::E), ((next_min.0, next_min.1-1), Dir::N), ((next_min.0, next_min.1+1), Dir::S))
                    }
                    Dir::W => {
                        (((next_min.0-1, next_min.1), Dir::W), ((next_min.0, next_min.1-1), Dir::N), ((next_min.0, next_min.1+1), Dir::S))
                    }
                };

                let cur_dist = info.0;
                if try_update(f.0, &mut grid, cur_dist + 1, f.1){
                    un_visited_nodes.replace(f.0);
                };
                if try_update(s_1.0, &mut grid, cur_dist + 1001, s_1.1){
                    un_visited_nodes.replace(s_1.0);
                };
                if try_update(s_2.0, &mut grid, cur_dist + 1001, s_2.1){
                    un_visited_nodes.replace(s_2.0);
                };
            },
            _ => ()
        }
    }

    let end = &grid[end_pos.1][end_pos.0];
    match end {
        Space::Tile { distance: Some(info)} => info.0,
        _ => 0
    };

    let mut seen_spaces: HashSet<(usize, usize)> = HashSet::new();
    count_backwards(&grid, (end_pos.0, end_pos.1), &mut seen_spaces)
}

fn count_backwards(grid: &Vec<Vec<Space>>, cur_pos: (usize, usize), seen_spaces: &mut HashSet<(usize, usize)>) -> usize {
    if seen_spaces.contains(&cur_pos) {
        return 0
    }
    seen_spaces.insert(cur_pos);
    let space = &grid[cur_pos.1][cur_pos.0];
    let mut total = 1;
    if let Space::Tile { distance: Some(info) } = space {
        if info.0 != 0 {
            match info.1 {
                Dir::N => {
                    if let Space::Tile {distance: Some(i)} = &grid[cur_pos.1-1][cur_pos.0] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0, cur_pos.1-1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1][cur_pos.0-1] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0-1, cur_pos.1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1][cur_pos.0+1] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0+1, cur_pos.1), seen_spaces);
                        }
                    }
                }
                Dir::S => {
                    if let Space::Tile {distance: Some(i)} = &grid[cur_pos.1+1][cur_pos.0] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0, cur_pos.1+1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1][cur_pos.0-1] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0-1, cur_pos.1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1][cur_pos.0+1] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0+1, cur_pos.1), seen_spaces);
                        }
                    }
                }
                Dir::E => {
                    if let Space::Tile {distance: Some(i)} = &grid[cur_pos.1][cur_pos.0-1] {
                        if i.0 == info.0 - 1 {
                            total += count_backwards(grid, (cur_pos.0-1, cur_pos.1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1+1][cur_pos.0] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0, cur_pos.1+1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1+1][cur_pos.0] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0, cur_pos.1+1), seen_spaces);
                        }
                    }
                }
                Dir::W => {
                    if let Space::Tile {distance: Some(i)} = &grid[cur_pos.1][cur_pos.0+1] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0+1, cur_pos.1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1-1][cur_pos.0] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0, cur_pos.1-1), seen_spaces);
                        }
                    }

                    if let Space::Tile {distance: Some (i)} = &grid[cur_pos.1+1][cur_pos.0] {
                        if i.0 < info.0 {
                            total += count_backwards(grid, (cur_pos.0, cur_pos.1+1), seen_spaces);
                        }
                    }
                }
            }
        }
    }
    total
}

fn part_2(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(0, result);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }
}
