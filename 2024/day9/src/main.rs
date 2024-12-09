use std::fs;

fn main() {
    let input = get_file("resource/input.txt");
    let result = part_1(input);
    println!("{result}");
}

fn get_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn part_1(input:String) -> usize {
    let chars = input.chars().map(|c| c.to_string().parse::<usize>().unwrap());

    let mut list: Vec<Option<usize>> = vec![];

    for (index, c) in chars.enumerate() {
        if index % 2 == 1 {
            for _ in 0..c {
                list.push(None);
            }
        } else {
            let file_index = index / 2;
            for _ in 0..c {
                list.push(Some(file_index));
            }
        }
    }

    let mut start_index = 0;
    let mut end_index = list.len() - 1;
    loop {
        if start_index >= end_index {
            break;
        }

        if list[start_index].is_none() && list[end_index].is_some() {
            list.swap(start_index, end_index);
            continue;
        }
        if list[start_index].is_some() {
            start_index += 1;
        }
        if list[end_index].is_none() {
            end_index -= 1;
        }
    }

    list.into_iter().enumerate().map(|(index, item)| -> usize {
        match item {
            Some(i) => index * i,
            None => 0
        }
    }).sum()
}

fn part_2(input:String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(1928, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result);
    }
}