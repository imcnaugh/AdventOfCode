use crate::item::{Item, ItemType};
use crate::player::Player;
use regex::Regex;

mod item;
mod player;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut boss = crate_boss(input);
    let (weapons, armors, rings) = parse_all_items();

    println!("{:?}, {:?}, {:?}", weapons, armors, rings);

    todo!()
}

// TODO fix this
fn will_player_win(player: Player, boss: Player) -> bool {
    let turns = vec![player, boss];
    let mut turn_iter = turns.iter().cycle();
    loop {
        let current_turn = turn_iter.next().unwrap();
        if current_turn.is_alive() {
            return false;
        }
    }
}

fn crate_boss(input: &str) -> Player {
    let hit_points_regex = Regex::new(r#"Hit Points: (\d+)"#).unwrap();
    let damage_regex = Regex::new(r#"Damage: (\d+)"#).unwrap();
    let armor_regex = Regex::new(r#"Armor: (\d+)"#).unwrap();

    let hit_points = hit_points_regex.captures(input).unwrap()[1]
        .parse::<i32>()
        .unwrap();
    let damage = damage_regex.captures(input).unwrap()[1]
        .parse::<i32>()
        .unwrap();
    let armor = armor_regex.captures(input).unwrap()[1]
        .parse::<i32>()
        .unwrap();

    let weapon = Item::new_weapon(String::from("boss weapon"), damage, armor, 0);
    Player::new(hit_points, weapon)
}

fn parse_all_items() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let (weapons, armors, rings) = (
        parse_items("./resources/weapons.txt", ItemType::Weapon),
        parse_items("./resources/armor.txt", ItemType::Armor),
        parse_items("./resources/rings.txt", ItemType::Ring),
    );
    (weapons, armors, rings)
}

fn parse_items(file: &str, item_type: ItemType) -> Vec<Item> {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    let file_text = std::fs::read_to_string(file).unwrap();
    file_text
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let create_fn = match item_type {
                ItemType::Weapon => Item::new_weapon,
                ItemType::Armor => Item::new_armor,
                ItemType::Ring => Item::new_ring,
            };
            create_fn(
                String::from(parts[0]),
                parts[1].parse::<i32>().unwrap(),
                parts[2].parse::<i32>().unwrap(),
                parts[3].parse::<i32>().unwrap(),
            )
        })
        .collect()
}
