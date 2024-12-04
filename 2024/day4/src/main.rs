use std::fs;

fn main() {
    let result = part_1("resource/input.txt");
    println!("{result}");
}

fn part_1(path: &str) -> usize {
    let text = fs::read_to_string(path).unwrap();
    let mut result = 0;

    let grid = text.split("\n").map(|line| -> Vec<char> {
        line.chars().into_iter().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let directions = [(1i32, -1i32), (1,0), (1,1), (0, 1), (-1,1), (-1,0), (-1, -1), (0,-1)];
    let expected = ['X', 'M', 'A', 'S'];


    for row in 0..grid.len(){
        let r = &grid[row];
        for col in 0..r.len() {
            'here: for d in directions {
                for (index, expected_char) in expected.iter().enumerate() {
                    let check_col = col as i32 + (index as i32 * d.0);
                    let row_check = row as i32 + (index as i32 * d.1);
                    if check_col < 0
                        || check_col >= r.len() as i32
                        || row_check < 0
                        || row_check >= grid.len() as i32
                        || grid[check_col as usize][row_check as usize] != *expected_char{
                        continue 'here;
                    }
                }
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let result = part_1("resource/test.txt");
        assert_eq!(18, result);
    }
}
