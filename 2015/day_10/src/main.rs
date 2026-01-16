fn main() {
    let input = include_str!("../resources/input.txt");
    let output_part_1 = part_1(input);
    let output_part_2 = part_2(input);
    println!("Part 1: {}", output_part_1);
    println!("Part 2: {}", output_part_2);
}

fn part_1(input: &str) -> usize {
    let mapped_input = input
        .chars()
        .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
        .collect::<Vec<usize>>();
    let result = (0..40).fold(mapped_input, |acc, _| say_and_speak(&acc));
    result.len()
}

fn say_and_speak(input: &[usize]) -> Vec<usize> {
    let mut ret_val = Vec::new();
    let mut iter = input.iter();
    let mut prev = iter.next().unwrap();
    let mut count = 1;
    for curr in iter {
        if curr == prev {
            count += 1;
        } else {
            ret_val.push(count);
            ret_val.push(*prev);
            prev = curr;
            count = 1;
        }
    }
    ret_val.push(count);
    ret_val.push(*prev);
    ret_val
}

fn part_2(input: &str) -> usize {
    let mapped_input = input
        .chars()
        .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
        .collect::<Vec<usize>>();
    let result = (0..50).fold(mapped_input, |acc, _| say_and_speak(&acc));
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(say_and_speak(&vec![2, 1, 1]), vec![1, 2, 2, 1]);
    }
}
