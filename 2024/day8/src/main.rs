use std::fs;

fn main() {
    let input = get_file_as_string("resource/input.txt");
    let result = part_1(input);
    println!("{}", result);
}

fn get_file_as_string(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
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
        let input = get_file_as_string("resource/test.txt");
        let result = part_1(input);
        assert_eq!(0, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_file_as_string("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }
}