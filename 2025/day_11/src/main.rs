use std::collections::HashMap;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let rack = Rack::from(input);
    rack.basic_dfs("you", "out")
}

fn part_2(input: &str) -> usize {
    let rack = Rack::from(input);
    let mut memo = HashMap::new();
    let svr_to_fft = rack.dfs_with_memo("svr", "fft", &mut memo);
    memo.clear();
    let fft_to_dac = rack.dfs_with_memo("fft", "dac", &mut memo);
    memo.clear();
    let dac_to_out = rack.dfs_with_memo("dac", "out", &mut memo);
    dac_to_out * svr_to_fft * fft_to_dac
}

struct Rack {
    nodes: HashMap<String, Vec<String>>,
}

impl Rack {
    fn basic_dfs(&self, current_node: &str, exit_node: &str) -> usize {
        if current_node == exit_node {
            return 1;
        }
        if let Some(children) = self.nodes.get(current_node) {
            children
                .iter()
                .map(|child| self.basic_dfs(child, exit_node))
                .sum()
        } else {
            0
        }
    }

    fn dfs_with_memo<'a>(
        &'a self,
        current_node: &'a str,
        exit_node: &str,
        mut memo: &mut HashMap<String, usize>,
    ) -> usize {
        if current_node == exit_node {
            return 1;
        }

        if let Some(paths) = memo.get(current_node) {
            return paths.clone();
        }

        if let Some(children) = self.nodes.get(current_node) {
            let paths = children
                .iter()
                .map(|child| self.dfs_with_memo(child, exit_node, &mut memo))
                .sum();
            memo.insert(current_node.to_string(), paths);
            paths
        } else {
            0
        }
    }
}

impl From<&str> for Rack {
    fn from(value: &str) -> Self {
        let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
        value.lines().for_each(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let node = parts[0].to_string();
            let children: Vec<String> = parts[1]
                .trim()
                .split(" ")
                .map(|s| s.trim().to_string())
                .collect();
            nodes.entry(node).or_insert(children);
        });
        Self { nodes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test_part_1.txt");
        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../resource/test_part_2.txt");
        assert_eq!(part_2(input), 2);
    }
}
