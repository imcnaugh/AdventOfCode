// use std::cmp::PartialEq; // not needed

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    let width = map[0].len();
    let height = map.len();

    (0..height)
        .map(|h| {
            (0..width)
                .filter(|w| {
                    if map[h][*w] == '.' {
                        return false;
                    }

                    let can_check_up = h > 0;
                    let can_check_down = h < height - 1;
                    let can_check_left = w > &0;
                    let can_check_right = w < &(width - 1);
                    let mut total = 0;

                    if can_check_up && map[h - 1][*w] == '@' {
                        total += 1;
                    }
                    if can_check_down && map[h + 1][*w] == '@' {
                        total += 1;
                    }
                    if can_check_left && map[h][w - 1] == '@' {
                        total += 1;
                    }
                    if can_check_right && map[h][w + 1] == '@' {
                        total += 1;
                    }

                    if can_check_up && can_check_left && map[h - 1][w - 1] == '@' {
                        total += 1;
                    }
                    if can_check_up && can_check_right && map[h - 1][w + 1] == '@' {
                        total += 1;
                    }
                    if can_check_down && can_check_left && map[h + 1][w - 1] == '@' {
                        total += 1;
                    }
                    if can_check_down && can_check_right && map[h + 1][w + 1] == '@' {
                        total += 1;
                    }

                    total < 4
                })
                .count()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut map = parse_input(input);
    let width = map[0].len();
    let height = map.len();

    let mut total_removed = 0;

    loop {
        let mut next_map = map.clone();

        let removed: usize = (0..height)
            .map(|h| {
                (0..width)
                    .filter(|&w| {
                        if map[h][w] == '.' {
                            return false;
                        }

                        let can_check_up = h > 0;
                        let can_check_down = h < height - 1;
                        let can_check_left = w > 0;
                        let can_check_right = w < (width - 1);
                        let mut total = 0;

                        if can_check_up && map[h - 1][w] == '@' {
                            total += 1;
                        }
                        if can_check_down && map[h + 1][w] == '@' {
                            total += 1;
                        }
                        if can_check_left && map[h][w - 1] == '@' {
                            total += 1;
                        }
                        if can_check_right && map[h][w + 1] == '@' {
                            total += 1;
                        }

                        if can_check_up && can_check_left && map[h - 1][w - 1] == '@' {
                            total += 1;
                        }
                        if can_check_up && can_check_right && map[h - 1][w + 1] == '@' {
                            total += 1;
                        }
                        if can_check_down && can_check_left && map[h + 1][w - 1] == '@' {
                            total += 1;
                        }
                        if can_check_down && can_check_right && map[h + 1][w + 1] == '@' {
                            total += 1;
                        }

                        if total < 4 {
                            // Remove the current cell in the next_map; neighbors remain unchanged this round
                            next_map[h][w] = '.';
                            true
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum();

        total_removed += removed;
        map = next_map;

        if removed == 0 {
            return total_removed;
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(part_2(input), 43);
    }
}
