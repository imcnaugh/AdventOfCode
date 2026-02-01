use crate::item::Item;
use crate::player::Player;
use regex::Regex;

mod item;
mod player;

fn main() {
    let input = include_str!("../resources/input.txt");
}

fn part_1(input: &str) -> usize {
    let mut boss = crate_boss(input);

    todo!()
}

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
