use regex::Regex;

fn main() {
    let input = include_str!("../resources/input.txt");
    let rx = Regex::new(r"(\d+), (\d+)").unwrap();
    let parts = rx.captures(input).unwrap();
    let row = parts.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let col = parts.get(2).unwrap().as_str().parse::<usize>().unwrap();

    let mult = 252533usize;
    let divi = 33554393usize;

    let start_of_row_num = (1..=row).enumerate().map(|(x, _)| x).sum::<usize>() + 1;
    let index = (1..col).fold(start_of_row_num, |acc, x| acc + (row + x));

    let code = (1..index).fold(20151125usize, |acc, _| (acc * mult) % divi);
    println!("code: {code}");
}
