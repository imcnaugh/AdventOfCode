use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1 : {}", part_1(input));
    println!("Part 2 : {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    sum_silly_ids_in_ranges(input, check_for_sillynes)
}

fn part_2(input: &str) -> usize {
    sum_silly_ids_in_ranges(input, check_for_repeating_sillynes)
}

fn sum_silly_ids_in_ranges(input: &str, is_silly: fn(&usize) -> bool) -> usize {
    input
        .split(',')
        .map(|range| parse_range(range).filter(is_silly).sum::<usize>())
        .sum()
}
fn parse_range(range: &str) -> RangeInclusive<usize> {
    let s = range
        .split('-')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    s[0]..=s[1]
}

fn check_for_sillynes(input: &usize) -> bool {
    let input = input.to_string();
    if input.len() % 2 != 0 {
        return false;
    }

    let mid = input.len() / 2;
    input[..mid] == input[mid..]
}

fn check_for_repeating_sillynes(input: &usize) -> bool {
    let input = input.to_string();
    (1..=input.len() / 2)
        .filter(|i| input.len() % i == 0)
        .any(|size| {
            (0..input.len())
                .step_by(size)
                .all(|i| input[i..i + size] == input[0..size])
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 4174379265);
    }
}
