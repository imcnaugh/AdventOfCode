use std::collections::HashSet;
use std::fs;

fn main() {
    let input = get_data_from_file("resources/input.txt");
    let result = part_2(input);
    println!("{result}");
}

// fn part_1(input: String) -> usize {
//     let grid = input.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
//     let height = grid[0].len() as i32;
//     let width = grid.len() as i32;
//     let movement = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
//     let mut movement_index = 0;
//
//     let mut current_pos = (0i32, 0i32);
//     'here: for row in 0..grid.len() {
//         for col in 0..grid[row].len() {
//             current_pos = (row as i32, col as i32);
//             if grid[row][col] == '^' {
//                 break 'here;
//             }
//         }
//     }
//
//     let mut visited_squares: HashSet<(i32, i32)> = HashSet::new();
//
//     loop {
//         let next_square = (current_pos.0 + movement[movement_index].0, current_pos.1 + movement[movement_index].1);
//         if next_square.0 < 0 || next_square.0 >= width || next_square.1 < 0 || next_square.1 >= height {
//             break;
//         }
//
//         if grid[next_square.0 as usize][next_square.1 as usize] == '#' {
//             movement_index = (movement_index + 1) % 4;
//             continue;
//         }
//         current_pos = next_square;
//         visited_squares.insert(current_pos);
//     }
//
//     visited_squares.len()
// }

fn _print_grid(grid: &Vec<Vec<char>>, current_pos: &(i32, i32), movement_index: usize, obs: Option<(i32, i32)>) {
    for col in 0..grid.len(){
        for row in 0..grid[col].len() {
            if col == current_pos.0 as usize && row == current_pos.1 as usize {
                match movement_index {
                    0 =>print!("^"),
                    1 =>print!(">"),
                    2 =>print!("V"),
                    3 =>print!("<"),
                    _ =>print!("x"),
                }
            } else if let Some(obs) = obs {
                if col == obs.0 as usize && row == obs.1 as usize {
                    print!("O")
                } else {
                    print!("{}", grid[col][row])
                }
            }else {
                print!("{}", grid[col][row])
            }
        }
        println!();
    }
    println!();
    println!();
}


fn part_2(input: String) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines() // Iterate over lines (similar to `split("\n")`, but skips empty trailing lines)
        .map(|line| line.chars().collect()) // Convert each line to a Vec<char>
        .collect();

    let height = &grid[0].len();
    let width = &grid.len();
    let height = *height as i32;
    let width = *width as i32;
    let movement = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
    let mut movement_index = 0;

    let mut current_pos = (0i32, 0i32);
    'here: for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            current_pos = (row as i32, col as i32);
            if grid[row][col] == '^' {
                break 'here;
            }
        }
    }
    let initial_position = current_pos.clone();

    let mut possible_blocks: HashSet<(i32, i32)> = HashSet::new();

    for c in 0..height {
        for r in 0..width {
            let char_at = grid[c as usize][r as usize];
            match char_at {
                '#' => continue,
                '^' => continue,
                _ => {
                    let mut copy_of_grid = grid.clone();
                    copy_of_grid[c as usize][r as usize] = '#';
                    if is_loop(&copy_of_grid, movement_index, current_pos) {
                        possible_blocks.insert((c, r));
                    }
                }
            }
        }
    }

    possible_blocks.remove(&initial_position);

    possible_blocks.len()
}

fn is_loop(grid: &Vec<Vec<char>>, movement_index: usize, current_pos: (i32, i32)) -> bool {
    let movement = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
    let height = grid[0].len() as i32;
    let width = grid.len() as i32;

    let mut visited_squares: HashSet<(usize, usize, usize)> = HashSet::new();

    let mut movement_index = movement_index.clone();
    let mut current_pos = current_pos.clone();
    loop {
        if visited_squares.contains(&(current_pos.0 as usize, current_pos.1 as usize, movement_index)){
            return true;
        }
        visited_squares.insert((current_pos.0 as usize, current_pos.1 as usize, movement_index));

        let next_square = (
            current_pos.0 + movement[movement_index].0,
            current_pos.1 + movement[movement_index].1
        );
        if next_square.0 < 0 || next_square.0 >= width || next_square.1 < 0 || next_square.1 >= height {
            return false;
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
    
    // #[test]
    // fn test_part_1() {
    //     let input = get_data_from_file("resources/test.txt");
    //     let result = part_1(input);
    //     assert_eq!(41, result);
    // }

    #[test]
    fn test_part_2() {
        let input = get_data_from_file("resources/test.txt");
        let result = part_2(input);
        assert_eq!(6, result);
    }
}
