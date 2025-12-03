fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    parse_input(input).iter().map(|row| find_max_joltage(row, 2)).sum()
}

fn part_2(input: &str) -> usize {
    parse_input(input).iter().map(|row| find_max_joltage(row, 12)).sum()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_max_joltage(row: &Vec<u8>, output_len: usize) -> usize {
    let mut buffer: Vec<Option<u8>> = vec![None; output_len];

    row.iter().enumerate().for_each(|(row_index, &row_value)| {
        let buffer_start_index = buffer.len() - (row.len() - row_index).min(buffer.len());
        if let Some(b_index) = buffer[buffer_start_index..]
            .iter()
            .position(|&bv| bv.map_or(true, |buffer_value| row_value > buffer_value))
        {
            buffer[b_index + buffer_start_index] = Some(row_value);
            buffer[b_index + buffer_start_index + 1..].fill(None);
        }
    });

    buffer
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| v.unwrap() as usize * 10_usize.pow(i as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(357, part_1(input));
    }

    #[test]
    fn test_max() {
        let row = vec![8, 1, 9];
        assert_eq!(89, find_max_joltage(&row, 2));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(3121910778619, part_2(input));
    }
}
