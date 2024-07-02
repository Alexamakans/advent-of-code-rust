use std::str::FromStr;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, input: &str) -> u64 {
        let mut steps = Vec::new();
        for line in input.lines() {
            steps.push(line.parse::<Step>().unwrap());
        }

        solve(steps)
    }

    fn part_two_driver(&self, input: &str) -> u64 {
        let mut steps = Vec::new();
        for line in input.lines() {
            steps.push(Step::from_hex(line.split_whitespace().last().unwrap()));
        }

        solve(steps)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 18)
    }
}

fn solve(steps: Vec<Step>) -> u64 {
    let mut points = Vec::new();
    let mut pos = (0, 0);
    points.push(pos);

    for step in steps.iter() {
        for _ in 0..step.amt {
            step.step(&mut pos.0, &mut pos.1);
            points.push(pos);
        }
    }

    // wrap around
    points.push(*points.first().unwrap());

    // shoelace
    let mut area = 0 as i128;
    for i in 0..points.len() {
        let xi = points.get(i).unwrap().0;
        let yim1 = if i > 0 {
            points.get(i - 1).unwrap().1
        } else {
            0
        };
        let yip1 = if i < points.len() - 1 {
            points.get(i + 1).unwrap().1
        } else {
            0
        };

        area += xi as i128 * (yip1 - yim1) as i128;
    }

    (0.5 * area as f64).abs() as u64 + (points.len() as u64 / 2)
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => unreachable!(),
        }
    }
}

struct Step {
    dir: Direction,
    amt: u32,
}

impl Step {
    fn step(&self, x: &mut i64, y: &mut i64) {
        match self.dir {
            Direction::Up => {
                *y -= 1;
            }
            Direction::Right => {
                *x += 1;
            }
            Direction::Down => {
                *y += 1;
            }
            Direction::Left => {
                *x -= 1;
            }
        }
    }

    fn from_hex(hex: &str) -> Self {
        Step {
            amt: u32::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap(),
            dir: match hex.chars().nth_back(1).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            },
        }
    }
}

impl FromStr for Step {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split_whitespace();
        Ok(Step {
            dir: parts.next().unwrap().parse::<Direction>().unwrap(),
            amt: parts.next().unwrap().parse::<u32>().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_one(), 40761);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 106920098354636);
    }
}
