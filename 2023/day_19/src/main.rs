use std::collections::HashMap;

fn main() {
    let input = include_str!("../resource/input.txt");
    // println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
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

fn part_2(input: &str) -> u128 {
    let (workflows, _) = parse_input(input);
    let workflows: HashMap<String, Workflow> = workflows
        .into_iter()
        .map(|workflow| (workflow.id.clone(), workflow))
        .collect();

    recurse("in", 1, 4000, 1, 4000, 1, 4000, 1, 4000, &workflows)
}

fn recurse(
    workflow_id: &str,
    x_min: u128,
    x_max: u128,
    m_min: u128,
    m_max: u128,
    a_min: u128,
    a_max: u128,
    s_min: u128,
    s_max: u128,
    workflows: &HashMap<String, Workflow>,
) -> u128 {
    if workflow_id == "A" {
        if x_max < x_min || m_max < m_min || a_max < a_min || s_max < s_min {
            return 0;
        }
        return (x_max - x_min + 1)
            * (m_max - m_min + 1)
            * (a_max - a_min + 1)
            * (s_max - s_min + 1);
    }
    if workflow_id == "R" {
        return 0;
    }

    let mut x_max = x_max.clone();
    let mut x_min = x_min.clone();
    let mut m_max = m_max.clone();
    let mut m_min = m_min.clone();
    let mut a_max = a_max.clone();
    let mut a_min = a_min.clone();
    let mut s_max = s_max.clone();
    let mut s_min = s_min.clone();
    let current_workflow = workflows.get(workflow_id).unwrap();

    let condition_sums: u128 = current_workflow
        .conditions
        .iter()
        .map(|(condition, result)| {
            let this_x_min = if condition.0 == "x" && condition.1 == ">" {
                condition.2 as u128 + 1
            } else {
                x_min
            };
            let this_x_max = if condition.0 == "x" && condition.1 == "<" {
                condition.2 as u128 - 1
            } else {
                x_max
            };
            let this_m_min = if condition.0 == "m" && condition.1 == ">" {
                condition.2 as u128 + 1
            } else {
                m_min
            };
            let this_m_max = if condition.0 == "m" && condition.1 == "<" {
                condition.2 as u128 - 1
            } else {
                m_max
            };
            let this_a_min = if condition.0 == "a" && condition.1 == ">" {
                condition.2 as u128 + 1
            } else {
                a_min
            };
            let this_a_max = if condition.0 == "a" && condition.1 == "<" {
                condition.2 as u128 - 1
            } else {
                a_max
            };
            let this_s_min = if condition.0 == "s" && condition.1 == ">" {
                condition.2 as u128 + 1
            } else {
                s_min
            };
            let this_s_max = if condition.0 == "s" && condition.1 == "<" {
                condition.2 as u128 - 1
            } else {
                s_max
            };

            let s = recurse(
                &result, this_x_min, this_x_max, this_m_min, this_m_max, this_a_min, this_a_max,
                this_s_min, this_s_max, workflows,
            );

            if condition.0 == "x" && condition.1 == ">" {
                x_max = x_max.min(condition.2 as u128)
            }
            if condition.0 == "x" && condition.1 == "<" {
                x_min = x_min.max(condition.2 as u128)
            }
            if condition.0 == "m" && condition.1 == ">" {
                m_max = m_max.min(condition.2 as u128)
            }
            if condition.0 == "m" && condition.1 == "<" {
                m_min = m_min.max(condition.2 as u128)
            }
            if condition.0 == "a" && condition.1 == ">" {
                a_max = a_max.min(condition.2 as u128)
            }
            if condition.0 == "a" && condition.1 == "<" {
                a_min = a_min.max(condition.2 as u128)
            }
            if condition.0 == "s" && condition.1 == ">" {
                s_max = s_max.min(condition.2 as u128)
            }
            if condition.0 == "s" && condition.1 == "<" {
                s_min = s_min.max(condition.2 as u128)
            }

            s
        })
        .sum();
    let default_sum = recurse(
        &current_workflow.default_condition,
        x_min,
        x_max,
        m_min,
        m_max,
        a_min,
        a_max,
        s_min,
        s_max,
        workflows,
    );
    condition_sums + default_sum
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

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(part_2(input), 167409079868000);
    }
}
