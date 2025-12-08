use std::collections::HashSet;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input, 1000));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str, to_connect: usize) -> usize {
    let boxes: Vec<JunctionBox> = input.lines().map(JunctionBox::from).collect();

    let mut distances: Vec<(String, String, f64)> = Vec::new();

    for (i, box_a) in boxes.iter().enumerate() {
        ((i + 1)..boxes.len()).for_each(|b_index| {
            let box_b = &boxes[b_index];
            distances.push((
                box_a.id().clone(),
                box_b.id().clone(),
                box_a.distance(box_b),
            ));
        })
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<HashSet<String>> = Vec::new();
    let mut distances_iter = distances.iter();

    (0..to_connect).for_each(|_| {
        let (a, b, _) = distances_iter.next().unwrap();

        let already_in_same_circuit = &circuits
            .iter()
            .any(|circuit| circuit.contains(a) && circuit.contains(b));
        if !already_in_same_circuit {
            let a_circuit_index = circuits.iter().position(|circuit| circuit.contains(a));
            let b_circuit_index = circuits.iter().position(|circuit| circuit.contains(b));

            match (a_circuit_index, b_circuit_index) {
                (Some(a_circuit_index), Some(b_circuit_index)) => {
                    let b = circuits[b_circuit_index].clone();
                    let a = &mut circuits[a_circuit_index];
                    a.extend(b);
                    circuits.remove(b_circuit_index);
                }
                (Some(a_circuit_index), None) => {
                    let a = &mut circuits[a_circuit_index];
                    a.insert(b.clone());
                }
                (None, Some(b_circuit_index)) => {
                    let b = &mut circuits[b_circuit_index];
                    b.insert(a.clone());
                }
                (None, None) => {
                    circuits.push(HashSet::from([a.clone(), b.clone()]));
                }
            }
        }
    });

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    (0..3c).map(|i| circuits[i].len()).product()
}

fn part_2(input: &str) -> usize {
    let boxes: Vec<JunctionBox> = input.lines().map(JunctionBox::from).collect();

    let mut distances: Vec<(String, String, f64, usize)> = Vec::new();

    for (i, box_a) in boxes.iter().enumerate() {
        ((i + 1)..boxes.len()).for_each(|b_index| {
            let box_b = &boxes[b_index];
            distances.push((
                box_a.id().clone(),
                box_b.id().clone(),
                box_a.distance(box_b),
                box_a.x * box_b.x,
            ));
        })
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<HashSet<String>> = Vec::new();
    let mut last: usize = 0;

    for (a, b, _, x) in distances.iter() {
        let already_in_same_circuit = &circuits
            .iter()
            .any(|circuit| circuit.contains(a) && circuit.contains(b));
        if !already_in_same_circuit {
            let a_circuit_index = circuits.iter().position(|circuit| circuit.contains(a));
            let b_circuit_index = circuits.iter().position(|circuit| circuit.contains(b));

            last = *x;

            match (a_circuit_index, b_circuit_index) {
                (Some(a_circuit_index), Some(b_circuit_index)) => {
                    let b = circuits[b_circuit_index].clone();
                    let a = &mut circuits[a_circuit_index];
                    a.extend(b);
                    circuits.remove(b_circuit_index);
                }
                (Some(a_circuit_index), None) => {
                    let a = &mut circuits[a_circuit_index];
                    a.insert(b.clone());
                }
                (None, Some(b_circuit_index)) => {
                    let b = &mut circuits[b_circuit_index];
                    b.insert(a.clone());
                }
                (None, None) => {
                    circuits.push(HashSet::from([a.clone(), b.clone()]));
                }
            }
        }
    }

    last
}

struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for JunctionBox {
    fn from(value: &str) -> Self {
        let parts: Vec<usize> = value
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }
}

impl JunctionBox {
    pub fn id(&self) -> String {
        format!("{}-{}-{}", self.x, self.y, self.z)
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let x_diff = self.x as f64 - other.x as f64;
        let y_diff = self.y as f64 - other.y as f64;
        let z_diff = self.z as f64 - other.z as f64;

        let flat_dist = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
        (flat_dist.powi(2) + z_diff.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt"), 10), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 25272);
    }
}
