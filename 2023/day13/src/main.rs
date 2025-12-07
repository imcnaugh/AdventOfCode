fn main() {
    let input = include_str!("../resource/input.txt");
    // println!("{}", part_1(input));
    println!("{}", part_2(input));
}

// fn part_1(input: &str) -> usize {
//     input.split("\n\n").map(|map| find_reflection(map)).sum()
// }

fn part_2(input: &str) -> usize {
    input.split("\n\n").map(|map| find_reflection(map)).sum()
}

fn find_reflection(map_str: &str) -> usize {
    if let Some(row) = find_rows_above_reflection_point(map_str) {
        return row * 100;
    }
    if let Some(column) = find_columns_before_reflection_point(map_str) {
        return column;
    }
    panic!("Reflection point not found!");
}

fn find_rows_above_reflection_point(map_str: &str) -> Option<usize> {
    let line_itter = map_str.lines().collect::<Vec<_>>();

    for i in 1..line_itter.len() {

        let mut out = 0;
        let mut smudge_count = 0;

        loop {
            let line_index_below = i + out;
            let line_index_above = i as i32 - 1 - out as i32;

            if line_index_above < 0 || line_index_below >= line_itter.len() {
                if smudge_count == 0 {
                    break;
                }
                return Some(i);
            }

            if line_itter[line_index_below] != line_itter[line_index_above as usize] {
                smudge_count += 1;

                let diff_count = line_itter[line_index_below].chars().zip(line_itter[line_index_above as usize].chars()).filter(|&(a, b)| a != b).count();
                if smudge_count > 1 || diff_count > 1 {
                    break;
                }
            }
            out += 1;
        }
    }

    None
}

fn find_columns_before_reflection_point(map_str: &str) -> Option<usize> {
    let lines: Vec<Vec<char>> = map_str.lines().map(|l| l.chars().collect()).collect();
    let len = lines[0].len();

    let idk = (0..len).map(|i| {
        lines.iter().map(|line| {
            line[i]
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    find_rows_above_reflection_point(&idk)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(405, part_1(include_str!("../resource/test.txt")));
    // }

    #[test]
    fn test_part_2() {
        assert_eq!(400, part_2(include_str!("../resource/test.txt")));
    }
}