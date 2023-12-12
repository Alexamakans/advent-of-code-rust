use std::collections::{HashSet, VecDeque};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        // [y, x]
        let mut world = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| Pipe::try_from(c).unwrap())
                    .collect::<Vec<Pipe>>()
            })
            .collect::<Vec<Vec<Pipe>>>();

        let height = world.len() as i32;
        let width = world[0].len() as i32;

        for y in 0..height {
            for x in 0..width {
                let me = *world.get(y as usize).unwrap().get(x as usize).unwrap();
                let mut connections = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let direction = match Direction::try_from((dy, dx)) {
                            Ok(res) => res,
                            Err(_) => continue,
                        };
                        let y2 = y + dy;
                        let x2 = x + dx;
                        if y2 < 0 || y2 >= height {
                            continue;
                        }
                        if x2 < 0 || x2 >= width {
                            continue;
                        }
                        let other = *world.get(y2 as usize).unwrap().get(x2 as usize).unwrap();

                        if !me.connects_to_pipe(&other, direction) {
                            println!(
                                "{} at {:?} does not connect to {} at {:?} (direction = {:?})",
                                me.get_representation(),
                                (y, x),
                                other.get_representation(),
                                (y2, x2),
                                direction
                            );
                        } else {
                            connections += 1;
                        }
                    }
                }

                if connections < 2 {
                    *world
                        .get_mut(y as usize)
                        .unwrap()
                        .get_mut(x as usize)
                        .unwrap() = Pipe::Empty;
                }
            }
        }

        let start_position = 'found: loop {
            for y in 0..height as usize {
                for x in 0..width as usize {
                    if world.get(y).unwrap().get(x).unwrap() == &Pipe::StartingPosition {
                        break 'found (y, x);
                    }
                }
            }
        };

        println!("Start found at {:?}", start_position);

        let mut queue = VecDeque::new();
        queue.push_back(start_position);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(start_position);
        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();
            let me = world.get(current.0).unwrap().get(current.1).unwrap();
            println!("processing {} at {:?}", me.get_representation(), current);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let direction = match Direction::try_from((dy, dx)) {
                        Ok(res) => res,
                        Err(_) => continue,
                    };
                    let y2 = current.0 as i32 + dy;
                    let x2 = current.1 as i32 + dx;
                    if y2 < 0 || y2 >= height {
                        continue;
                    }
                    if x2 < 0 || x2 >= width {
                        continue;
                    }
                    let other = world.get(y2 as usize).unwrap().get(x2 as usize).unwrap();

                    if me.connects_to_pipe(other, direction) {
                        if !visited.insert((y2 as usize, x2 as usize)) {
                            continue;
                        }
                        println!(
                            "\tConnection to {} found at {:?}",
                            other.get_representation(),
                            (y2, x2)
                        );
                        queue.push_back((y2 as usize, x2 as usize));
                    }
                }
            }
        }

        for y in 0..height as usize {
            for x in 0..width as usize {
                if visited.iter().find(|pos| *pos == &(y, x)).is_none() {
                    *world.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::Empty;
                }
            }
        }

        for row in world.iter() {
            println!("{}", pipe_vec_to_string(row.clone()));
        }

        world.into_iter().fold(0, |acc, v| {
            acc + v
                .into_iter()
                .fold(0, |acc, pipe| acc + if pipe == Pipe::Empty { 0 } else { 1 })
        }) / 2
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        // [y, x]
        let mut world = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| Pipe::try_from(c).unwrap())
                    .collect::<Vec<Pipe>>()
            })
            .collect::<Vec<Vec<Pipe>>>();

        let height = world.len() as i32;
        let width = world[0].len() as i32;

        let mut loop_again = true;
        while loop_again {
            loop_again = false;
            for y in 0..height {
                for x in 0..width {
                    let me = *world.get(y as usize).unwrap().get(x as usize).unwrap();
                    if me == Pipe::Empty {
                        continue;
                    }
                    let mut connections = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let direction = match Direction::try_from((dy, dx)) {
                                Ok(res) => res,
                                Err(_) => continue,
                            };
                            let y2 = y + dy;
                            let x2 = x + dx;
                            if y2 < 0 || y2 >= height {
                                continue;
                            }
                            if x2 < 0 || x2 >= width {
                                continue;
                            }
                            let other = *world.get(y2 as usize).unwrap().get(x2 as usize).unwrap();

                            if me.connects_to_pipe(&other, direction) {
                                connections += 1;
                            }
                        }
                    }

                    if connections < 2 {
                        loop_again = true;
                        *world
                            .get_mut(y as usize)
                            .unwrap()
                            .get_mut(x as usize)
                            .unwrap() = Pipe::Empty;
                    }
                }
            }
        }

        let start_position = 'found: loop {
            for y in 0..height as usize {
                for x in 0..width as usize {
                    if world.get(y).unwrap().get(x).unwrap() == &Pipe::StartingPosition {
                        break 'found (y, x);
                    }
                }
            }
        };

        let mut current = start_position;
        let mut prev_direction: Option<Direction> = None;
        let mut segments = Vec::new();
        'done: loop {
            let me = world.get(current.0).unwrap().get(current.1).unwrap();
            'next: for dy in -1..=1 {
                for dx in -1..=1 {
                    let direction = match Direction::try_from((dy, dx)) {
                        Ok(res) => res,
                        Err(_) => continue,
                    };
                    if prev_direction.is_some()
                        && direction.get_opposite_direction() == prev_direction.unwrap()
                    {
                        continue;
                    }
                    let y2 = current.0 as i32 + dy;
                    let x2 = current.1 as i32 + dx;
                    if y2 < 0 || y2 >= height {
                        continue;
                    }
                    if x2 < 0 || x2 >= width {
                        continue;
                    }
                    let other = world.get(y2 as usize).unwrap().get(x2 as usize).unwrap();

                    if me.connects_to_pipe(other, direction) {
                        segments.push(Vector {
                            x0: current.1 as f32,
                            y0: current.0 as f32,
                            x1: x2 as f32,
                            y1: y2 as f32,
                        });
                        prev_direction = Some(direction);
                        current = (y2 as usize, x2 as usize);
                        if current == start_position {
                            break 'done;
                        }
                        break 'next;
                    }
                }
            }
        }

        let mut points_inside = 0;
        for y in 0..height {
            for x in 0..width {
                // let's wind some numbers
                let point = (x as f32, y as f32);
                let mut counter = 0;
                let mut point_on_line = false;
                for segment in segments.iter() {
                    if point.0 == segment.x0 && point.1 == segment.y0 {
                        point_on_line = true;
                        break;
                    }
                    if point.0 == segment.x1 && point.1 == segment.y1 {
                        point_on_line = true;
                        break;
                    }
                    if segment.y0 <= point.1 {
                        if segment.y1 > point.1 {
                            if is_left(&segment, point) > 0. {
                                counter += 1;
                            }
                        }
                    } else if segment.y1 <= point.1 {
                        if is_left(&segment, point) < 0. {
                            counter -= 1;
                        }
                    }
                }

                if !point_on_line && counter != 0 {
                    points_inside += 1;
                    *world
                        .get_mut(point.1 as usize)
                        .unwrap()
                        .get_mut(point.0 as usize)
                        .unwrap() = Pipe::Invisible;
                }
            }
        }

        for row in world.iter() {
            println!("{}", pipe_vec_to_string(row.clone()));
        }

        println!();
        println!();
        println!();

        for row in world.iter() {
            println!("{}", pipe_vec_to_string_inverse(row.clone()));
        }

        points_inside
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 10)
    }
}

