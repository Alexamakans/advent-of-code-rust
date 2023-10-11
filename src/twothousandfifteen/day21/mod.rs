use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let boss = Boss::from(input);
        let shop = Shop::new();
        let mut lowest_money_spent = i32::MAX;
        for weapon in shop.weapons.iter() {
            {
                let gear = vec![weapon.clone()];
                let player = Player::from_gear(gear);
                if player.money_spent < lowest_money_spent {
                    if get_winner(boss.clone(), player.clone()) == 1 {
                        lowest_money_spent = player.money_spent;
                    }
                }
            }

            for ring in shop.rings.iter() {
                let gear = vec![weapon.clone(), ring.clone()];
                let player = Player::from_gear(gear);
                if player.money_spent < lowest_money_spent {
                    if get_winner(boss.clone(), player.clone()) == 1 {
                        lowest_money_spent = player.money_spent;
                    }
                }
            }

            for ring_a in shop.rings.iter() {
                for ring_b in shop.rings.iter() {
                    if ring_a == ring_b {
                        continue;
                    }

                    let gear = vec![weapon.clone(), ring_a.clone(), ring_b.clone()];
                    let player = Player::from_gear(gear);
                    if player.money_spent < lowest_money_spent {
                        if get_winner(boss.clone(), player.clone()) == 1 {
                            lowest_money_spent = player.money_spent;
                        }
                    }
                }
            }
            for armor in shop.armors.iter() {
                {
                    let gear = vec![weapon.clone(), armor.clone()];
                    let player = Player::from_gear(gear);
                    if player.money_spent < lowest_money_spent {
                        if get_winner(boss.clone(), player.clone()) == 1 {
                            lowest_money_spent = player.money_spent;
                        }
                    }
                }

                for ring in shop.rings.iter() {
                    let gear = vec![weapon.clone(), armor.clone(), ring.clone()];
                    let player = Player::from_gear(gear);
                    if player.money_spent < lowest_money_spent {
                        if get_winner(boss.clone(), player.clone()) == 1 {
                            lowest_money_spent = player.money_spent;
                        }
                    }
                }

                for ring_a in shop.rings.iter() {
                    for ring_b in shop.rings.iter() {
                        if ring_a == ring_b {
                            continue;
                        }

                        let gear = vec![
                            weapon.clone(),
                            armor.clone(),
                            ring_a.clone(),
                            ring_b.clone(),
                        ];
                        let player = Player::from_gear(gear);
                        if player.money_spent < lowest_money_spent {
                            if get_winner(boss.clone(), player.clone()) == 1 {
                                lowest_money_spent = player.money_spent;
                            }
                        }
                    }
                }
            }
        }

        lowest_money_spent
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let boss = Boss::from(input);
        let shop = Shop::new();
        let mut highest_money_spent = i32::MIN;
        for weapon in shop.weapons.iter() {
            {
                let gear = vec![weapon.clone()];
                let player = Player::from_gear(gear);
                if player.money_spent > highest_money_spent {
                    if get_winner(boss.clone(), player.clone()) == 0 {
                        highest_money_spent = player.money_spent;
                    }
                }
            }

            for ring in shop.rings.iter() {
                let gear = vec![weapon.clone(), ring.clone()];
                let player = Player::from_gear(gear);
                if player.money_spent > highest_money_spent {
                    if get_winner(boss.clone(), player.clone()) == 0 {
                        highest_money_spent = player.money_spent;
                    }
                }
            }

            for ring_a in shop.rings.iter() {
                for ring_b in shop.rings.iter() {
                    if ring_a == ring_b {
                        continue;
                    }

                    let gear = vec![weapon.clone(), ring_a.clone(), ring_b.clone()];
                    let player = Player::from_gear(gear);
                    if player.money_spent > highest_money_spent {
                        if get_winner(boss.clone(), player.clone()) == 0 {
                            highest_money_spent = player.money_spent;
                        }
                    }
                }
            }
            for armor in shop.armors.iter() {
                {
                    let gear = vec![weapon.clone(), armor.clone()];
                    let player = Player::from_gear(gear);
                    if player.money_spent > highest_money_spent {
                        if get_winner(boss.clone(), player.clone()) == 0 {
                            highest_money_spent = player.money_spent;
                        }
                    }
                }

                for ring in shop.rings.iter() {
                    let gear = vec![weapon.clone(), armor.clone(), ring.clone()];
                    let player = Player::from_gear(gear);
                    if player.money_spent > highest_money_spent {
                        if get_winner(boss.clone(), player.clone()) == 0 {
                            highest_money_spent = player.money_spent;
                        }
                    }
                }

                for ring_a in shop.rings.iter() {
                    for ring_b in shop.rings.iter() {
                        if ring_a == ring_b {
                            continue;
                        }

                        let gear = vec![
                            weapon.clone(),
                            armor.clone(),
                            ring_a.clone(),
                            ring_b.clone(),
                        ];
                        let player = Player::from_gear(gear);
                        if player.money_spent > highest_money_spent {
                            if get_winner(boss.clone(), player.clone()) == 0 {
                                highest_money_spent = player.money_spent;
                            }
                        }
                    }
                }
            }
        }

        highest_money_spent
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 21)
    }
}

