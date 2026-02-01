use crate::item::Item;

pub struct Player {
    max_hp: i32,
    current_hp: i32,
    weapon: Item,
    armor: Option<Item>,
    rings: Vec<Item>,
}

impl Player {
    pub fn new(max_hp: i32, weapon: Item) -> Player {
        Player {
            max_hp,
            current_hp: max_hp,
            weapon,
            armor: None,
            rings: Vec::new(),
        }
    }

    pub fn get_damage(&self) -> i32 {
        self.weapon.get_damage_mod()
    }

    pub fn take_attack(&mut self, attacker: &Player) {
        let armor = self.armor.as_ref().map(|a| a.get_armor_mod()).unwrap_or(0);
        let incoming_damage = attacker.get_damage();
        let damage = (incoming_damage - armor).max(1);
        self.current_hp -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub fn set_weapon(&mut self, weapon: Item) {
        self.weapon = weapon;
    }

    pub fn set_armor(&mut self, armor: Item) {
        self.armor = Some(armor);
    }

    pub fn add_ring(&mut self, ring: Item) -> bool {
        if self.rings.len() >= 2 {
            return false;
        }
        self.rings.push(ring);
        true
    }
}
