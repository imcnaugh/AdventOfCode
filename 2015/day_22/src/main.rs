use crate::Spells::{Drain, MagicMissile, Poison, Recharge, Shield};
use std::collections::HashSet;

fn main() {
    let initial_state = FightState {
        player_health: 50,
        boss_health: 55,
        player_mana: 500,
        used_mana: 0,
        is_player_turn: true,
        active_effects: HashSet::new(),
        poison_remaining_turns: None,
        shields_remaining_turns: None,
        recharges_remaining_turns: None,
    };
    println!("{}", fight(initial_state));
}

fn fight(state: FightState) -> i32 {
    if state.used_mana > 1500 {
        return i32::MAX;
    }

    let mut new_state = state.clone();
    if new_state.is_player_turn {
        new_state.player_health -= 1;
        if new_state.player_health <= 0 {
            return i32::MAX;
        }
    }

    state.active_effects.iter().for_each(|effect| match effect {
        Effect::Poison => new_state.boss_health -= 3,
        Effect::Recharge => new_state.player_mana += 101,
        _ => {}
    });

    if let Some(turns) = state.poison_remaining_turns {
        if turns > 1 {
            new_state.poison_remaining_turns = Some(turns - 1);
        } else {
            new_state.active_effects.remove(&Effect::Poison);
            new_state.poison_remaining_turns = None;
        }
    }

    if let Some(turns) = state.shields_remaining_turns {
        if turns > 1 {
            new_state.shields_remaining_turns = Some(turns - 1);
        } else {
            new_state.active_effects.remove(&Effect::Shield);
            new_state.shields_remaining_turns = None;
        }
    }

    if let Some(turns) = state.recharges_remaining_turns {
        if turns > 1 {
            new_state.recharges_remaining_turns = Some(turns - 1);
        } else {
            new_state.active_effects.remove(&Effect::Recharge);
            new_state.recharges_remaining_turns = None;
        }
    }

    if new_state.player_health <= 0 {
        return i32::MAX;
    }
    if new_state.boss_health <= 0 {
        return new_state.used_mana;
    }
    if state.is_player_turn {
        let available_spells = get_available_spells(&new_state);
        if available_spells.is_empty() {
            return i32::MAX;
        }
        available_spells
            .iter()
            .map(|spell| {
                let mut ns = new_state.clone();
                match spell {
                    MagicMissile => {
                        ns.used_mana += 53;
                        ns.boss_health -= 4;
                        ns.is_player_turn = false;
                        ns.player_mana -= 53;
                        fight(ns)
                    }
                    Drain => {
                        ns.used_mana += 73;
                        ns.player_mana -= 73;
                        ns.boss_health -= 2;
                        ns.player_health += 2;
                        ns.is_player_turn = false;
                        fight(ns)
                    }
                    Shield => {
                        ns.active_effects.insert(Effect::Shield);
                        ns.shields_remaining_turns = Some(6);
                        ns.is_player_turn = false;
                        ns.player_mana -= 113;
                        ns.used_mana += 113;
                        fight(ns)
                    }
                    Poison => {
                        ns.active_effects.insert(Effect::Poison);
                        ns.poison_remaining_turns = Some(6);
                        ns.is_player_turn = false;
                        ns.player_mana -= 173;
                        ns.used_mana += 173;
                        fight(ns)
                    }
                    Recharge => {
                        ns.active_effects.insert(Effect::Recharge);
                        ns.recharges_remaining_turns = Some(5);
                        ns.is_player_turn = false;
                        ns.player_mana -= 229;
                        ns.used_mana += 229;
                        fight(ns)
                    }
                }
            })
            .min()
            .unwrap()
    } else {
        let boss_damage = 8;
        if state.active_effects.contains(&Effect::Shield) {
            new_state.player_health -= 1;
        } else {
            new_state.player_health -= boss_damage
        }
        new_state.is_player_turn = true;
        fight(new_state)
    }
}

fn get_available_spells(state: &FightState) -> Vec<Spells> {
    vec![MagicMissile, Drain, Shield, Poison, Recharge]
        .into_iter()
        .filter(|spell| match spell {
            MagicMissile => state.player_mana >= 53,
            Drain => state.player_mana >= 73,
            Shield => state.player_mana >= 113 && !state.active_effects.contains(&Effect::Shield),
            Poison => state.player_mana >= 173 && !state.active_effects.contains(&Effect::Poison),
            Recharge => {
                state.player_mana >= 229 && !state.active_effects.contains(&Effect::Recharge)
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct FightState {
    player_health: i32,
    boss_health: i32,
    player_mana: i32,
    used_mana: i32,
    is_player_turn: bool,
    active_effects: HashSet<Effect>,
    poison_remaining_turns: Option<u8>,
    shields_remaining_turns: Option<u8>,
    recharges_remaining_turns: Option<u8>,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Effect {
    Poison,
    Shield,
    Recharge,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Spells {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}
