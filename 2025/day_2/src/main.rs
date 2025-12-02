fn main() {
    let input = include_str!("../resource/input.txt");
    let result = part_2(input);
    println!("Part 1: {}", result);
}

fn part_1(input: &str) -> usize {
    input.split(',').fold(0, |acc, range| {
        let s = range
            .split('-')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let start = s[0];
        let end = s[1];

        acc + (start..=end).fold(0, |acc, i| {
            if check_for_kinda_sillyness(&i.to_string()) {
                acc + i
            } else {
                acc
            }
        })
    })
}

fn part_2(input: &str) -> usize {
    input.split(',').fold(0, |acc, range| {
        let s = range
            .split('-')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let start = s[0];
        let end = s[1];

        acc + (start..=end).fold(0, |acc, i| {
            if check_for_sillyness(&i.to_string()) {
                acc + i
            } else {
                acc
            }
        })
    })
}

fn check_for_kinda_sillyness(input: &str) -> bool {
    if input.len() % 2 != 0 {
        return false;
    }

    let mid = input.len() / 2;
    input[0..mid] == input[mid..]
}

fn check_for_sillyness(input: &str) -> bool {
    'outer: for i in 1..input.len() {
        if input.len() % i != 0 {
            continue;
        }

        let rep = &input[0..i];
        for j in (0..input.len()).step_by(i) {
            if input[j..j + i] != *rep {
                continue 'outer;
            }
        }
        return true;
    }
    false
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
