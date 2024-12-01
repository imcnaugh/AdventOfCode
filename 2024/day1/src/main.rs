use std::collections::HashMap;
use std::fs;

fn main() {
    let result = find_result("resource/input.txt");
    println!("{result}");
}

fn find_result(path_string: &str) -> usize {
    let data = fs::read_to_string(path_string).unwrap();

    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    data.split("\n").for_each(|row| {
        let mut parts = row.split("   ");
        let left: usize = parts.next().unwrap().parse().unwrap();
        let right: usize = parts.next().unwrap().parse().unwrap();

        left_list.push(left);
        right_list.push(right);
    });

    left_list.sort();
    right_list.sort();

    let mut left_memo = HashMap::<usize, usize>::new();
    let mut right_memo = HashMap::<usize, usize>::new();

    let mut result: usize = 0;

    left_list.iter().for_each(|i| {
        match left_memo.get(i) {
            Some(r) => result += r,
            None => {
                let count = get_count(*i, &right_list);
                let count = count * i;
                left_memo.insert(*i, count);
                result += count;
            }
        }
    });

    // right_list.iter().for_each(|i| {
    //     match right_memo.get(i) {
    //         Some(r) => result += r,
    //         None => {
    //             let count = get_count(*i, &left_list);
    //             right_memo.insert(*i, count);
    //             result += count;
    //         }
    //     }
    // });

    result
}

fn get_count(i: usize, list: &Vec<usize>) -> usize {
    list.iter().filter(|&m| -> bool {
        i == *m
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = find_result("resource/test.txt");
        println!("{result}");

        assert_eq!(31, result);
    }
}