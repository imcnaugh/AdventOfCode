use std::fs;
use crate::Direction::{Down, Left, Right, Up};
use crate::Space::Wall;

enum Space {
    Box,
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
        line.chars().enumerate().map(|(col_index, c) | -> Option<Space> {
            match c {
                '#' => Some(Wall),
                'O' => Some(Space::Box),
                '@' => {
                    robot_pos = (col_index, row_index);
                    None
                },
                _ => None
            }
        }).collect()
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
                        Space::Box => row_gps + col_index,
                        _ => 0,
                    }
                },
                _ => 0,
            }
        }).sum()
    }).sum()
}

fn try_push(grid: &Vec<Vec<Option<Space>>>, cur_pos: (usize, usize), dir: &Direction) -> Option<(usize, usize)> {
    loop {
        let next_pos = get_next_pos(cur_pos, dir);
        let next_space = &grid[next_pos.1][next_pos.0];
        return match next_space {
            Some(i) => {
                match i {
                    Space::Box => try_push(grid, next_pos, dir),
                    Wall => None
                }
            },
            None => Some(next_pos)
        }
    }
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
                    Space::Box => 'O'
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

    for dir in directions {
        match try_push(&grid, cur_pos, &dir) {
            Some(next_free) => {
                let next_pos = get_next_pos(cur_pos, &dir);
                if next_free != next_pos {
                    grid[next_pos.1][next_pos.0] = None;
                    grid[next_free.1][next_free.0] = Some(Space::Box);
                }
                cur_pos = next_pos;
            },
            None => continue,
        }
        //
        // println!("{:?}", dir);
        // print_wh(&grid, cur_pos);

    }

    calc_gps(&grid)
}

fn part_2(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(2028, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }
}