fn get_winner<A: Fighter, B: Fighter>(mut boss: A, mut player: B) -> i32 {
    loop {
        player.attack(&mut boss);
        if boss.is_dead() {
            return 1;
        }
        boss.attack(&mut player);
        if player.is_dead() {
            return 0;
        }
    }
}

trait Fighter {
    fn attack<T>(&mut self, other: &mut T)
    where
        T: Fighter;
    fn take_damage(&mut self, amount: i32);
    fn is_dead(&self) -> bool;
}

macro_rules! impl_fighter_for {
    ($t:ident) => {
        impl Fighter for $t {
            fn attack<T>(&mut self, other: &mut T)
            where
                T: Fighter,
            {
                other.take_damage(self.damage);
            }

            fn take_damage(&mut self, amount: i32) {
                self.hit_points -= i32::max(0, amount - self.armor);
            }

            fn is_dead(&self) -> bool {
                self.hit_points <= 0
            }
        }
    };
}

#[derive(Clone)]
struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
    money_spent: i32,
}

impl Player {
    fn from_gear(gear: Vec<Equipment>) -> Self {
        let (damage, armor, money_spent) = gear
            .into_iter()
            .map(|e| (e.damage, e.armor, e.cost))
            .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1, acc.2 + e.2))
            .unwrap();
        Player {
            hit_points: 100,
            damage,
            armor,
            money_spent,
        }
    }
}

impl_fighter_for!(Player);

#[derive(Clone)]
struct Boss {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

impl_fighter_for!(Boss);

impl From<&str> for Boss {
    fn from(value: &str) -> Self {
        let mut stats = Vec::new();
        for line in value.lines() {
            stats.push(line.rsplit_once(' ').unwrap().1);
        }

        Boss {
            hit_points: stats[0].parse().unwrap(),
            damage: stats[1].parse().unwrap(),
            armor: stats[2].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        assert_eq!(solver.part_one(), 78);
    }

    #[test]
    fn part_two_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_two(), 123);
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Equipment {
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone)]
struct Shop {
    weapons: Vec<Equipment>,
    armors: Vec<Equipment>,
    rings: Vec<Equipment>,
}

impl Shop {
    fn new() -> Self {
        Shop {
            weapons: vec![
                Equipment {
                    cost: 8,
                    damage: 4,
                    armor: 0,
                },
                Equipment {
                    cost: 10,
                    damage: 5,
                    armor: 0,
                },
                Equipment {
                    cost: 25,
                    damage: 6,
                    armor: 0,
                },
                Equipment {
                    cost: 40,
                    damage: 7,
                    armor: 0,
                },
                Equipment {
                    cost: 74,
                    damage: 8,
                    armor: 0,
                },
            ],
            armors: vec![
                Equipment {
                    cost: 13,
                    damage: 0,
                    armor: 1,
                },
                Equipment {
                    cost: 31,
                    damage: 0,
                    armor: 2,
                },
                Equipment {
                    cost: 53,
                    damage: 0,
                    armor: 3,
                },
                Equipment {
                    cost: 75,
                    damage: 0,
                    armor: 4,
                },
                Equipment {
                    cost: 102,
                    damage: 0,
                    armor: 5,
                },
            ],
            rings: vec![
                Equipment {
                    cost: 25,
                    damage: 1,
                    armor: 0,
                },
                Equipment {
                    cost: 50,
                    damage: 2,
                    armor: 0,
                },
                Equipment {
                    cost: 100,
                    damage: 3,
                    armor: 0,
                },
                Equipment {
                    cost: 20,
                    damage: 0,
                    armor: 1,
                },
                Equipment {
                    cost: 40,
                    damage: 0,
                    armor: 2,
                },
                Equipment {
                    cost: 80,
                    damage: 0,
                    armor: 3,
                },
            ],
        }
    }
}
