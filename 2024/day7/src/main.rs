use std::fs;

fn main() {
    let input = get_string_from_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}");
}

fn get_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn part_1(input: String) -> usize {
    todo!()
}

fn part_2(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_string_from_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(0, result);
    }

    fn test_part_2() {
        let input = get_string_from_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }
}

