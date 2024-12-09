use std::fs;

fn main() {
    let input = get_file("resource/input.txt");
    let result = part_2(input);
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

    let mut rev_index = list.len() - 1;
    'all: while rev_index >= 0 {
        let mut current_file_id = 0;
        let mut current_file_size = 0;

        if let Some(pid) = list[rev_index] {
            current_file_id = pid;
            current_file_size = 1;
            loop {
                if rev_index == 0 {
                    break 'all;
                }
                rev_index -= 1;
                if rev_index == 1 || list[rev_index].is_none() {
                    break;
                }
                if let Some(cid) = list[rev_index] {
                    if cid != pid {
                        break;
                    }
                    current_file_size += 1;
                }
            }
            // print_list(&list);

            'here: for fi in 0..rev_index {
                if list[fi].is_none() {
                    for cj in 0..current_file_size {
                        if list[fi + cj].is_some() {
                            continue 'here;
                        }
                    }
                    for n in 0..current_file_size {
                        list.swap(fi + n, rev_index+n + 1);
                    }
                    break;
                }
            }
        } else {
            rev_index -= 1;
        }
    }

    print_list(&mut list);

    list.into_iter().enumerate().map(|(index, item)| -> usize {
        match item {
            Some(i) => index * i,
            None => 0
        }
    }).sum()
}

fn print_list(list: &Vec<Option<usize>>) {
    for x in list {
        match x {
            Some(n) => print!("{n}"),
            None => print!(".")
        }
    }
    println!()
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
        assert_eq!(2858, result);
    }
}