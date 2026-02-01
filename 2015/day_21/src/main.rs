use crate::item::ItemType::{Armor, Ring, Weapon};
use crate::item::{Item, ItemType};
use crate::player::Player;
use regex::Regex;

mod item;
mod player;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 2: {}", part_2(input));
}

fn part_2(input: &str) -> i32 {
    let boss = crate_boss(input);
    let player = Player::new(100);

    let weapons = parse_items(Weapon);
    let armors = parse_items(Armor);
    let rings = parse_items(Ring);

    get_max_cost_of_weapon(&player, &boss, &weapons, &armors, &rings).unwrap()
}

fn get_max_cost_of_weapon(
    player: &Player,
    boss: &Player,
    weapons: &Vec<Item>,
    armors: &Vec<Item>,
    rings: &Vec<Item>,
) -> Option<i32> {
    let max_cost = weapons
        .iter()
        .map(|w| {
            let mut player = player.clone();
            player.set_weapon(w.clone());
            if will_player_win(&player, boss) {
                None
            } else {
                let cost_with_1_ring = get_max_cost_of_1_ring(&player, boss, rings).unwrap_or(0);
                let cost_with_2_rings = get_max_cost_of_2_rings(&player, boss, rings).unwrap_or(0);
                let cost_with_armor =
                    get_max_cost_of_armor(&player, boss, armors, rings).unwrap_or(0);
                let max_cost = w.get_cost().max(
                    cost_with_1_ring.max(cost_with_2_rings).max(cost_with_armor) + w.get_cost(),
                );
                Some(max_cost)
            }
        })
        .flatten()
        .collect::<Vec<i32>>();
    max_cost.into_iter().max()
}

fn get_max_cost_of_armor(
    player: &Player,
    boss: &Player,
    armors: &Vec<Item>,
    rings: &Vec<Item>,
) -> Option<i32> {
    armors
        .iter()
        .map(|a| {
            let mut player = player.clone();
            player.set_armor(a.clone());
            if will_player_win(&player, boss) {
                None
            } else {
                let cost_with_0_rings = a.get_cost();
                let cost_with_1_ring = get_max_cost_of_1_ring(&player, boss, rings).unwrap_or(0);
                let cost_with_2_rings = get_max_cost_of_2_rings(&player, boss, rings).unwrap_or(0);
                Some(cost_with_0_rings.max(cost_with_1_ring.max(cost_with_2_rings) + a.get_cost()))
            }
        })
        .flatten()
        .collect::<Vec<i32>>()
        .into_iter()
        .max()
}

fn get_max_cost_of_1_ring(player: &Player, boss: &Player, rings: &Vec<Item>) -> Option<i32> {
    rings
        .iter()
        .map(|r| {
            let mut player = player.clone();
            player.add_ring(r.clone());

            if !will_player_win(&player, boss) {
                Some(r.get_cost())
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<i32>>()
        .into_iter()
        .max()
}

fn get_max_cost_of_2_rings(player: &Player, boss: &Player, rings: &Vec<Item>) -> Option<i32> {
    let mut max = None;
    for p1 in 0..rings.len() {
        for p2 in p1 + 1..rings.len() {
            let mut player = player.clone();
            player.add_ring(rings[p1].clone());
            player.add_ring(rings[p2].clone());
            if !will_player_win(&player, boss) {
                let cost = rings[p1].get_cost() + rings[p2].get_cost();
                if cost > max.unwrap_or(0) {
                    max = Some(cost);
                }
            }
        }
    }
    max
}

fn will_player_win(player: &Player, boss: &Player) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();
    loop {
        boss.take_attack(&player);
        if !boss.is_alive() {
            return true;
        }
        player.take_attack(&boss);
        if !player.is_alive() {
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

    let weapon = Item::new(String::from("boss weapon"), damage, 0, 0, Weapon);
    let armor = Item::new(String::from("boss armor"), 0, armor, 0, Armor);
    let mut boss = Player::new(hit_points);
    boss.set_weapon(weapon);
    boss.set_armor(armor);
    boss
}

fn parse_items(item_type: ItemType) -> Vec<Item> {
    let file = match item_type {
        Weapon => "./resources/weapons.txt",
        Armor => "./resources/armor.txt",
        Ring => "./resources/rings.txt",
    };
    let text = std::fs::read_to_string(file).unwrap();
    let items = text
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            Item::new(
                String::from(parts[0]),
                parts[2].parse::<i32>().unwrap(),
                parts[3].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
                item_type.clone(),
            )
        })
        .collect();
    println!("{:?}", items);
    items
}
