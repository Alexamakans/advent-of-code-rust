use super::{Effect, Spell};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Player {
    pub hit_points: i32,
    pub mana_points: i32,
    pub armor: i32,
    pub spells: Vec<Spell>,
    pub effects: Vec<Effect>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hit_points: 50,
            mana_points: 500,
            armor: 0,
            spells: Spell::get_all_spells(),
            effects: Vec::new(),
        }
    }

    pub fn tick_effects(&mut self) {
        self.armor = 0;
        for e in self.effects.iter_mut() {
            self.hit_points += e.turn_heal;
            self.mana_points += e.turn_mana_regen;
            self.armor += e.armor_buff;
            e.duration -= 1;
        }

        self.effects = self
            .effects
            .clone()
            .into_iter()
            .filter(|e| e.duration > 0)
            .collect();
    }
}
