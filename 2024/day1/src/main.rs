use std::fs;

fn main() {
    let data = fs::read_to_string("resource/input.txt").unwrap();

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

    let mut diff: usize = 0;

    for index in 0..left_list.len() {
        if right_list[index] < left_list[index] {
            diff += left_list[index] - right_list[index];
        } else {
            diff += right_list[index] - left_list[index];
        }
    }

    println!("{diff}");
}
