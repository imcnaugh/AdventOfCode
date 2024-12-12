use std::collections::HashSet;
use std::fs;

fn main() {
    let input = read_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}")
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn make_grid(input: String) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| -> Vec<String> { line.chars().map(|c| c.to_string()).collect() })
        .collect()
}

fn print_grid(grid: &Vec<Vec<String>>) {
    for col in grid {
        for item in col {
            print!("{item}")
        }
        println!()
    }
}

fn part_1(input: String) -> usize {
    let grid = make_grid(input);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let mut result = 0;

    for col_index in 0..grid.len() {
        for row_index in 0..grid[col_index].len() {
            if seen.contains(&(col_index, row_index)) {
                continue;
            }
            &seen.insert((col_index, row_index));
            let plot = flood_fill(&grid, &mut seen, (col_index, row_index));
            let cost = plot.0 * plot.1;
            println!(
                "Region: {}, Area: {}, Perim: {}, Price: {}",
                &grid[col_index][row_index], plot.0, plot.1, cost
            );
            result += cost;
        }
    }

    result as usize
}

fn flood_fill(
    grid: &Vec<Vec<String>>,
    seen: &mut HashSet<(usize, usize)>,
    pos: (usize, usize),
) -> (i32, i32) {
    let directions = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    let current_char = &grid[pos.0][pos.1];
    let mut totals = (1, 0);

    for dir in directions {
        let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if is_out_of_bounds(&grid, new_pos) {
            totals.1 += 1;
        } else {
            let other_char = &grid[new_pos.0 as usize][new_pos.1 as usize];
            if other_char == current_char {
                if !seen.contains(&(new_pos.0 as usize, new_pos.1 as usize)) {
                    seen.insert((new_pos.0 as usize, new_pos.1 as usize));
                    let flood_results =
                        flood_fill(grid, seen, (new_pos.0 as usize, new_pos.1 as usize));
                    totals.0 += flood_results.0;
                    totals.1 += flood_results.1;
                }
            } else {
                totals.1 += 1;
            }
        }
    }

    totals
}

fn is_out_of_bounds(grid: &Vec<Vec<String>>, pos: (i32, i32)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        return true;
    }

    if pos.0 >= grid.len() as i32 || pos.1 >= grid[0].len() as i32 {
        return true;
    }
    false
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
        assert_eq!(1930, result);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }
}
