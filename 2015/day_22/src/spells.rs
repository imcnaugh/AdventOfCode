use crate::effect::Effect;

pub struct Spells {
    name: String,
    mana_to_cast: i32,
    caster: String,
    target: String,
    effect: Vec<Effect>,
    remaining_duration: i32,
}

impl Spells {
    pub fn magic_missile(caster: String, target: String) -> Self {
        Self {
            name: "Magic Missile".to_string(),
            mana_to_cast: 53,
            caster,
            target,
            effect: vec![Effect::Damage(4)],
            remaining_duration: 1,
        }
    }
}
