use std::fmt::Display;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u128> for Solver {
    fn part_one_driver(&self, input: &str) -> u128 {
        let mut u = Universe::from(input);
        u.expand(2);
        println!("{}", u);

        u.calc_sum_of_all_shortest_distances()
    }

    fn part_two_driver(&self, input: &str) -> u128 {
        let mut u = Universe::from(input);
        u.expand(1_000_000);
        println!("{}", u);
        u.calc_sum_of_all_shortest_distances()
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 11)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Entity {
    Empty,
    ExpandedRow(usize),
    ExpandedColumn(usize),
    Galaxy,
}

impl From<char> for Entity {
    fn from(c: char) -> Self {
        match c {
            '.' => Entity::Empty,
            '#' => Entity::Galaxy,
            _ => panic!("failed parsing '{}' to Entity", c),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Universe {
    width: usize,
    height: usize,
    entities: Vec<Vec<Entity>>,
    galaxy_positions: Vec<Point>,
}

impl Universe {
    fn expand(&mut self, times: usize) {
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        let expanded_row = vec![Entity::ExpandedRow(times); self.width];
        for row_index in empty_rows {
            *self.entities.get_mut(row_index).unwrap() = expanded_row.clone();
        }

        for col_index in empty_cols {
            for row_index in 0..self.height {
                for _ in 0..times {
                    *self
                        .entities
                        .get_mut(row_index)
                        .unwrap()
                        .get_mut(col_index)
                        .unwrap() = Entity::ExpandedColumn(times);
                }
            }
        }

        self.galaxy_positions.clear();
        for (y, row) in self.entities.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                if entity == &Entity::Galaxy {
                    self.galaxy_positions.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
    }

    fn calc_sum_of_all_shortest_distances(&self) -> u128 {
        let mut result = 0;
        for (index, a) in self.galaxy_positions.iter().enumerate() {
            for b_index in index + 1..self.galaxy_positions.len() {
                let b = self.galaxy_positions.get(b_index).unwrap();
                if a == b {
                    continue;
                }

                let mut distance = 0;
                let dx = (b.x - a.x).signum();
                let mut x = a.x + dx;
                while x != b.x {
                    let entity = self
                        .entities
                        .get(a.y as usize)
                        .unwrap()
                        .get(x as usize)
                        .unwrap();
                    distance += match entity {
                        Entity::Empty => 1,
                        Entity::ExpandedRow(size) => *size,
                        Entity::ExpandedColumn(size) => *size,
                        Entity::Galaxy => 1,
                    };
                    x += dx;
                }

                if dx != 0 {
                    let entity = self
                        .entities
                        .get(a.y as usize)
                        .unwrap()
                        .get(x as usize)
                        .unwrap();
                    distance += match entity {
                        Entity::Empty => 1,
                        Entity::ExpandedRow(size) => *size,
                        Entity::ExpandedColumn(size) => *size,
                        Entity::Galaxy => 1,
                    };
                }

                let dy = (b.y - a.y).signum();
                let mut y = a.y + dy;
                while y != b.y {
                    let entity = self
                        .entities
                        .get(y as usize)
                        .unwrap()
                        .get(x as usize)
                        .unwrap();
                    distance += match entity {
                        Entity::Empty => 1,
                        Entity::ExpandedRow(size) => *size,
                        Entity::ExpandedColumn(size) => *size,
                        Entity::Galaxy => 1,
                    };
                    y += dy;
                }

                if dy != 0 {
                    distance += 1; // step to the galaxy
                }

                result += distance as u128;
            }
        }

        result
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows = Vec::new();
        for (row_index, row) in self.entities.iter().enumerate() {
            let has_galaxy = 'found: loop {
                for entity in row.iter() {
                    if entity == &Entity::Galaxy {
                        break 'found true;
                    }
                }
                break 'found false;
            };

            if !has_galaxy {
                empty_rows.push(row_index);
            }
        }
        empty_rows
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        let mut empty_cols = Vec::new();

        for col_index in 0..self.width {
            let has_galaxy = 'found: loop {
                for (row_index, row) in self.entities.iter().enumerate() {
                    let entity = row
                        .get(col_index)
                        .expect(&format!("failed getting y={} x={}", row_index, col_index));
                    if entity == &Entity::Galaxy {
                        break 'found true;
                    }
                }
                break 'found false;
            };

            if !has_galaxy {
                empty_cols.push(col_index);
            }
        }
        empty_cols
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.entities.iter() {
            for entity in row.iter() {
                let c = match entity {
                    Entity::Empty => '.',
                    Entity::Galaxy => '#',
                    &Entity::ExpandedRow(_) => 'R',
                    &Entity::ExpandedColumn(_) => 'C',
                };
                write!(f, "{}", c).expect(&format!(
                    "this really should work idk, failed writing '{}'",
                    c
                ));
            }
            writeln!(f).expect("failed writing new line");
        }
        Ok(())
    }
}

impl From<&str> for Universe {
    fn from(s: &str) -> Self {
        let width = s.lines().nth(0).unwrap().chars().count();
        let height = s.lines().count();
        let entities = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Entity::from(c))
                    .collect::<Vec<Entity>>()
            })
            .collect::<Vec<Vec<Entity>>>();

        let mut galaxy_positions = Vec::new();
        for (y, row) in entities.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                if entity == &Entity::Galaxy {
                    galaxy_positions.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        Universe {
            width,
            height,
            entities,
            galaxy_positions,
        }
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

        assert_eq!(solver.part_one(), 9605127);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 458191688761);
    }
}
