use std::collections::HashSet;

fn main() {
    let input = "cqjxjnds";
    println!("Part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> String {
    let mut current = input.to_string();
    loop {
        let next = increment_password(&current);
        if is_password_valid(&next) {
            return next;
        }
        current = next;
    }
}

fn increment_password(password: &str) -> String {
    let mut char_array = password.chars().map(|c| c as u8).rev().collect::<Vec<u8>>();

    for mut c in char_array.iter_mut() {
        if c == &122 {
            *c = 97;
        } else {
            *c += 1;
            break;
        }
    }

    char_array
        .iter()
        .rev()
        .map(|&c| c as char)
        .collect::<String>()
}

fn is_password_valid(password: &str) -> bool {
    let char_array = password.chars().map(|c| c as u8).collect::<Vec<u8>>();

    let invalid_chars = vec!['i', 'o', 'l']
        .iter()
        .map(|&c| c as u8)
        .collect::<HashSet<u8>>();

    let mut contain_invalid =
        invalid_chars.contains(&char_array[0]) || invalid_chars.contains(&char_array[1]);
    let mut contain_increasing_string = false;
    let mut overlapping_pairs = if char_array[0] == char_array[1] { 1 } else { 0 };

    (2..char_array.len()).for_each(|i| {
        let partial_string = &char_array[i - 2..=i];
        if invalid_chars.contains(&partial_string[2]) {
            contain_invalid = true;
        }
        if partial_string[1] == (partial_string[0] + 1)
            && partial_string[2] == (partial_string[1] + 1)
        {
            contain_increasing_string = true;
        }
        if partial_string[1] == partial_string[2] && partial_string[0] != partial_string[1] {
            overlapping_pairs += 1;
        }
    });

    !contain_invalid && contain_increasing_string && overlapping_pairs >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_password_valid_1() {
        assert!(!is_password_valid("hijklmmn"));
    }

    #[test]
    fn test_is_password_valid_2() {
        assert!(is_password_valid("ghjaabcc"));
    }

    #[test]
    fn get_ascii_values() {
        let a_value = 'a' as u8;
        let z_value = 'z' as u8;
        println!("a {}", a_value);
        println!("z {}", z_value);
    }

    #[test]
    fn increment_password_1() {
        assert_eq!("abcdffab", increment_password("abcdffaa"));
        assert_eq!("abcdffba", increment_password("abcdffaz"));
    }
}
