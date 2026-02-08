fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let packages = parse_input(input);
    let total: usize = packages.iter().sum();
    let compartment_total = total / 3;
    println!("Sum of packages: {}", total);
    println!("Compartment total: {}", compartment_total);
    println!("pack: {:?}", packages);

    let mut idk: Vec<Vec<usize>> = Vec::new();
    get_elements_to_sum_target(compartment_total, &vec![], &packages, &mut idk);
    let min_elements = idk.iter().map(|v| v.len()).min().unwrap_or(0);
    let min_set = idk
        .iter()
        .filter(|v| v.len() == min_elements)
        .map(|v| v.iter().product())
        .min()
        .unwrap_or(0);
    min_set
}

fn get_elements_to_sum_target(
    target: usize,
    current: &Vec<usize>,
    elements: &Vec<usize>,
    ret_val: &mut Vec<Vec<usize>>,
) {
    let current_total: usize = current.iter().sum();
    let t: isize = target as isize - current_total as isize;
    if t == 0 {
        ret_val.push(current.to_vec());
        return;
    }
    if t < 0isize {
        return;
    }

    for (index, element) in elements.iter().enumerate() {
        let mut next_current = current.clone();
        next_current.push(*element);
        let next_elements = elements[index + 1..].to_vec();
        get_elements_to_sum_target(target, &next_current, &next_elements, ret_val);
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut packages: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    packages.sort();
    packages.reverse();
    packages
}
