use crate::Direction::{East, North, South, West};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let map = MirrorMap::from(input);
    map.get_energized_cell_count((0, 0), East)
}

fn part_2(input: &str) -> usize {
    let map = MirrorMap::from(input);
    todo!()
}

struct MirrorMap {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl MirrorMap {
    fn get_energized_cell_count(&self, start: (usize, usize), direction: Direction) -> usize {
        let mut energized_cells: HashSet<(usize, usize)> = HashSet::new();
        let mut previous: HashSet<(usize, usize, Direction)> = HashSet::new();
        self.trace_beam(start, direction, &mut energized_cells, &mut previous);
        energized_cells.len()
    }

    fn trace_beam(
        &self,
        location: (usize, usize),
        direction: Direction,
        energized_cells: &mut HashSet<(usize, usize)>,
        previous: &mut HashSet<(usize, usize, Direction)>,
    ) {
        if previous.contains(&(location.0.clone(), location.1.clone(), direction.clone())) {
            return;
        }
        previous.insert((location.0.clone(), location.1.clone(), direction.clone()));
        energized_cells.insert(location);

        let mut go = |dir: Direction| {
            let (nx, ny) = match dir {
                North => (location.0 as i32 - 1, location.1 as i32),
                South => (location.0 as i32 + 1, location.1 as i32),
                East => (location.0 as i32, location.1 as i32 + 1),
                West => (location.0 as i32, location.1 as i32 - 1),
            };

            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                self.trace_beam((nx as usize, ny as usize), dir, energized_cells, previous);
            }
        };

        let char_at_location = self.map[location.0][location.1];
        match (direction, char_at_location) {
            (East | West, '|') => {
                go(North);
                go(South);
            }
            (North | South, '-') => {
                go(East);
                go(West);
            }
            (North, '/') => go(East),
            (North, '\\') => go(West),
            (South, '/') => go(West),
            (South, '\\') => go(East),
            (East, '/') => go(North),
            (East, '\\') => go(South),
            (West, '/') => go(South),
            (West, '\\') => go(North),
            (North, _) => go(North),
            (South, _) => go(South),
            (East, _) => go(East),
            (West, _) => go(West),
        }
    }
}

impl From<&str> for MirrorMap {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        let height = map.len();
        let width = map[0].len();
        Self { map, height, width }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 46);
    }
}
