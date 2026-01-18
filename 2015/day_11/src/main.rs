fn main() {
    let input = "cqjxjnds";
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_1("cqjxxyzz"));
}

fn part_1(input: &str) -> String {
    let mut current = input.to_string();
    loop {
        current = increment_password(&current);
        if is_password_valid(&current) {
            return current;
        }
    }
}

fn increment_password(password: &str) -> String {
    let mut bytes = password.as_bytes().to_vec();

    for byte in bytes.iter_mut().rev() {
        if *byte == b'z' {
            *byte = b'a';
        } else {
            *byte += 1;
            break;
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn is_password_valid(password: &str) -> bool {
    let bytes = password.as_bytes();

    // Rule 1: Passwords must include one increasing straight of at least three letters.
    let has_straight = bytes
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);

    if !has_straight {
        return false;
    }

    // Rule 2: Passwords may not contain the letters i, o, or l.
    let has_invalid = bytes
        .iter()
        .any(|&b| b == b'i' || b == b'o' || b == b'l');

    if has_invalid {
        return false;
    }

    // Rule 3: Passwords must contain at least two different, non-overlapping pairs of letters.
    let mut pairs = 0;
    let mut i = 0;
    while i < bytes.len() - 1 {
        if bytes[i] == bytes[i + 1] {
            pairs += 1;
            i += 2; // Skip next to ensure non-overlapping
        } else {
            i += 1;
        }
    }

    pairs >= 2
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
    fn test_part_1_examples() {
        assert_eq!("abcdffaa", part_1("abcdefgh"));
        assert_eq!("ghjaabcc", part_1("ghijklmn"));
    }

    #[test]
    fn test_increment_password() {
        assert_eq!("abcdffab", increment_password("abcdffaa"));
        assert_eq!("abcdffba", increment_password("abcdffaz"));
    }
}
