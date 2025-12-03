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
                .map(|C| C.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn max(row: &Vec<usize>, output_len: usize) -> usize {
    let mut buffer: Vec<Option<usize>> = vec![None; output_len];

    row.iter().enumerate().for_each(|(i, &n)| {
        let remaining = row.len() - i;
        let buffer_start_index = buffer.len() - remaining.min(buffer.len());
        let index_to_modify = buffer[buffer_start_index..].iter().position(|&bv| {
            if let Some(buffer_value) = bv {
                n > buffer_value
            } else {
                true
            }
        });
        if let Some(index) = index_to_modify {
            buffer[index + buffer_start_index] = Some(n);
            for x in index + 1 + buffer_start_index..buffer.len() {
                buffer[x] = None;
            }
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
