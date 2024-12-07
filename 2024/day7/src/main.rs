use std::fs;

fn main() {
    let input = get_string_from_file("resource/input.txt");
    let result = part_2(input);
    println!("{result}");
}

fn get_string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn part_1(input: String) -> usize {
    input.lines().map(|line| -> usize {
        let mut parts = line.split(":");
        let total = parts.next().unwrap().parse::<usize>().unwrap();
        let numbers: Vec<usize> = parts.next().unwrap().trim()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let operations = [
            |a: usize, b:usize| -> usize {a + b},
            |a: usize, b:usize| -> usize {a * b},
        ];

        if get_result(&numbers[..], 0, &operations[..], total){
            return total
        }
        0
    }).reduce(|a, b| a+b).unwrap()
}

fn get_result(nums: &[usize], current_total: usize, operations: &[fn(usize, usize)-> usize], expected: usize) -> bool {
    if nums.is_empty() {
        if current_total == expected {
            return true;
        } else {
            return false;
        }
    }
    if current_total > expected {
        return false;
    }

    let a = current_total;
    let b = nums[0];
    for op in operations {
        let result = op(a, b);
        if get_result(&nums[1..], result, &operations, expected) {
            return true;
        }
    }

    false
}

fn part_2(input: String) -> usize {
    input.lines().map(|line| -> usize {
        let mut parts = line.split(":");
        let total = parts.next().unwrap().parse::<usize>().unwrap();
        let numbers: Vec<usize> = parts.next().unwrap().trim()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let operations = [
            |a: usize, b:usize| -> usize {a + b},
            |a: usize, b:usize| -> usize {a * b},
            |a: usize, b:usize| -> usize {
                format!("{a}{b}").parse::<usize>().unwrap()
            },
        ];

        if get_result(&numbers[..], 0, &operations[..], total){
            return total
        }
        0
    }).reduce(|a, b| a+b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_string_from_file("resource/test.txt");
        let result = part_1(input);
        assert_eq!(3749, result);
    }

    #[test]
    fn test_part_2() {
        let input = get_string_from_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(11387, result);
    }
}

