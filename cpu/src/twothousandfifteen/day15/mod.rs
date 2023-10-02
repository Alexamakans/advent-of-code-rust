// Could likely be optimized by using an algorithm that generates the proper n choose 100 (with repeats)
// combinations, but cba.

use crate::scanf;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i64> for Solver {
    fn part_one_driver(&self, input: &str) -> i64 {
        let ingredients = input
            .lines()
            .map(|e| Ingredient::from(e))
            .collect::<Vec<Ingredient>>();
        let zero_to_hundred = (0..=100).collect::<Vec<u8>>();
        let mut increasing_sequence = zero_to_hundred.into_increasing_sequence();
        increasing_sequence.set_indices(vec![0; ingredients.len()]);
        let increasing_sequence_limited =
            increasing_sequence.take(101_usize.pow(ingredients.len() as u32));

        let mut best_score = i64::MIN;
        for next in increasing_sequence_limited {
            let sum = next.iter().fold(0 as u32, |acc, count| acc + *count as u32);
            if sum != 100 {
                continue;
            }

            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            for (index, ingredient) in ingredients.iter().enumerate() {
                let amount = *next.get(index).unwrap() as i64;
                capacity += amount * ingredient.capacity;
                durability += amount * ingredient.durability;
                flavor += amount * ingredient.flavor;
                texture += amount * ingredient.texture;
            }

            let score = capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);
            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    fn part_two_driver(&self, input: &str) -> i64 {
        let ingredients = input
            .lines()
            .map(|e| Ingredient::from(e))
            .collect::<Vec<Ingredient>>();
        let zero_to_hundred = (0..=100).collect::<Vec<u8>>();
        let mut increasing_sequence = zero_to_hundred.into_increasing_sequence();
        increasing_sequence.set_indices(vec![0; ingredients.len()]);
        let increasing_sequence_limited =
            increasing_sequence.take(101_usize.pow(ingredients.len() as u32));

        let mut best_score = i64::MIN;
        for next in increasing_sequence_limited {
            let sum = next.iter().fold(0 as u32, |acc, count| acc + *count as u32);
            if sum != 100 {
                continue;
            }

            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;
            for (index, ingredient) in ingredients.iter().enumerate() {
                let amount = *next.get(index).unwrap() as i64;
                capacity += amount * ingredient.capacity;
                durability += amount * ingredient.durability;
                flavor += amount * ingredient.flavor;
                texture += amount * ingredient.texture;
                calories += amount * ingredient.calories;
            }

            if calories != 500 {
                continue;
            }

            let score = capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);
            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 15)
    }
}

#[derive(Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        let modified = value.replace(",", "");
        // Butterscotch:[0] capacity[1] -1[2] durability[3] -2[4] flavor[5] 6[6] texture[7] 3[8] calories[9] 8[10]
        let parts = scanf!(
            &modified, " ", String, String, i64, String, i64, String, i64, String, i64, String, i64
        );
        Ingredient {
            capacity: parts.2,
            durability: parts.4,
            flavor: parts.6,
            texture: parts.8,
            calories: parts.10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
            62842880,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 18965440);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
            57600000,
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 15862900);
    }
}
