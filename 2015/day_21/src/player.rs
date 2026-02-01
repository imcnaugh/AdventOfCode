use crate::item::{Item, ItemType};

#[derive(Debug, Clone)]
pub struct Player {
    max_hp: i32,
    current_hp: i32,
    items: Vec<Item>,
}

impl Player {
    pub fn new(max_hp: i32) -> Player {
        Player {
            max_hp,
            current_hp: max_hp,
            items: Vec::new(),
        }
    }

    pub fn get_damage(&self) -> i32 {
        self.items.iter().map(|i| i.get_damage_mod()).sum()
    }

    pub fn take_attack(&mut self, attacker: &Player) {
        let total_armor = self.items.iter().map(|i| i.get_armor_mod()).sum::<i32>();
        let incoming_damage = attacker.get_damage();
        let damage = (incoming_damage - total_armor).max(1);
        self.current_hp -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub fn set_weapon(&mut self, weapon: Item) -> bool {
        if self
            .items
            .iter()
            .any(|i| i.get_item_type() == ItemType::Weapon)
        {
            false
        } else {
            self.items.push(weapon);
            true
        }
    }

    pub fn set_armor(&mut self, armor: Item) -> bool {
        if self
            .items
            .iter()
            .any(|i| i.get_item_type() == ItemType::Armor)
        {
            false
        } else {
            self.items.push(armor);
            true
        }
    }

    pub fn add_ring(&mut self, ring: Item) -> bool {
        if self
            .items
            .iter()
            .filter(|i| i.get_item_type() == ItemType::Ring)
            .count()
            >= 2
        {
            return false;
        }
        self.items.push(ring);
        true
    }
}
