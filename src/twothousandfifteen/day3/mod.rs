use std::collections::HashSet;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        let mut visited = HashSet::new();
        let mut current_position = (0, 0);
        visited.insert(current_position);
        let mut unique_visited = 1;
        for char in input.chars() {
            match char {
                '>' => current_position.0 += 1,
                '^' => current_position.1 -= 1,
                '<' => current_position.0 -= 1,
                'v' => current_position.1 += 1,
                _ => unreachable!(),
            };
            if visited.insert(current_position) {
                unique_visited += 1;
            }
        }
        unique_visited
    }

    fn part_two_driver(&self, input: String) -> i32 {
        let mut visited = HashSet::new();
        let mut current_santa_position = (0, 0);
        let mut current_robot_position = (0, 0);
        visited.insert(current_santa_position);
        let mut unique_visited = 1;
        for (index, char) in input.chars().enumerate() {
            let current_position = if index % 2 == 0 {
                &mut current_santa_position
            } else {
                &mut current_robot_position
            };
            match char {
                '>' => current_position.0 += 1,
                '^' => current_position.1 -= 1,
                '<' => current_position.0 -= 1,
                'v' => current_position.1 += 1,
                _ => unreachable!(),
            };
            if visited.insert(*current_position) {
                unique_visited += 1;
            }
        }
        unique_visited
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 3)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![
            (">", 2),
            ("^>v<", 4),
            ("^v^v^v^v^v", 2),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(String::from(case.0)), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 2565);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        let cases = vec![
            ("^v", 3),
            ("^>v<", 3),
            ("^v^v^v^v^v", 11),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(String::from(case.0)), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 2639);
    }
}