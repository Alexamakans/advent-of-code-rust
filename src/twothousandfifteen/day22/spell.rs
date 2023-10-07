use super::{Boss, Player};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Spell {
    pub name: String,
    pub id: i32,
    pub cost: i32,
    pub target: Target,
    pub effect: Effect,
}

impl Spell {
    pub fn cast(&self, player: &mut Player, boss: &mut Boss) {
        match self.name.as_str() {
            "Magic Missile" => boss.hit_points -= self.effect.instant_damage,
            "Drain" => {
                boss.hit_points -= self.effect.instant_damage;
                player.hit_points += self.effect.instant_heal;
            }
            "Shield" => player.effects.push(self.effect.clone()),
            "Poison" => boss.effects.push(self.effect.clone()),
            "Recharge" => player.effects.push(self.effect.clone()),
            _ => unreachable!(),
        }

        player.mana_points -= self.cost;
    }

    pub fn can_cast(&self, boss: &Boss, player: &Player) -> bool {
        if player.mana_points < self.cost {
            false
        } else {
            match self.target {
                Target::Ally => player
                    .effects
                    .iter()
                    .find(|e| e.id == self.effect.id)
                    .is_none(),
                Target::Enemy => boss
                    .effects
                    .iter()
                    .find(|e| e.id == self.effect.id)
                    .is_none(),
                Target::Both => true,
            }
        }
    }
    
    pub fn get_all_spells() -> Vec<Spell> {
        vec![
                    Spell {
                        name: String::from("Magic Missile"),
                        id: 0,
                        target: Target::Enemy,
                        cost: 53,
                        effect: Effect {
                            id: 0,
                            name: String::from("Magic Missile"),
                            duration: 0,
                            instant_damage: 4,
                            turn_damage: 0,
                            instant_heal: 0,
                            turn_heal: 0,
                            turn_mana_regen: 0,
                            armor_buff: 0,
                        },
                    },
                    Spell {
                        name: String::from("Drain"),
                        id: 1,
                        target: Target::Both,
                        cost: 73,
                        effect: Effect {
                            id: 1,
                            name: String::from("Drain"),
                            duration: 0,
                            instant_damage: 2,
                            turn_damage: 0,
                            instant_heal: 2,
                            turn_heal: 0,
                            turn_mana_regen: 0,
                            armor_buff: 0,
                        },
                    },
                    Spell {
                        name: String::from("Shield"),
                        id: 2,
                        target: Target::Ally,
                        cost: 113,
                        effect: Effect {
                            id: 2,
                            name: String::from("Shield"),
                            duration: 6,
                            instant_damage: 0,
                            turn_damage: 0,
                            instant_heal: 0,
                            turn_heal: 0,
                            turn_mana_regen: 0,
                            armor_buff: 7,
                        },
                    },
                    Spell {
                        name: String::from("Poison"),
                        id: 3,
                        target: Target::Enemy,
                        cost: 173,
                        effect: Effect {
                            id: 3,
                            name: String::from("Poison"),
                            duration: 6,
                            instant_damage: 0,
                            turn_damage: 3,
                            instant_heal: 0,
                            turn_heal: 0,
                            turn_mana_regen: 0,
                            armor_buff: 0,
                        },
                    },
                    Spell {
                        name: String::from("Recharge"),
                        id: 4,
                        target: Target::Ally,
                        cost: 229,
                        effect: Effect {
                            id: 4,
                            name: String::from("Recharge"),
                            duration: 5,
                            instant_damage: 0,
                            turn_damage: 0,
                            instant_heal: 0,
                            turn_heal: 0,
                            turn_mana_regen: 101,
                            armor_buff: 0,
                        },
                    },
                ]
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Target {
    Ally,
    Enemy,
    Both,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Effect {
    pub id: i32,
    pub name: String,
    pub duration: i32,
    pub instant_damage: i32,
    pub turn_damage: i32,
    pub instant_heal: i32,
    pub turn_heal: i32,
    pub turn_mana_regen: i32,
    pub armor_buff: i32,
}
