mod boss;
mod player;
mod spell;

use boss::*;
use player::*;
use spell::*;

use std::collections::{HashSet, VecDeque};

use super::{super::utils::*, YEAR};

#[derive(Clone, PartialEq, Eq)]
struct State {
    boss: Boss,
    player: Player,
    mana_spent: i32,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.boss.hash(state);
        self.player.hash(state);
        self.mana_spent.hash(state);
    }
}

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut least_amount_of_mana_spent = i32::MAX;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let original_state = State {
            boss: Boss::from(input),
            player: Player::new(),
            mana_spent: 0,
        };

        queue.push_back(original_state.clone());
        while let Some(mut state) = queue.pop_front() {
            state.player.tick_effects();
            state.boss.tick_effects();

            if state.boss.hit_points <= 0 {
                if state.mana_spent < least_amount_of_mana_spent {
                    least_amount_of_mana_spent = state.mana_spent;
                }
                continue;
            }

            let castable_spells = state
                .player
                .spells
                .clone()
                .into_iter()
                .filter(|spell| spell.can_cast(&state.boss, &state.player))
                .collect::<Vec<Spell>>();

            for spell in castable_spells {
                let mut new_state = state.clone();

                spell.cast(&mut new_state.player, &mut new_state.boss);
                new_state.mana_spent += spell.cost;

                // Boss turn
                // apply effects on player and boss
                new_state.player.tick_effects();
                new_state.boss.tick_effects();

                if new_state.boss.hit_points <= 0 {
                    if new_state.mana_spent < least_amount_of_mana_spent {
                        least_amount_of_mana_spent = new_state.mana_spent;
                    }
                    continue;
                }

                new_state.player.hit_points -=
                    1.max(new_state.boss.damage - new_state.player.armor);
                if new_state.player.hit_points <= 0 {
                    continue;
                }

                if visited.insert(new_state.clone()) {
                    queue.push_back(new_state.clone());
                }
            }
        }

        least_amount_of_mana_spent
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut least_amount_of_mana_spent = i32::MAX;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let original_state = State {
            boss: Boss::from(input),
            player: Player::new(),
            mana_spent: 0,
        };

        queue.push_back(original_state.clone());
        while let Some(mut state) = queue.pop_front() {
            { // part 2 change here
                state.player.hit_points -= 1;
                if state.player.hit_points <= 0 {
                    continue;
                }
            }
            state.player.tick_effects();
            state.boss.tick_effects();

            if state.boss.hit_points <= 0 {
                if state.mana_spent < least_amount_of_mana_spent {
                    least_amount_of_mana_spent = state.mana_spent;
                }
                continue;
            }

            let castable_spells = state
                .player
                .spells
                .clone()
                .into_iter()
                .filter(|spell| spell.can_cast(&state.boss, &state.player))
                .collect::<Vec<Spell>>();

            for spell in castable_spells {
                let mut new_state = state.clone();

                spell.cast(&mut new_state.player, &mut new_state.boss);
                new_state.mana_spent += spell.cost;

                // Boss turn
                // apply effects on player and boss
                new_state.player.tick_effects();
                new_state.boss.tick_effects();

                if new_state.boss.hit_points <= 0 {
                    if new_state.mana_spent < least_amount_of_mana_spent {
                        least_amount_of_mana_spent = new_state.mana_spent;
                    }
                    continue;
                }

                new_state.player.hit_points -=
                    1.max(new_state.boss.damage - new_state.player.armor);
                if new_state.player.hit_points <= 0 {
                    continue;
                }

                if visited.insert(new_state.clone()) {
                    queue.push_back(new_state.clone());
                }
            }
        }

        least_amount_of_mana_spent
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 22)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        assert_eq!(solver.part_one(), 1269);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        assert_eq!(solver.part_two(), 1309);
    }
}
