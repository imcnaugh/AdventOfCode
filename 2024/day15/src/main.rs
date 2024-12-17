use std::collections::HashSet;
use std::fs;
use crate::Direction::{Down, Left, Right, Up};
use crate::Space::Wall;

#[derive(Copy, Clone)]
enum Space {
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn main() {
    let input = get_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}");
}

fn get_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn parse_input(input: String) -> (Vec<Vec<Option<Space>>>, Vec<Direction>, (usize, usize)) {
    let mut parts = input.split("\n\n");

    let wh_str = parts.next().unwrap();
    let dir_str = parts.next().unwrap().replace("\n", "");

    let mut robot_pos = (0usize, 0usize);

    let wh: Vec<Vec<Option<Space>>> = wh_str.lines().enumerate().map(|(row_index, line)| {
        line.chars().enumerate().map(|(col_index, c) | -> Vec<Option<Space>> {
            match c {
                '#' => vec![Some(Wall), Some(Wall)],
                'O' => vec![Some(Space::BoxLeft), Some(Space::BoxRight)],
                '@' => {
                    robot_pos = (col_index * 2, row_index);
                    vec![None, None]
                },
                _ => vec![None, None]
            }
        }).flat_map(|i| i).collect()
    }).collect();

    let dir: Vec<Direction> = dir_str.chars().map(|c| match c {
        '^' => Up,
        '>' => Right,
        'v' => Down,
        '<' => Left,
        _ => {Up}
    }).collect();

    (wh, dir, robot_pos)
}

fn get_next_pos(cur: (usize, usize), dir: &Direction) -> (usize, usize) {
    match dir {
        Up => (cur.0, cur.1 - 1),
        Left => (cur.0 - 1, cur.1),
        Down => (cur.0, cur.1 + 1),
        Right => (cur.0 + 1, cur.1),
    }
}

fn calc_gps(wh: &Vec<Vec<Option<Space>>>) -> usize {
    wh.iter().enumerate().map(|(row_index, line)| -> usize {
        let row_gps = row_index * 100;
        line.iter().enumerate().map(|(col_index, s)| -> usize {
            match s {
                Some(item) =>  {
                    match item {
                        Space::BoxLeft => row_gps + col_index,
                        _ => 0,
                    }
                },
                _ => 0,
            }
        }).sum()
    }).sum()
}

fn try_push(grid: &mut Vec<Vec<Option<Space>>>, cur_pos: (usize, usize), dir: &Direction) -> bool {
    match dir {
        Left => try_push_horizontal(grid, cur_pos, dir),
        Right => try_push_horizontal(grid, cur_pos, dir),
        Up => {
            let mut positions = HashSet::new();
            positions.insert(cur_pos);
            try_push_vertically(grid, positions, dir)
        },
        Down => {
            let mut positions = HashSet::new();
            positions.insert(cur_pos);
            try_push_vertically(grid, positions, dir)
        },
    }
}

fn try_push_vertically(grid: &mut Vec<Vec<Option<Space>>>, positions: HashSet<(usize, usize)>, dir: &Direction) -> bool {
    let mut pos_to_check: HashSet<(usize, usize)> = HashSet::new();
    for pos in &positions {
        let next_pos = get_next_pos(*pos, &dir);
        let item_at_next = grid[next_pos.1][next_pos.0];
        if let Some(item) = item_at_next {
            match item {
                Space::BoxLeft => {
                    pos_to_check.insert(next_pos);
                    pos_to_check.insert((next_pos.0 + 1, next_pos.1));
                }
                Space::BoxRight => {
                    pos_to_check.insert(next_pos);
                    pos_to_check.insert((next_pos.0 - 1, next_pos.1));
                }
                Wall => return false
            }
        }
    }

    let can_push = if pos_to_check.is_empty() {
        true
    } else {
        try_push_vertically(grid, pos_to_check, dir)
    };

    if can_push {
        for pos in &positions {
            let next_pos = get_next_pos(*pos, &dir);
            grid[next_pos.1][next_pos.0] = grid[pos.1][pos.0];
            grid[pos.1][pos.0] = None;
        }
    }

    can_push
}

fn try_push_horizontal(grid: &mut Vec<Vec<Option<Space>>>, cur_pos: (usize, usize), dir: &Direction) -> bool {
    let next_pos = get_next_pos(cur_pos, dir);
    let next_space = &grid[next_pos.1][next_pos.0];
    let can_push =  match next_space {
        Some(i) => {
            match i {
                Space::BoxLeft => try_push_horizontal(grid, next_pos, dir),
                Space::BoxRight => try_push_horizontal(grid, next_pos, dir),
                Wall => false
            }
        },
        None => true
    };

    if can_push {
        grid[next_pos.1][next_pos.0] = grid[cur_pos.1][cur_pos.0];
        grid[cur_pos.1][cur_pos.0] = None
    }

    can_push
}

fn print_wh(grid: &Vec<Vec<Option<Space>>>, cur_pos: (usize, usize)) {
    grid.iter().enumerate().for_each(|(row_index ,line)| {
        let l: String = line.iter().enumerate().map(|(col_index, s)| ->char {
            if cur_pos.0 == col_index && cur_pos.1 == row_index {
                return '@'
            }
            match s {
                Some(s) => match s {
                    Wall => '#',
                    Space::BoxLeft => '[',
                    Space::BoxRight => ']',
                },
                None => '.'
            }
        }).collect();
        println!("{l}");
    });
}

fn part_1(input: String) -> usize {
    let (mut grid, directions, start_pos) = parse_input(input);

    let mut cur_pos = start_pos;

    print_wh(&grid, start_pos);

    for dir in directions {
        if try_push(&mut grid, cur_pos, &dir) {
            cur_pos = get_next_pos(cur_pos, &dir);
        }
    }
    calc_gps(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(9021, result);
    }
}