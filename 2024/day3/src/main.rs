use std::fs;
use regex::Regex;

fn main() {
    let result = get_result_part_2("resource/input.txt");
    println!("answer is: {result}");
}

fn get_result_part_1(path_to_input: &str) -> usize {
    let data = fs::read_to_string(path_to_input).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = re.captures_iter(&*data)
        .map(|mut_line| mut_line.extract())
        .map(|(_, [d1, d2])| -> usize {
            println!("{d1} * {d2}");

            let d1 = d1.parse::<usize>().unwrap();
            let d2 = d2.parse::<usize>().unwrap();

            d1 * d2
        }).reduce(|a, b| a + b);
    result.unwrap()
}

fn get_result_part_2(path_to_input: &str) -> usize {
    let data = fs::read_to_string(path_to_input).unwrap();

    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))").unwrap();
    let mut on = true;

    let result = re.captures_iter(&*data)
        .map(|cap| cap.extract())
        .map(|(_, [cap])| -> usize {
            if cap.starts_with("don") {
                on = false;
            } else if cap.starts_with("do") {
                on = true;
            }

            if on && cap.starts_with("mul") {
                let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
                regex.captures(cap).map(|c| c.extract())
                    .map(|(_, [d1, d2])| -> usize {
                        let d1 = d1.parse::<usize>().unwrap();
                        let d2 = d2.parse::<usize>().unwrap();
                        d1 * d2
                    }).unwrap()
            } else {
                0
            }
        }).reduce(|a, b| a + b);

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = get_result_part_1("resource/test.txt");
        assert_eq!(161, result);
    }

    #[test]
    fn part_2() {
        let result = get_result_part_2("resource/test.txt");
        assert_eq!(48, result);
    }
}
