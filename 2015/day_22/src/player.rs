use crate::effect::Effect;

pub struct Player {
    name: String,
    max_hp: i32,
    current_hp: i32,
    effects: Vec<Effect>,
}

impl Player {
    pub fn new(name: String, max_hp: i32) -> Self {
        Player {
            name,
            max_hp,
            current_hp: max_hp,
            effects: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn damage(&mut self, damage: i32) {
        self.current_hp -= damage;
    }

    pub fn apply_effects(&mut self) {
        self.effects
            .iter_mut()
            .for_each(|effect| effect.apply(self));
        self.effects.retain(|effect| effect.is_active());
    }
}
