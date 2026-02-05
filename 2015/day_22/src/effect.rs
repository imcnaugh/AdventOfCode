use crate::player::Player;

pub enum Effect {
    Poison { remaining_turns: usize, damage: i32 },
    Damage(i32),
}

impl Effect {
    pub fn apply(&mut self, player: &mut Player) {
        match self {
            Effect::Poison {
                remaining_turns,
                damage,
            } => {
                player.damage(*damage);
                *remaining_turns -= 1;
            }
            &mut Effect::Damage(amount) => todo!(),
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Effect::Poison {
                remaining_turns, ..
            } => *remaining_turns > 0,
        }
    }
}
