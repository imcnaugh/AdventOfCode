fn main() {
    let input = include_str!("../resource/input.txt");
    let result = part_1(&input);
    println!("Part 1: {}", result);
}

fn part_1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        let result = part_1(&input);
        assert_eq!(result, "");
    }
}
