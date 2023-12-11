use std::{cell::RefCell, cmp::Ordering, fmt::Display, rc::Rc};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut u = Universe::from(input);
        u.expand(1);

        println!("{}", u);

        let mut result = 0;
        for (index, a) in u.galaxy_positions.iter().enumerate() {
            for b_index in index + 1..u.galaxy_positions.len() {
                let b = u.galaxy_positions.get(b_index).unwrap();
                if a == b {
                    continue;
                }

                let distance = u.shortest_distance(*a, *b);
                result += distance;
            }
        }

        let distance = u.shortest_distance(*a, *b);
        result += distance;
        result as i32
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut u = Universe::from(input);
        u.expand(1000000);

        let mut result = 0;
        for (index, a) in u.galaxy_positions.iter().enumerate() {
            for b_index in index + 1..u.galaxy_positions.len() {
                let b = u.galaxy_positions.get(b_index).unwrap();
                if a == b {
                    continue;
                }

                let horizontal_distance = a.x.abs_diff(b.x);
                let vertical_distance = a.y.abs_diff(b.y);

                result += horizontal_distance + vertical_distance;
            }
        }

        result as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 11)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Entity {
    Empty,
    ExpandedRow(i32),
    ExpandedColumn(i32),
    ExpandedSpace(i32),
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
    fn expand(&mut self, times: i32) {
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        let expanded_row = vec![Entity::ExpandedRow(times + 1); self.width];
        for row_index in empty_rows {
            *self.entities.get_mut(row_index).unwrap() = expanded_row.clone();
        }

        for col_index in empty_cols {
            for row_index in 0..self.height {
                for _ in 0..times {
                    let e = self
                        .entities
                        .get_mut(row_index)
                        .unwrap()
                        .get_mut(col_index)
                        .unwrap();
                    *e = match e {
                        Entity::Empty => Entity::ExpandedColumn(times + 1),
                        Entity::ExpandedRow(_) => Entity::ExpandedSpace(times + 1),
                        Entity::ExpandedColumn(_) => unreachable!(),
                        Entity::ExpandedSpace(_) => unreachable!(),
                        Entity::Galaxy => unreachable!(),
                    };
                }
            }
        }
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

    fn shortest_distance(&self, start: Point, goal: Point) -> usize {
        let mut open = Vec::with_capacity(100000);
        let mut closed: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(20000);
        let mut distance = 0;
        let mut neighbors;
        open.push(self.make_node_from_entity(self.get_entity(start).unwrap(), start, start));
        'path_found: while open.len() > 0 {
            open.sort_unstable_by(|a, b| {
                if a.borrow().f < b.borrow().f {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            let current = open.pop().unwrap();
            neighbors = vec![
                Point {
                    x: current.borrow().pos.x - 1,
                    y: current.borrow().pos.y,
                },
                Point {
                    x: current.borrow().pos.x + 1,
                    y: current.borrow().pos.y,
                },
                Point {
                    x: current.borrow().pos.x,
                    y: current.borrow().pos.y - 1,
                },
                Point {
                    x: current.borrow().pos.x,
                    y: current.borrow().pos.y + 1,
                },
            ]
            .into_iter()
            .filter(|p| self.get_entity(*p).is_some())
            .map(|p| {
                self.make_node_from_entity(self.get_entity(p).unwrap(), p, current.borrow().pos)
            })
            .collect::<Vec<Rc<RefCell<Node>>>>();

            for neighbor in neighbors.iter_mut() {
                {
                    let mut neighbor_mut = neighbor.borrow_mut();
                    neighbor_mut.parent = Some(current.clone());
                    neighbor_mut.g = current.borrow().g + neighbor_mut.weight;
                    neighbor_mut.h = (((neighbor_mut.pos.x - goal.x).pow(2)
                        + (neighbor_mut.pos.y - goal.y).pow(2))
                        as f32)
                        .sqrt();
                    neighbor_mut.f = neighbor_mut.g + neighbor_mut.h as i32 + neighbor_mut.weight;
                }

                if neighbor.borrow().pos == goal {
                    closed.push(neighbor.clone());
                    break 'path_found;
                }

                let same_pos_open_nodes = open
                    .iter()
                    .filter(|node| node.borrow().pos == neighbor.borrow().pos);
                match same_pos_open_nodes.min_by_key(|node| node.borrow().f) {
                    Some(n) => {
                        if n.borrow().f >= neighbor.borrow().f {
                            let same_pos_closed_nodes = closed
                                .iter()
                                .filter(|node| node.borrow().pos == neighbor.borrow().pos);

                            match same_pos_closed_nodes.min_by_key(|node| node.borrow().f) {
                                Some(n) => {
                                    if n.borrow().f >= neighbor.borrow().f {
                                        open.push(neighbor.clone());
                                    }
                                }
                                None => open.push(neighbor.clone()),
                            }
                        }
                    }
                    None => open.push(neighbor.clone()),
                }
            }

            closed.push(current.clone());
        }

        let mut current = closed.pop().unwrap();
        while current.borrow().parent.is_some() {
            distance += current.borrow().weight;
            let next = current.borrow().parent.clone().unwrap().clone();
            current = next;
        }

        distance.try_into().unwrap()
    }

    fn get_entity(&self, p: Point) -> Option<Entity> {
        if p.x < 0 || p.x >= self.width as i32 || p.y < 0 || p.y >= self.height as i32 {
            None
        } else {
            Some(
                *self
                    .entities
                    .get(p.y as usize)
                    .unwrap()
                    .get(p.x as usize)
                    .unwrap(),
            )
        }
    }

    fn make_node_from_entity(&self, entity: Entity, pos: Point, from: Point) -> Rc<RefCell<Node>> {
        let weight = match entity {
            Entity::Empty => 1,
            Entity::ExpandedRow(x) => {
                if pos.y.abs_diff(from.y) == 0 {
                    1
                } else {
                    x
                }
            }
            Entity::ExpandedColumn(x) => {
                if pos.x.abs_diff(from.x) == 0 {
                    1
                } else {
                    x
                }
            }
            Entity::ExpandedSpace(x) => x,
            Entity::Galaxy => 1,
        };

        Rc::new(RefCell::new(Node {
            weight,
            pos,
            parent: None,
            f: 0,
            g: 0,
            h: 0.,
        }))
    }
}

#[derive(Clone)]
struct Node {
    weight: i32,
    f: i32,
    parent: Option<Rc<RefCell<Node>>>,
    g: i32,
    h: f32,
    pos: Point,
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
                    &Entity::ExpandedSpace(_) => 'S',
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
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_one(), 123);
    }

    #[test]
    fn part_two_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_two(), 123);
    }
}
