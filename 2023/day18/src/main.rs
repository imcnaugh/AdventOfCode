fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", solve(input, false));
    println!("Part 2: {}", solve(input, true));
}

fn solve(input: &str, is_part_2: bool) -> i64 {
    let mut current = (0, 0);
    let mut vertices = vec![current];
    let mut boundary_points = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (dir, steps) = if is_part_2 {
            part_2_parser(line)
        } else {
            part_1_parser(line)
        };

        let (dx, dy) = match dir {
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            "U" => (0, 1),
            _ => unreachable!(),
        };

        current = (current.0 + dx * steps as i64, current.1 + dy * steps as i64);
        vertices.push(current);
        boundary_points += steps as i64;
    }

    // Shoelace formula for area
    let mut area = 0;
    for i in 0..vertices.len() - 1 {
        area += vertices[i].0 * vertices[i + 1].1;
        area -= vertices[i + 1].0 * vertices[i].1;
    }
    area = area.abs() / 2;

    // Pick's Theorem: Area = InternalPoints + BoundaryPoints / 2 - 1
    // InternalPoints = Area - BoundaryPoints / 2 + 1
    // Total Volume = InternalPoints + BoundaryPoints
    // Total Volume = (Area - BoundaryPoints / 2 + 1) + BoundaryPoints
    // Total Volume = Area + BoundaryPoints / 2 + 1
    area + boundary_points / 2 + 1
}

fn part_1_parser(line: &str) -> (&str, i32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let dir = parts[0];
    let steps = parts[1].parse::<i32>().unwrap();
    (dir, steps)
}

fn part_2_parser(line: &str) -> (&str, i32) {
    let start = line.find('#').unwrap();
    let hex = &line[start + 1..start + 7];
    let steps = i32::from_str_radix(&hex[0..5], 16).unwrap();
    let dir = match &hex[5..6] {
        "0" => "R",
        "1" => "D",
        "2" => "L",
        "3" => "U",
        _ => panic!("Invalid direction"),
    };

    (dir, steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(solve(input, false), 62);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(solve(input, true), 952408144115);
    }
}
