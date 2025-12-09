use std::collections::HashMap;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut parsed = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = parsed.len();
    let width = parsed[0].len();

    tilt_north(&mut parsed, height, width);

    parsed
        .iter()
        .enumerate()
        .map(|(index, line)| line.iter().filter(|&&c| c == 'O').count() * (height - index))
        .sum()
}

fn tilt_north(mut parsed: &mut Vec<Vec<char>>, height: usize, width: usize) {
    (0..height).for_each(|h| {
        (0..width).for_each(|w| match parsed[h][w] {
            'O' => {
                parsed[h][w] = '.';
                let idk = (0..h).rev().find(|&i| parsed[i][w] != '.');
                if let Some(i) = idk {
                    parsed[i + 1][w] = 'O';
                } else {
                    parsed[0][w] = 'O';
                }
            }
            _ => {}
        })
    });
}

fn tilt_south(mut parsed: &mut Vec<Vec<char>>, height: usize, width: usize) {
    (0..height).rev().for_each(|h| {
        (0..width).for_each(|w| match parsed[h][w] {
            'O' => {
                parsed[h][w] = '.';
                let idk = (h..height).find(|&i| parsed[i][w] != '.');
                if let Some(i) = idk {
                    parsed[i - 1][w] = 'O';
                } else {
                    parsed[height - 1][w] = 'O';
                }
            }
            _ => {}
        })
    });
}

fn tilt_west(mut parsed: &mut Vec<Vec<char>>, height: usize, width: usize) {
    (0..width).for_each(|w| {
        (0..height).rev().for_each(|h| match parsed[h][w] {
            'O' => {
                parsed[h][w] = '.';
                let idk = (0..w).rev().find(|&i| parsed[h][i] != '.');
                if let Some(i) = idk {
                    parsed[h][i + 1] = 'O';
                } else {
                    parsed[h][0] = 'O';
                }
            }
            _ => {}
        })
    });
}

fn tilt_east(mut parsed: &mut Vec<Vec<char>>, height: usize, width: usize) {
    (0..width).rev().for_each(|w| {
        (0..height).rev().for_each(|h| match parsed[h][w] {
            'O' => {
                parsed[h][w] = '.';
                let idk = (w..width).find(|&i| parsed[h][i] != '.');
                if let Some(i) = idk {
                    parsed[h][i - 1] = 'O';
                } else {
                    parsed[h][width - 1] = 'O';
                }
            }
            _ => {}
        })
    });
}

fn cycle(mut parsed: &mut Vec<Vec<char>>, height: usize, width: usize) {
    tilt_north(&mut parsed, height, width);
    tilt_west(&mut parsed, height, width);
    tilt_south(&mut parsed, height, width);
    tilt_east(&mut parsed, height, width);
}

fn part_2(input: &str) -> usize {
    let mut parsed = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = parsed.len();
    let width = parsed[0].len();

    let mut memo: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    let mut cycle_start: Option<usize> = None;
    let mut first_in_cycle: Option<Vec<Vec<char>>> = None;

    for c in 0..1000000000 {
        if memo.contains_key(&parsed) {
            parsed = memo[&parsed].clone();
            if cycle_start.is_none() {
                cycle_start = Some(c);
                first_in_cycle = Some(parsed.clone());
            } else if let Some(idk) = &first_in_cycle {
                if idk == &parsed {
                    println!("Cycle found at {}", cycle_start.unwrap());
                    println!("Cycle length: {}", c - cycle_start.unwrap());
                    (0..c - cycle_start.unwrap()).for_each(|_| {
                        cycle(&mut parsed, height, width);
                        let weight: usize = parsed
                            .iter()
                            .enumerate()
                            .map(|(index, line)| {
                                line.iter().filter(|&&c| c == 'O').count() * (height - index)
                            })
                            .sum();
                        println!("{}", weight);
                    });

                    break;
                }
            }
        } else {
            let key = parsed.clone();
            cycle(&mut parsed, height, width);
            memo.insert(key, parsed.clone());
        }
    }

    parsed
        .iter()
        .enumerate()
        .map(|(index, line)| line.iter().filter(|&&c| c == 'O').count() * (height - index))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 136);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 64);
    }
}
