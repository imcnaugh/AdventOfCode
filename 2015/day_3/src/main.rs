use std::collections::HashSet;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);
    input.chars().fold((0, 0), |cur, c| {
        let new_pos = get_new_pos(cur, c);
        visited.insert(new_pos);
        new_pos
    });
    visited.len()
}

fn part_2(input: &str) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);
    input
        .chars()
        .enumerate()
        .fold(((0, 0), (0, 0)), |cur, (index, c)| {
            let i_to_change = index & 1;
            let new_pos = match i_to_change {
                0 => {
                    let pos = get_new_pos(cur.0, c);
                    visited.insert(pos);
                    (pos, cur.1)
                }
                1 => {
                    let pos = get_new_pos(cur.1, c);
                    visited.insert(pos);
                    (cur.0, pos)
                }
                _ => panic!("Invalid index"),
            };
            new_pos
        });
    visited.len()
}

fn get_new_pos(cur: (isize, isize), c: char) -> (isize, isize) {
    match c {
        '>' => (cur.0 + 1, cur.1),
        '<' => (cur.0 - 1, cur.1),
        'v' => (cur.0, cur.1 + 1),
        '^' => (cur.0, cur.1 - 1),
        _ => cur,
    }
}
