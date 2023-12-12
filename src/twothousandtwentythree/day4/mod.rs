use std::collections::{HashMap, HashSet};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut result = 0;
        for line in input.lines() {
            let parts = line.split_once(":");
            let (card_str, hand_str) = parts.unwrap().1.split_once("|").unwrap();
            let mut cards = HashSet::new();
            for c in card_str.split_ascii_whitespace() {
                cards.insert(c.parse::<i32>().unwrap());
            }

            let mut hand = HashSet::new();
            for h in hand_str.split_ascii_whitespace() {
                hand.insert(h.parse::<i32>().unwrap());
            }

            let in_both = cards.intersection(&hand);
            let mut score = 0;
            for _ in in_both {
                score *= 2;
                if score == 0 {
                    score = 1;
                }
            }

            result += score;
        }

        result
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut cache = HashMap::new();
        let mut total_instances = 0;
        let lines = input.lines().collect::<Vec<&str>>();
        let mut next_lines = Vec::new();
        for (index, _) in lines.iter().enumerate() {
            next_lines.push(index);
        }
        while next_lines.len() > 0 {
            let mut new_next_lines = Vec::new();
            for line_index in next_lines.clone() {
                total_instances += 1;
                if cache.get(&line_index).is_some() {
                    let c: &Vec<usize> = cache.get(&line_index).unwrap();
                    for a in c {
                        new_next_lines.push(*a);
                    }
                } else {
                    let line = lines.get(line_index).expect("should exist");
                    let parts = line.split_once(":").unwrap();
                    let (card_str, hand_str) = parts.1.split_once("|").unwrap();
                    let mut cards = HashSet::new();
                    for c in card_str.split_ascii_whitespace() {
                        cards.insert(c.parse::<i32>().unwrap());
                    }

                    let mut hand = HashSet::new();
                    for h in hand_str.split_ascii_whitespace() {
                        hand.insert(h.parse::<i32>().unwrap());
                    }

                    let in_both = cards.intersection(&hand);
                    let mut new_lines = Vec::new();
                    for (index, _) in in_both.enumerate() {
                        new_lines.push(line_index + index + 1);
                    }

                    cache.insert(line_index, new_lines.clone());
                    for l in new_lines {
                        new_next_lines.push(l);
                    }
                }
            }
            next_lines = new_next_lines;
        }

        total_instances as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            13,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 18653);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            30,
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 5921508);
    }
}
