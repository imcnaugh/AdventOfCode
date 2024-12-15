use std::fs;
use std::io::{stdin, stdout, Read, Write};
use regex::Regex;

fn main() {
    let input = read_file("resource/input.txt");
    let result = part_2(input);
    println!("{result}");
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn parse_input_line(line: &str) -> (i64, i64, i64, i64){
    let r = Regex::new(r"p=(?<p1>.+),(?<p2>.+) v=(?<v1>.+),(?<v2>.+)").unwrap();

    let captures = r.captures(line).unwrap();

    let v1 = captures["v1"].parse::<i64>().unwrap();
    let v2 = captures["v2"].parse::<i64>().unwrap();
    let p1 = captures["p1"].parse::<i64>().unwrap();
    let p2 = captures["p2"].parse::<i64>().unwrap();

    (p1,p2,v1,v2)
}

fn part_1(input: String, width: i64, height: i64) -> usize {
    let mut quardrents = [0usize,0,0,0];

    let robots: Vec<(i64, i64, i64, i64)> = input.lines().map(|l| parse_input_line(l)).collect();

    let mid_width = (width - 1)/2;
    let mid_height = (height - 1)/2;
    let seconds: i64 = 100;

    robots.iter().for_each(|r| {
        let (px, py, vx, vy) = r;
        let final_x = ((*vx * seconds) + *px).rem_euclid(width);
        let final_y = ((*vy * seconds) + *py).rem_euclid(height);

        if final_x != mid_width && final_y != mid_height {
            let left = final_x < mid_width;
            let top = final_y < mid_height;

            if top && left {
                quardrents[0] += 1;
            }
            if top && !left {
                quardrents[1] += 1;
            }
            if !top && left{
                quardrents[2] += 1;
            }
            if !top && !left {
                quardrents[3] += 1;
            }
        }
    });

    quardrents[0] * quardrents[1] * quardrents[2] * quardrents[3]
}


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn part_2(input: String) -> usize {
    let width = 101;
    let height = 103;

    let robots: Vec<(i64, i64, i64, i64)> = input.lines().map(|l| parse_input_line(l)).collect();
    let mut seconds = 84;

    loop {
        let mut arr: Vec<Vec<char>> = vec![vec![' '; width]; height ];
        robots.iter().for_each(|r| {
            let (px, py, vx, vy) = r;
            let final_x = ((*vx * seconds) + *px).rem_euclid(width as i64);
            let final_y = ((*vy * seconds) + *py).rem_euclid(height as i64);
            arr[final_y as usize][final_x as usize] = '*';
        });

        &arr.into_iter().for_each(|row| {
            let row_as_string: String = row.into_iter().collect();
            println!("{row_as_string}")
        });
        println!("seconds: {seconds}");

        pause();
        seconds += 103;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("resource/test.txt");
        let result = part_1(input, 11, 7);
        assert_eq!(12, result)
    }

    #[test]
    fn test_part_2() {
        let input = read_file("resource/test.txt");
        let result = part_2(input);
        assert_eq!(0, result)
    }

    #[test]
    fn simple_mod_test(){
        let x = 5;
        let v = -2;
        let ticks = 100;
        let width = 10;
        let result: i64 = ((v * ticks) + x) % width;
        let result: i64 = result.abs();
        assert_eq!(5, result)

    }
}