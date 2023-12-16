use std::{collections::HashSet, fmt::Display};

use crate::parse_world;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let world = World::from(input);
        let mut disp = format!("{}", world);

        println!("{}", disp);
        println!("\n");

        let mut beams = Beams {
            visited_positions: HashSet::new(),
            visited_states: HashSet::new(),
            beams: vec![Beam {
                done: false,
                position: [0, 0],
                velocity: [1, 0],
            }],
        };

        while beams.update(&world) {}

        for state in beams.visited_states.iter() {
            let (x, y) = (state.0 as usize, state.1 as usize);
            let mut new_disp = disp.lines().map(str::to_string).collect::<Vec<String>>();
            new_disp.get_mut(y).unwrap().replace_range(x..=x, "#");
            disp = new_disp.join("\n");
        }

        println!("{}", disp);

        beams.get_num_tiles_visited()
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let world = World::from(input);
        let width = world.tiles.first().unwrap().len();
        let height = world.tiles.len();

        vec![0; width]
            .into_iter()
            .zip(0..width)
            .zip(vec![(0, 1); width].into_iter())
            .chain(
                vec![height - 1; width]
                    .into_iter()
                    .zip(0..width)
                    .zip(vec![(0, -1); width].into_iter()),
            )
            .chain(
                (0..height)
                    .zip(vec![0; height].into_iter())
                    .zip(vec![(1, 0); height].into_iter()),
            )
            .chain(
                (0..height)
                    .zip(vec![width - 1; height].into_iter())
                    .zip(vec![(-1, 0); height].into_iter()),
            )
            .map(|((y, x), (vx, vy))| {
                let mut beams = Beams {
                    visited_positions: HashSet::new(),
                    visited_states: HashSet::new(),
                    beams: vec![Beam {
                        done: false,
                        position: [x as i32, y as i32],
                        velocity: [vx, vy],
                    }],
                };

                while beams.update(&world) {}

                beams.get_num_tiles_visited()
            })
            .max()
            .unwrap()
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 16)
    }
}

#[derive(Clone, Copy, Debug)]
struct Beam {
    velocity: [i32; 2],
    position: [i32; 2],
    done: bool,
}

impl Beam {
    fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    }
}

struct Beams {
    visited_positions: HashSet<(i32, i32)>,
    visited_states: HashSet<(i32, i32, i32, i32)>,
    beams: Vec<Beam>,
}

impl Beams {
    fn update(&mut self, world: &World) -> bool {
        let width = world.tiles.first().unwrap().len();
        let height = world.tiles.len();
        let mut new_visited = false;
        let mut beams_to_add = Vec::new();
        for beam in self.beams.iter_mut() {
            let x = beam.position[0];
            let y = beam.position[1];
            let vx = beam.velocity[0];
            let vy = beam.velocity[1];

            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }

            self.visited_positions
                .insert((beam.position[0], beam.position[1]));

            if self.visited_states.insert((
                beam.position[0],
                beam.position[1],
                beam.velocity[0],
                beam.velocity[1],
            )) {
                new_visited = true;
            } else {
                beam.done = true;
                continue;
            }

            let tile = world
                .tiles
                .get(y as usize)
                .unwrap()
                .get(x as usize)
                .unwrap();

            (beam.velocity[0], beam.velocity[1]) = match tile {
                Tile::Empty => (vx, vy),
                Tile::MirrorUp => (-vy, -vx),
                Tile::MirrorDown => (vy, vx),
                Tile::SplitterVertical => match (vx, vy) {
                    (1, 0) | (-1, 0) => {
                        let mut new_beam = *beam;
                        new_beam.velocity[0] = 0;
                        new_beam.velocity[1] = -1;
                        beams_to_add.push(new_beam);
                        (0, 1)
                    }
                    (0, 1) | (0, -1) => (vx, vy),
                    _ => unreachable!(),
                },
                Tile::SplitterHorizontal => match (vx, vy) {
                    (1, 0) | (-1, 0) => (vx, vy),
                    (0, 1) | (0, -1) => {
                        let mut new_beam = *beam;
                        new_beam.velocity[0] = -1;
                        new_beam.velocity[1] = 0;
                        beams_to_add.push(new_beam);
                        (1, 0)
                    }
                    _ => unreachable!(),
                },
            }
        }

        self.beams.retain(|beam| !beam.done);
        self.beams.append(&mut beams_to_add);
        self.beams.iter_mut().for_each(|beam| beam.update());

        new_visited
    }

    fn get_num_tiles_visited(&self) -> i32 {
        self.visited_positions.len() as i32
    }
}

parse_world!(
    '.' => Empty
    '/' => MirrorUp
    '\\'=> MirrorDown
    '|' => SplitterVertical
    '-' => SplitterHorizontal
);

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

        assert_eq!(solver.part_one(), 7728);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 8061);
    }
}
