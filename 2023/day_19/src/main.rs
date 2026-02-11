use std::collections::HashMap;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);
    let workflows_map: HashMap<String, Workflow> = workflows
        .into_iter()
        .map(|workflow| (workflow.id.clone(), workflow))
        .collect();

    parts
        .iter()
        .filter(|part| {
            let mut current = workflows_map.get("in").unwrap();
            loop {
                let matched_condition = current
                    .conditions
                    .iter()
                    .find(|(w, r)| part.match_condition(w));
                let next = if let Some((_, result)) = matched_condition {
                    result
                } else {
                    &current.default_condition.clone()
                };
                if next == "A" {
                    return true;
                } else if next == "R" {
                    return false;
                }
                current = workflows_map.get(next).unwrap();
            }
        })
        .map(|part| part.value())
        .sum()
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let workflows = parts[0]
        .lines()
        .map(|line| {
            let rx = regex::Regex::new(r#"^(.*)\{(.*)}"#).unwrap();
            let captures = rx.captures(line).unwrap();
            let id: String = captures.get(1).unwrap().as_str().to_string();
            let mut caps_iter = captures.get(2).unwrap().as_str().split(",");
            let caps_iter_len = caps_iter.clone().count().clone();
            let conditions: Vec<((String, String, usize), String)> = caps_iter
                .clone()
                .take(caps_iter_len - 1)
                .map(|condition| {
                    let crx = regex::Regex::new(r#"([a-z]+)([<>])(\d+):(.*)"#).unwrap();
                    let con_captures = crx.captures(condition).unwrap();
                    let condition: (String, String, usize) = (
                        con_captures.get(1).unwrap().as_str().to_string(),
                        con_captures.get(2).unwrap().as_str().to_string(),
                        con_captures
                            .get(3)
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                    );
                    let result = con_captures.get(4).unwrap().as_str().to_string();
                    (condition, result)
                })
                .collect();
            let default_condition = caps_iter.last().unwrap();
            Workflow::new(id, conditions, default_condition.to_string())
        })
        .collect();

    let parts = parts[1]
        .lines()
        .map(|line| {
            let xrx = regex::Regex::new(r#"x=(\d+)"#).unwrap();
            let mrx = regex::Regex::new(r#"m=(\d+)"#).unwrap();
            let arx = regex::Regex::new(r#"a=(\d+)"#).unwrap();
            let srx = regex::Regex::new(r#"s=(\d+)"#).unwrap();
            let x = xrx
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let m = mrx
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let a = arx
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let s = srx
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            Part::new(x, m, a, s)
        })
        .collect();
    (workflows, parts)
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
    }

    pub fn match_condition(&self, (condition, value, result): &(String, String, usize)) -> bool {
        let v = match condition.as_str() {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid condition"),
        };
        match value.as_str() {
            "<" => v < *result,
            ">" => v > *result,
            _ => panic!("Invalid value"),
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Workflow {
    id: String,
    conditions: Vec<((String, String, usize), String)>,
    default_condition: String,
}

impl Workflow {
    fn new(
        id: String,
        conditions: Vec<((String, String, usize), String)>,
        default_condition: String,
    ) -> Self {
        Self {
            id,
            conditions,
            default_condition,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(part_1(input), 19114);
    }
}
