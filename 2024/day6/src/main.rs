use std::collections::HashSet;
use std::fs;

fn main() {
    let input = get_data_from_file("resources/input.txt");
    let result = part_2(input);
    println!("{result}");
}

fn part_1(input: String) -> usize {
    let grid = input.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let height = grid[0].len() as i32;
    let width = grid.len() as i32;
    let movement = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
    let mut movement_index = 0;

    let mut current_pos = (0i32, 0i32);
    'here: for col in 0..grid.len() {
        for row in 0..grid[col].len() {
            current_pos = (row as i32, col as i32);
            if grid[row][col] == '^' {
                break 'here;
            }
        }
    }

    let mut visited_squares: HashSet<(i32, i32)> = HashSet::new();

    loop {
        // print_grid(&grid, &current_pos);
        let next_square = (current_pos.0 + movement[movement_index].0, current_pos.1 + movement[movement_index].1);
        if next_square.0 < 0 || next_square.0 >= width || next_square.1 < 0 || next_square.1 >= height {
            break;
        }

        if grid[next_square.0 as usize][next_square.1 as usize] == '#' {
            movement_index = (movement_index + 1) % 4;
            continue;
        }
        current_pos = next_square;
        visited_squares.insert(current_pos);
    }

    visited_squares.len()
}

fn _print_grid(grid: &Vec<Vec<char>>, current_pos: &(i32, i32)) {
    for col in 0..grid.len(){
        for row in 0..grid[col].len() {
            if col == current_pos.0 as usize && row == current_pos.1 as usize {
                print!("O")
            } else {
                print!("{}", grid[col][row])
            }
        }
        println!();
    }
    println!();
    println!();
}

fn part_2(input: String) -> usize {
    let grid = input.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let height = grid[0].len() as i32;
    let width = grid.len() as i32;

    let mut current_pos = (0i32, 0i32);
    'here: for col in 0..grid.len() {
        for row in 0..grid[col].len() {
            current_pos = (row as i32, col as i32);
            if grid[row][col] == '^' {
                break 'here;
            }
        }
    }

    let mut possible_blocks: HashSet<(i32, i32)> = HashSet::new();

    let movement = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];

    let mut movement_index = 0;
    loop {
        let next_square: (i32, i32) = (current_pos.0 + movement[movement_index].0, current_pos.1 + movement[movement_index].1);
        if next_square.0 < 0 || next_square.0 >= width || next_square.1 < 0 || next_square.1 >= height {
            break;
        }

        if grid[next_square.0 as usize][next_square.1 as usize] == '#' {
            movement_index = (movement_index + 1) % 4;
            continue;
        }

        if do_loop(&grid.clone(), height, width, (movement_index + 1) % 4, current_pos) {
            possible_blocks.insert(next_square);
        }

        current_pos = next_square;
    }

    possible_blocks.len()
}

fn do_loop(grid: &Vec<Vec<char>>, height: i32, width: i32, movement_index: usize, current_pos: (i32, i32)) -> bool {
    let movement = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];

    let initial_movement_index = movement_index;
    let initial_pos = current_pos;
    let expected_move_index = match movement_index {
        0 => 3,
        _ => movement_index - 1,
    };

    let mut movement_index = initial_movement_index.clone();
    let mut current_pos = initial_pos.clone();
    loop {
        let next_square = (current_pos.0 + movement[movement_index].0, current_pos.1 + movement[movement_index].1);
        if next_square.0 < 0 || next_square.0 >= width || next_square.1 < 0 || next_square.1 >= height {
            return false;
        }
        if next_square.0 == initial_pos.0 && next_square.1 == initial_pos.1 && expected_move_index == movement_index {
            return true;
        }

        if grid[next_square.0 as usize][next_square.1 as usize] == '#' {
            movement_index = (movement_index + 1) % 4;
            continue;
        }

        current_pos = next_square;
    }
}

fn get_data_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_1() {
        let input = get_data_from_file("resources/test.txt");
        let result = part_1(input);
        assert_eq!(41, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_data_from_file("resources/test.txt");
        let result = part_2(input);
        assert_eq!(6, result);
    }
}
