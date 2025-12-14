fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input.split(',').map(|s| hash_input(s.as_bytes())).sum()
}

fn part_2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::new(); 256];

    input.split(',').for_each(|inst| {
        if inst.chars().last().unwrap() == '-' {
            let label = inst[0..inst.len() - 1].to_string();
            let box_id = hash_input(&label.as_bytes());
            let box_ref = &mut boxes[box_id];
            let index_to_remove = box_ref.iter().position(|(c, _)| *c == label);
            if let Some(index) = index_to_remove {
                box_ref.remove(index);
            }
        } else {
            let label = inst[0..inst.len() - 2].to_string();
            let box_id = hash_input(&label.as_bytes());
            let leans_id = inst[inst.len() - 1..].parse::<u8>().unwrap();
            let box_ref = &mut boxes[box_id];
            let index_to_swap = box_ref.iter().position(|(c, _)| *c == label);
            if let Some(index) = index_to_swap {
                box_ref[index] = (label, leans_id);
            } else {
                box_ref.push((label, leans_id));
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i_2, b)| {
            b.iter()
                .enumerate()
                .map(|(i, c)| c.1 as usize * (i + 1) * (i_2 + 1))
                .sum::<usize>()
        })
        .sum()
}

fn hash_input(input: &[u8]) -> usize {
    input
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17) & 0b11111111)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 1320);
    }

    #[test]
    fn test_HASH() {
        assert_eq!(hash_input(String::from("HASH").as_bytes()), 52);
    }

    #[test]
    fn test_hash_label() {
        assert_eq!(hash_input(String::from("rn").as_bytes()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 145);
    }
}
