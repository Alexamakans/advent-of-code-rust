use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut world = World::from(input);
        println!("world before tilt north:\n{}", world);
        world.tilt_north();
        println!("world after tilt north:\n{}", world);

        world.calculate_load()
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut world = World::from(input);
        let mut visited = HashSet::new();

        let target_cycle = 1_000_000_000;
        for current_cycle in 0..target_cycle {
            if visited.insert(format!("{}", world)) {
                world.tilt_north();
                world.tilt_west();
                world.tilt_south();
                world.tilt_east();
            } else {
                // cycle encountered
                visited.clear();
                for current_cycle in current_cycle..target_cycle {
                    if visited.insert(format!("{}", world)) {
                        world.tilt_north();
                        world.tilt_west();
                        world.tilt_south();
                        world.tilt_east();
                    } else {
                        let cycle_length = visited.len();
                        let remaining_cycles = target_cycle - current_cycle;
                        let remaining_steps = remaining_cycles % cycle_length;

                        for _ in 0..remaining_steps {
                            world.tilt_north();
                            world.tilt_west();
                            world.tilt_south();
                            world.tilt_east();
                        }

                        return world.calculate_load();
                    }
                }
            }
        }

        world.calculate_load()
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 14)
    }
}

#[derive(Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
            Tile::Empty => '.',
        }
    }
}

impl<'a> From<&'a Tile> for char {
    fn from(value: &'a Tile) -> Self {
        match value {
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
            Tile::Empty => '.',
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

struct World {
    rows: Vec<Vec<Tile>>,
}

impl World {
    fn tilt_north(&mut self) {
        let num_tiles = self.rows.first().unwrap().len();
        let mut moved = true;
        while moved {
            moved = false;
            for destination_index in 0..self.rows.len() - 1 {
                let source_index = destination_index + 1;
                for tile_index in 0..num_tiles {
                    moved |= self._boosh(destination_index, source_index, tile_index, tile_index);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let num_tiles = self.rows.first().unwrap().len();
        let mut moved = true;
        while moved {
            moved = false;
            for destination_index in (1..self.rows.len()).rev() {
                let source_index = destination_index - 1;
                for tile_index in 0..num_tiles {
                    moved |= self._boosh(destination_index, source_index, tile_index, tile_index);
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let num_tiles = self.rows.first().unwrap().len();
        let mut moved = true;
        while moved {
            moved = false;
            for row_index in 0..self.rows.len() {
                for destination_index in (1..num_tiles).rev() {
                    let source_index = destination_index - 1;
                    moved |= self._boosh(row_index, row_index, destination_index, source_index);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let num_tiles = self.rows.first().unwrap().len();
        let mut moved = true;
        while moved {
            moved = false;
            for row_index in 0..self.rows.len() {
                for destination_index in 0..num_tiles - 1 {
                    let source_index = destination_index + 1;
                    moved |= self._boosh(row_index, row_index, destination_index, source_index);
                }
            }
        }
    }

    fn _boosh(
        &mut self,
        destination_row_index: usize,
        source_row_index: usize,
        destination_tile_index: usize,
        source_tile_index: usize,
    ) -> bool {
        let source_tile = *self
            .rows
            .get(source_row_index)
            .unwrap()
            .get(source_tile_index)
            .unwrap();

        if !matches!(source_tile, Tile::RoundRock) {
            return false;
        }

        let destination_tile = self
            .rows
            .get(destination_row_index)
            .unwrap()
            .get(destination_tile_index)
            .unwrap();

        match destination_tile {
            Tile::RoundRock => false,
            Tile::CubeRock => false,
            Tile::Empty => {
                *self
                    .rows
                    .get_mut(destination_row_index)
                    .unwrap()
                    .get_mut(destination_tile_index)
                    .unwrap() = source_tile;
                *self
                    .rows
                    .get_mut(source_row_index)
                    .unwrap()
                    .get_mut(source_tile_index)
                    .unwrap() = Tile::Empty;
                true
            }
        }
    }

    fn calculate_load(&self) -> i32 {
        self.rows
            .iter()
            .rev()
            .zip(1..=self.rows.len() as i32)
            .fold(0, |acc, (row, multiplier)| {
                acc + row.iter().filter(|e| matches!(e, Tile::RoundRock)).count() as i32
                    * multiplier
            })
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row.iter().map(char::from).collect::<String>()).expect("nooooo");
        }

        Ok(())
    }
}

impl From<&str> for World {
    fn from(value: &str) -> Self {
        World {
            rows: value
                .trim()
                .lines()
                .map(|line| line.chars().map(Tile::from).collect())
                .collect(),
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

        assert_eq!(solver.part_one(), 108840);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 103445);
    }
}
