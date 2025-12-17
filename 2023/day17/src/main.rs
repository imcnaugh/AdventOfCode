use crate::Direction::{Down, Left, Right, Up};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    // Dijkstra over (row, col, direction, run_len with ultra constraints: min 4 to turn/stop, max 10)
    let grid = parse_input_to_char_matrix(input);
    let h = grid.len();
    let w = grid[0].len();

    // dist[r][c][dir][run-1]
    let mut dist = vec![vec![vec![vec![u32::MAX; 10]; 4]; w]; h];
    let mut heap: BinaryHeap<Reverse<(u32, usize, usize, Direction, usize)>> = BinaryHeap::new();

    // seed from (0,0): can move Right or Down with run=1
    for &dir in &[Right, Down] {
        let (dr, dc) = delta(dir);
        let nr = dr;
        let nc = dc;
        if in_bounds(nr, nc, h, w) {
            let r = nr as usize;
            let c = nc as usize;
            let cost = grid[r][c].to_digit(10).unwrap();
            dist[r][c][dir_index(dir)][0] = cost;
            heap.push(Reverse((cost, r, c, dir, 1)));
        }
    }

    while let Some(Reverse((cost, r, c, dir, run))) = heap.pop() {
        if cost > dist[r][c][dir_index(dir)][run - 1] {
            continue;
        }
        // For ultra crucible (Part 2), you may only stop at the end if the current straight run >= 4
        if r == h - 1 && c == w - 1 && run >= 4 {
            return cost as usize;
        }

        for &ndir in &[Up, Down, Left, Right] {
            if is_reverse(dir, ndir) {
                continue;
            }
            let nrun = if ndir == dir { run + 1 } else { 1 };
            if nrun > 10 || (dir != ndir && run < 4) {
                continue;
            }
            let (dr, dc) = delta(ndir);
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if !in_bounds(nr, nc, h, w) {
                continue;
            }
            let rr = nr as usize;
            let cc = nc as usize;
            let ncost = cost + grid[rr][cc].to_digit(10).unwrap();
            let slot = &mut dist[rr][cc][dir_index(ndir)][nrun - 1];
            if ncost < *slot {
                *slot = ncost;
                heap.push(Reverse((ncost, rr, cc, ndir, nrun)));
            }
        }
    }

    unreachable!("target must be reachable")
}

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn delta(d: Direction) -> (isize, isize) {
    match d {
        Up => (-1, 0),
        Down => (1, 0),
        Left => (0, -1),
        Right => (0, 1),
    }
}

fn is_reverse(a: Direction, b: Direction) -> bool {
    matches!(
        (a, b),
        (Up, Down) | (Down, Up) | (Left, Right) | (Right, Left)
    )
}

fn dir_index(d: Direction) -> usize {
    match d {
        Up => 0,
        Down => 1,
        Left => 2,
        Right => 3,
    }
}

fn parse_input_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn in_bounds(r: isize, c: isize, h: usize, w: usize) -> bool {
    r >= 0 && c >= 0 && (r as usize) < h && (c as usize) < w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 94);
    }
}
