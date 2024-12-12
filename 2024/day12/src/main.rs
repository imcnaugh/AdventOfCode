use std::collections::HashSet;
use std::fs;

fn main() {
    let input = read_file("resource/input.txt");
    let result = part_2(input);
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

    let grid = make_grid(input.clone());
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let mut regions: HashSet<(i32, Vec<(usize, usize)>)> = HashSet::new();

    for col_index in 0..grid.len() {
        for row_index in 0..grid[col_index].len() {
            if seen.contains(&(col_index, row_index)) {
                continue;
            }
            &seen.insert((col_index, row_index));
            let mut region_seen: HashSet<(usize, usize)> = HashSet::new();
            region_seen.insert((col_index, row_index));
            let plot = flood_fill(&grid, &mut region_seen, (col_index, row_index));

            let vec_region: Vec<(usize, usize)> = region_seen.clone().into_iter().collect();
            regions.insert((plot.0, vec_region));

            seen.extend(region_seen);
        }
    }


    let mut result = 0;

    for (area, points) in regions {
        let mut previous_index = Vec::<usize>::new();
        let mut indexes = Vec::<usize>::new();

        let mut pre_in_index = Vec::<usize>::new();
        let mut pre_out_index = Vec::<usize>::new();
        let mut in_index = Vec::<usize>::new();
        let mut out_index = Vec::<usize>::new();

        let mut sides: usize = 0;

        for (col_index, line) in input.lines().enumerate() {
            indexes.clear();
            in_index.clear();
            out_index.clear();
            let mut inside_region = false;


            for (row_index, c) in line.chars().map(|c| c.to_string()).enumerate() {
                if !inside_region && points.contains(&(col_index, row_index)) {
                    in_index.push(row_index);
                    inside_region = true
                }
                if inside_region && !points.contains(&(col_index, row_index)) {
                    out_index.push(row_index);
                    inside_region = false
                }
            }
            if inside_region {
                out_index.push(line.len());
            }

            sides += in_index.iter().map(|i| -> usize {
                if pre_in_index.contains(i){
                    0usize
                } else {
                    1
                }
            }).sum::<usize>();
            sides += out_index.iter().map(|i| -> usize {
                if pre_out_index.contains(i){
                    0usize
                } else {
                    1
                }
            }).sum::<usize>();

            sides += pre_in_index.iter().map(|i| -> usize {
                if in_index.contains(i) {
                    0usize
                } else {
                    1
                }
            }).sum::<usize>();
            sides += pre_out_index.iter().map(|i| -> usize {
                if out_index.contains(i) {
                    0usize
                } else {
                    1
                }
            }).sum::<usize>();

            previous_index = indexes.clone();
            pre_in_index = in_index.clone();
            pre_out_index = out_index.clone();
        }
        sides += in_index.len();
        sides += out_index.len();
        let cost = sides as i32 * area;

        println!("Area: {area}, Sides: {sides}, Cost: {cost}");

        result += cost
    }
    result as usize
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
        assert_eq!(1206, result);
    }
}
