fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line
                .split("x")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            let side_a = parts[0] * parts[1];
            let side_b = parts[0] * parts[2];
            let side_c = parts[1] * parts[2];
            let min = side_a.min(side_b.min(side_c));

            (2 * side_a) + (2 * side_b) + (2 * side_c) + min
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line
                .split("x")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            let side_a = parts[0];
            let side_b = parts[1];
            let side_c = parts[2];
            let max = side_a.max(side_b.max(side_c));
            let bow_length = side_a * side_b * side_c;

            (2 * side_a) + (2 * side_b) + (2 * side_c) - (2 * max) + bow_length
        })
        .sum()
}
