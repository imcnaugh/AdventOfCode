use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufRead;

fn main() {
    let result = part_1("resource/input.txt");
    println!("{result}");
}



fn part_1(path: &str) -> usize {
    let mut result = 0;

    let (rules_raw, order) = get_data(path);

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    rules_raw.split("\n").for_each(|r| {
        let mut parts = r.split("|");
        let left = parts.next().unwrap().parse::<usize>().unwrap();
        let right = parts.next().unwrap().parse::<usize>().unwrap();
        if rules.contains_key(&left) {
            let mut current_vec = rules.get(&left).unwrap().clone();
            current_vec.push(right);
            rules.insert(left, current_vec);
        } else {
            rules.insert(left, vec![right]);
        }
    });

    let print_orders: Vec<Vec<usize>> = order
        .split("\n")
        .map(|o| -> Vec<usize> {
            o.split(",")
                .map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        }).collect();

    for po in print_orders {
        let mut in_correct_order = true;

        let mut already_seen_pages: HashSet<usize> = HashSet::new();

        for p in &po {
            let pair = rules.get(p);
            if let Some(pairs) = pair {
                for other in pairs {
                    if already_seen_pages.contains(other) {
                        in_correct_order = false;
                        break;
                    }
                }
            }
            already_seen_pages.insert(*p);
        }

        if in_correct_order {
            // println!("{:?}", po);
            // let mid_index = po.len() / 2;
            // result += po.get(mid_index).unwrap();
        } else {
            let mut ordered_pages: Vec<usize> = Vec::new();
            let mut remaining_pages : HashSet<usize> = po.into_iter().collect();

            loop {
                if remaining_pages.is_empty() {
                    break;
                }

                for page in &remaining_pages{
                    let deps = rules.get(page);
                    let mut next = true;
                    if let Some(deps) = deps {
                        for d in deps {
                            if remaining_pages.contains(d) {
                                next = false;
                            }
                        }
                    }

                    if next {
                        ordered_pages.push(*page);
                    }
                }

                remaining_pages.remove(ordered_pages.last().unwrap());
            }


            let mid_index = ordered_pages.len() / 2;
            result += ordered_pages.get(mid_index).unwrap();
        }
    }

    result
}

fn get_data(path: &str) -> (String, String) {
    let file_data = fs::read_to_string(path).unwrap();
    let mut parts = file_data.split("\n\n");
    (parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test(){
        let result = part_1("resource/test.txt");
        assert_eq!(143, result);
    }

    #[test]
    fn part_2_test() {
        let result = part_1("resource/test.txt");
        assert_eq!(123, result);
    }
}