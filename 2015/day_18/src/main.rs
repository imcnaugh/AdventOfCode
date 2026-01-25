fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let iterations = 100;
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let final_iteration = (0..iterations).fold(grid, |grid, _| run_iteration(grid, width, height));
    final_iteration
        .iter()
        .flatten()
        .filter(|&&cell| cell)
        .count()
}

fn part_2(input: &str) -> usize {
    let iterations = 100;

    let mut grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    grid[0][0] = true;
    grid[0][width - 1] = true;
    grid[height - 1][0] = true;
    grid[height - 1][width - 1] = true;

    let final_iteration = (0..iterations).fold(grid, |grid, _| {
        let mut new_grid = run_iteration(grid, width, height);
        new_grid[0][0] = true;
        new_grid[0][width - 1] = true;
        new_grid[height - 1][0] = true;
        new_grid[height - 1][width - 1] = true;
        new_grid
    });
    final_iteration
        .iter()
        .flatten()
        .filter(|&&cell| cell)
        .count()
}

fn run_iteration(grid: Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>> {
    let height = height as i32;
    let width = width as i32;
    let neighbors: Vec<(i32, i32)> = vec![
        (1, 1),
        (0, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let lit_neighbors = neighbors
                        .iter()
                        .filter(|(dx, dy)| {
                            let n_x = x + *dx;
                            let n_y = y + *dy;
                            if n_x < 0 || n_x >= width || n_y < 0 || n_y >= width {
                                false
                            } else {
                                grid[n_y as usize][n_x as usize]
                            }
                        })
                        .count();
                    match grid[y as usize][x as usize] {
                        true => lit_neighbors == 2 || lit_neighbors == 3,
                        false => lit_neighbors == 3,
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            print!("{}", if *cell { "#" } else { "." });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(part_1(input), 4);
    }

    #[test]
    fn test_part_2() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(part_2(input), 4);
    }
}
