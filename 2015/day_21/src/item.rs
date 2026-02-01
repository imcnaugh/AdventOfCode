#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    name: String,
    damage_mod: i32,
    armor_mod: i32,
    cost: i32,
    item_type: ItemType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Weapon,
    Armor,
    Ring,
}

impl Item {
    pub fn new(
        name: String,
        damage_mod: i32,
        armor_mod: i32,
        cost: i32,
        item_type: ItemType,
    ) -> Item {
        Item {
            name,
            damage_mod,
            armor_mod,
            cost,
            item_type,
        }
    }

    pub fn get_damage_mod(&self) -> i32 {
        self.damage_mod
    }

    pub fn get_armor_mod(&self) -> i32 {
        self.armor_mod
    }

    pub fn get_cost(&self) -> i32 {
        self.cost
    }

    pub fn get_item_type(&self) -> ItemType {
        self.item_type.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
