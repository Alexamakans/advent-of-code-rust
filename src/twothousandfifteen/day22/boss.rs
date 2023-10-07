use super::Effect;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Boss {
    pub hit_points: i32,
    pub damage: i32,
    pub effects: Vec<Effect>,
}

impl From<&str> for Boss {
    fn from(value: &str) -> Self {
        let mut stats = Vec::new();
        for line in value.lines() {
            stats.push(line.rsplit_once(' ').unwrap().1);
        }

        Boss {
            hit_points: stats[0].parse().unwrap(),
            damage: stats[1].parse().unwrap(),
            effects: Vec::new(),
        }
    }
}

impl Boss {
    pub fn tick_effects(&mut self) {
        for e in self.effects.iter_mut() {
            self.hit_points -= e.turn_damage;
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
