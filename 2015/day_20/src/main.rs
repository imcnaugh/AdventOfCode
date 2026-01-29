const GOAL: usize = 34_000_000;

fn main() {
    let limit = GOAL / 10;

    let mut presents = vec![0usize; limit + 1];

    for elf in 1..=limit {
        let gift = elf * 11;
        for house in (elf..=limit).step_by(elf).take(50) {
            presents[house] += gift;
        }
    }

    let answer = (1..=limit).find(|&h| presents[h] >= GOAL).unwrap();
    println!("{answer}");
}
