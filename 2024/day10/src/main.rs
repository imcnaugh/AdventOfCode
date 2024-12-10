use std::collections::HashSet;
use std::fs;

fn main() {
    let input = get_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}");
}

fn get_file(path: &str) -> String{
    fs::read_to_string(path).unwrap()
}

fn build_grid(input: String) -> Vec<Vec<i32>> {
    let mut base_grid: Vec<Vec<i32>> = input.lines().map(|line| -> Vec<i32> {
        line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect()
    }).collect();

    let width = base_grid.len();
    let height = base_grid[0].len();

    for line in &mut base_grid {
        line.push(-1);
        line.insert(0, -1);
    }

    let top_bottom_buffer = vec![-1; width + 2];
    base_grid.insert(0, top_bottom_buffer.clone());
    base_grid.push(top_bottom_buffer.clone());

    base_grid
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn part_1(input: String) -> i32 {
    let grid = build_grid(input);
    let mut result = 0;
    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            let mut visited_summits: HashSet<(i32, i32)> = HashSet::new();
            let item = grid[row_index][col_index];
            if item == 0 {
                result += dfs(&grid, (row_index as i32, col_index as i32), &mut visited_summits)
            }
        }
    }
    result
}

fn part_2(input: String) -> i32 {
    todo!()
}

fn dfs(grid: &Vec<Vec<i32>>, current: (i32, i32), visited_summits : &mut HashSet<(i32, i32)>) -> i32 {
    let current_value = grid[current.0 as usize][current.1 as usize];

    if current_value == 9 {
        if visited_summits.contains(&(current.0, current.1)) {
            return 0
        }
        visited_summits.insert((current.0, current.1));
        return 1
    }

    let directions = [(1i32,0i32), (0,1), (-1,0), (0, -1)];

    let mut total = 0;
    for dir in directions {
        if grid[(current.0 + dir.0) as usize][(current.1 + dir.1) as usize] == current_value + 1{
            total += dfs(grid, (current.0 + dir.0, current.1 + dir.1), visited_summits);
        }
    }
    total
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(36, result)
    }

    #[test]
    fn test_part_2() {
        let input = get_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(81, result)
    }
}