fn is_left(seg: &Vector, p: (f32, f32)) -> f32 {
    (seg.x1 - seg.x0) * (p.1 - seg.y0) - (p.0 - seg.x0) * (seg.y1 - seg.y0)
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vector {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
struct ParseDirectionFromDeltasError {}
impl TryFrom<(i32, i32)> for Direction {
    type Error = ParseDirectionFromDeltasError;

    fn try_from(deltas: (i32, i32)) -> Result<Self, Self::Error> {
        match deltas {
            (1, 0) => Ok(Self::South),
            (-1, 0) => Ok(Self::North),
            (0, 1) => Ok(Self::East),
            (0, -1) => Ok(Self::West),
            _ => Err(ParseDirectionFromDeltasError {}),
        }
    }
}

impl Direction {
    fn get_opposite_direction(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Empty,
    NorthToSouth,
    WestToEast,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    StartingPosition,
    Invisible,
}

#[derive(Debug)]
struct ParsePipeError {}

impl Pipe {
    fn get_representation(&self) -> char {
        match self {
            Pipe::Empty => '.',
            Pipe::NorthToSouth => '|',
            Pipe::WestToEast => '-',
            Pipe::NorthToEast => 'L',
            Pipe::NorthToWest => 'J',
            Pipe::SouthToEast => 'F',
            Pipe::SouthToWest => '7',
            Pipe::StartingPosition => 'S',

            Pipe::Invisible => ' ',
        }
    }

    fn connects_to_pipe(&self, other: &Pipe, direction: Direction) -> bool {
        self.connects_to(direction) && other.connects_to(direction.get_opposite_direction())
    }

    fn connects_to(&self, direction: Direction) -> bool {
        match self {
            Pipe::Empty => false,
            Pipe::NorthToSouth => match direction {
                Direction::North => true,
                Direction::East => false,
                Direction::South => true,
                Direction::West => false,
            },
            Pipe::WestToEast => match direction {
                Direction::North => false,
                Direction::East => true,
                Direction::South => false,
                Direction::West => true,
            },
            Pipe::NorthToEast => match direction {
                Direction::North => true,
                Direction::East => true,
                Direction::South => false,
                Direction::West => false,
            },
            Pipe::NorthToWest => match direction {
                Direction::North => true,
                Direction::East => false,
                Direction::South => false,
                Direction::West => true,
            },
            Pipe::SouthToEast => match direction {
                Direction::North => false,
                Direction::East => true,
                Direction::South => true,
                Direction::West => false,
            },
            Pipe::SouthToWest => match direction {
                Direction::North => false,
                Direction::East => false,
                Direction::South => true,
                Direction::West => true,
            },
            Pipe::StartingPosition => true,
            Pipe::Invisible => false,
        }
    }
}

fn pipe_vec_to_string(pipes: Vec<Pipe>) -> String {
    pipes
        .iter()
        .map(|pipe| pipe.get_representation())
        .collect::<String>()
}

fn pipe_vec_to_string_inverse(pipes: Vec<Pipe>) -> String {
    pipes
        .iter()
        .map(|pipe| if pipe != &Pipe::Invisible { ' ' } else { 'X' })
        .collect::<String>()
}

impl TryFrom<char> for Pipe {
    type Error = ParsePipeError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Pipe::Empty),
            '|' => Ok(Pipe::NorthToSouth),
            '-' => Ok(Pipe::WestToEast),
            'L' => Ok(Pipe::NorthToEast),
            'J' => Ok(Pipe::NorthToWest),
            'F' => Ok(Pipe::SouthToEast),
            '7' => Ok(Pipe::SouthToWest),
            'S' => Ok(Pipe::StartingPosition),
            _ => Err(ParsePipeError {}),
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

        assert_eq!(solver.part_one(), 6812);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 527);
    }
}
