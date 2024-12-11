use std::collections::HashMap;
use std::fs;

fn main() {
    let input = get_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}");
}

fn get_file(path: &str) -> String{
    fs::read_to_string(path).unwrap()
}

fn part_1(input: String) -> usize {
    let mut memo:HashMap<(usize, usize), usize> = HashMap::new();
    input.split(" ").map(|num_as_str| num_as_str.parse::<usize>().unwrap()).map(|num| do_it(0, num, &mut memo)).sum()
}

fn part_2(input: String) -> usize {
    let mut memo:HashMap<(usize, usize), usize> = HashMap::new();
    input.split(" ").map(|num_as_str| num_as_str.parse::<usize>().unwrap()).map(|num| do_it(0, num, &mut memo)).sum()
}

fn do_it(count: usize, num_on_stone: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if map.contains_key(&(count, num_on_stone)) {
        return *map.get(&(count,num_on_stone)).unwrap()
    }

    if count == 75 {
        return 1
    }
    let mut total_stones = 0;

    let num_as_string = num_on_stone.to_string();
    if num_on_stone == 0 {
        let total = do_it(count+1, 1, map);

        map.insert((count+1, 1), total);

        total_stones += total;
    } else if num_as_string.len() % 2 == 0{
        let half = num_as_string.len() / 2;
        let part_1 = &num_as_string[..half].parse::<usize>().unwrap();
        let part_2 = &num_as_string[half..].parse::<usize>().unwrap();

        let p1t = do_it(count+1, *part_1, map);

        map.insert((count+1, *part_1), p1t);

        let p2t = do_it(count+1, *part_2, map);

        map.insert((count+1, *part_2), p2t);

        total_stones += p1t;
        total_stones += p2t;
    } else {
        let total = do_it(count+1, num_on_stone * 2024, map);

        map.insert((count+1, num_on_stone * 2024), total);

        total_stones += total;
    }

    total_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(55312, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }

    #[test]
    fn split_string() {
        let num_as_string = String::from("1234");
        let half = num_as_string.len() / 2;
        let part_1 = &num_as_string[..half];
        let part_2 = &num_as_string[half..];
        assert_eq!("12", part_1);
        assert_eq!("34", part_2);
    }
}