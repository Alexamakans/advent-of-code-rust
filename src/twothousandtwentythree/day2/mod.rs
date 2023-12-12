use std::str::FromStr;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u32> for Solver {
    fn part_one_driver(&self, input: &str) -> u32 {
        let mut id_sum = 0;
        for line in input.lines() {
            let game = parse_line(line);
            let id = game.id;
            let mut is_valid = true;
            for cubes in game.cubes {
                let (r, g, b) = get_num_cubes(cubes);
                if !(r <= 12 && g <= 13 && b <= 14) {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                id_sum += id;
            }
        }

        id_sum
    }

    fn part_two_driver(&self, input: &str) -> u32 {
        let mut power_sum = 0;
        for line in input.lines() {
            let game = parse_line(line);
            let mut maxr = 0;
            let mut maxg = 0;
            let mut maxb = 0;
            for cubes in game.cubes {
                let (r, g, b) = get_num_cubes(cubes);
                if r > maxr {
                    maxr = r;
                }
                if g > maxg {
                    maxg = g;
                }
                if b > maxb {
                    maxb = b;
                }
            }

            power_sum += maxr * maxg * maxb;
        }

        power_sum
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 2)
    }
}

enum Cube {
    Red,
    Green,
    Blue,
}
#[derive(Debug, PartialEq, Eq)]
struct ParseCubeError;
impl FromStr for Cube {
    type Err = ParseCubeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Cube::Red),
            "green" => Ok(Cube::Green),
            "blue" => Ok(Cube::Blue),
            _ => Err(ParseCubeError {}),
        }
    }
}

struct Game {
    id: u32,
    cubes: Vec<Vec<(u32, Cube)>>,
}

fn get_num_cubes(cubes: Vec<(u32, Cube)>) -> (u32, u32, u32) {
    let mut ramt = 0;
    let mut gamt = 0;
    let mut bamt = 0;
    for cubeset in cubes {
        match cubeset.1 {
            Cube::Red => ramt += cubeset.0,
            Cube::Green => gamt += cubeset.0,
            Cube::Blue => bamt += cubeset.0,
        }
    }

    (ramt, gamt, bamt)
}

fn parse_line(s: &str) -> Game {
    let mut parts = s.split_whitespace();
    let id: u32 = parts.nth(1).unwrap().replace(":", "").parse().unwrap();
    let rest = parts.collect::<Vec<&str>>().join(" ");
    let rounds = rest.split(";");
    let sets = rounds.map(|v| v.split(", "));
    let mut game = Game {
        id,
        cubes: Vec::new(),
    };

    for set in sets {
        let mut cubes = Vec::new();
        for e in set {
            let mut parts = e.split_whitespace();
            let amt = parts.next().unwrap().parse::<u32>().unwrap();
            let cube = parts.next().unwrap().parse::<Cube>().unwrap();
            cubes.push((amt, cube));
        }
        if cubes.len() > 0 {
            game.cubes.push(cubes);
        }
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                2,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                0,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                0,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 2685);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 83707);
    }
}
