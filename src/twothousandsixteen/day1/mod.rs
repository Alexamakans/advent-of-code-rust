use std::collections::HashSet;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let (mut x, mut y, mut heading) = (0_i32, 0_i32, 0_i32);
        for step in input.split_terminator(", ") {
            let mut chars = step.chars();
            match chars.next().unwrap() {
                'L' => {
                    heading = turn(Direction::Left, heading);
                }
                'R' => {
                    heading = turn(Direction::Right, heading);
                }
                _ => unreachable!(),
            }
            let num_steps = chars.as_str().parse::<i32>().unwrap();
            (x, y) = walk(x, y, num_steps, heading);
        }

        x.abs() + y.abs()
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut visited_locations = HashSet::new();
        let (mut x, mut y, mut heading) = (0_i32, 0_i32, 0_i32);
        visited_locations.insert((x, y));
        for step in input.split_terminator(", ") {
            let mut chars = step.chars();
            match chars.next().unwrap() {
                'L' => {
                    heading = turn(Direction::Left, heading);
                }
                'R' => {
                    heading = turn(Direction::Right, heading);
                }
                _ => unreachable!(),
            }
            let num_steps = chars.as_str().parse::<i32>().unwrap();
            for _ in 0..num_steps {
                (x, y) = walk(x, y, 1, heading);
                if !visited_locations.insert((x, y)) {
                    return x.abs() + y.abs();
                }
            }
        }

        unreachable!("buggy code - expected solution to be found by now");
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 1)
    }
}

enum Direction {
    Left,
    Right,
}

fn turn(direction: Direction, heading: i32) -> i32 {
    match direction {
        Direction::Left => (heading - 1).rem_euclid(4),
        Direction::Right => (heading + 1).rem_euclid(4),
    }
}

fn walk(x: i32, y: i32, steps: i32, heading: i32) -> (i32, i32) {
    match heading {
        0 => (x, y + steps),
        1 => (x + steps, y),
        2 => (x, y - steps),
        3 => (x - steps, y),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![("R2, L3", 5), ("R2, R2, R2", 2), ("R5, L5, R5, R3", 12)];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 181);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![("R8, R4, R4, R8", 4)];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 140);
    }
}
