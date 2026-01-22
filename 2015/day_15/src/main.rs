use regex::Regex;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let inputs = parse_inputs(input);

    fn search(
        ingredients: &[Ingredient],
        idx: usize,
        remaining: i32,
        amounts: &mut Vec<i32>,
        best: &mut usize,
    ) {
        // ... existing code ...
        if idx + 1 == ingredients.len() {
            // Last ingredient gets whatever is left so the sum is exactly 100
            amounts[idx] = remaining;

            let pairs: Vec<(i32, &Ingredient)> = amounts
                .iter()
                .copied()
                .zip(ingredients.iter())
                .map(|(a, ing)| (a, ing))
                .collect();

            let calories = pairs.iter().map(|(amount, ing)| amount * ing.calories).sum::<i32>();
            if calories != 500 {
                return;
            }

            *best = (*best).max(get_score_part_1(&pairs));
            return;
        }

        for a in 0..=remaining {
            amounts[idx] = a;
            search(ingredients, idx + 1, remaining - a, amounts, best);
        }
        // ... existing code ...
    }

    let mut best = 0usize;
    let mut amounts = vec![0i32; inputs.len()];
    search(&inputs, 0, 100, &mut amounts, &mut best);
    best
}

fn parse_inputs(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(r#"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Ingredient::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
                captures[4].parse().unwrap(),
                captures[5].parse().unwrap(),
            )
        })
        .collect()
}


fn get_score_part_1(ingredients: &Vec<(i32, &Ingredient)>) -> usize {
    ingredients
        .iter()
        .fold(vec![0i32; 4], |acc, (amount, ingredient)| {
            vec![
                acc[0] + amount * ingredient.capacity,
                acc[1] + amount * ingredient.durability,
                acc[2] + amount * ingredient.flavor,
                acc[3] + amount * ingredient.texture,
            ]
        })
        .iter()
        .map(|&score| score.max(0) as usize)
        .product()
}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let parsed = parse_inputs(input);
        let test = vec![(44, &parsed[0]), (56, &parsed[1])];
        assert_eq!(get_score_part_1(&test), 62842880);
    }
}
