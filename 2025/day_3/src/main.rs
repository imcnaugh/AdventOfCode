fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    parse_input(input).iter().map(|row| max(row, 2)).sum()
}

fn part_2(input: &str) -> usize {
    parse_input(input).iter().map(|row| max(row, 12)).sum()
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn max(row: &Vec<usize>, output_len: usize) -> usize {
    let mut buffer: Vec<Option<usize>> = vec![None; output_len];

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
        .map(|&v| v.unwrap().to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
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
        assert_eq!(89, max(&row, 2));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(3121910778619, part_2(input));
    }
}
