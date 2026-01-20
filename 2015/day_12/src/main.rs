use regex::Regex;
use serde_json::Value;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"(-?\d+)").unwrap();
    let captures = regex.captures_iter(input);
    captures.fold(0, |acc, cap| acc + cap[1].parse::<i32>().unwrap_or(0))
}

fn part_2(input: &str) -> i32 {
    let as_json: Value = serde_json::from_str(input).unwrap();
    get_total_from_json_object(&as_json)
}

fn get_total_from_json_object(json_object: &Value) -> i32 {
    match json_object {
        Value::Object(o) => {
            if o.iter().any(|(_, v)| v == "red") {
                0
            } else {
                o.values().map(|v| get_total_from_json_object(v)).sum()
            }
        },
        Value::Number(n) => n.as_i64().unwrap_or(0) as i32,
        Value::Array(a) => a.iter().map(|v| get_total_from_json_object(v)).sum(),
        _ => 0,
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2_1() {
        let input = "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}";
        assert_eq!(part_2(input), 0);
    }
}