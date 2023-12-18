use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hasher,
};

use crate::parse_world;

use super::{super::utils::*, YEAR};

const INIT_VALUE: i32 = i32::MAX - 1_000_000;

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let world = World::from(input);
        let start = Position::new(0, 0);
        let goal = Position::new(
            world.tiles.first().unwrap().len() as i32 - 1,
            world.tiles.len() as i32 - 1,
        );

        world
            .astar(start, goal, 1, 3)
            .iter()
            .fold(0, |acc, node| acc + node.heat)
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let world = World::from(input);
        let start = Position::new(0, 0);
        let goal = Position::new(
            world.tiles.first().unwrap().len() as i32 - 1,
            world.tiles.len() as i32 - 1,
        );

        world
            .astar(start, goal, 4, 10)
            .iter()
            .fold(0, |acc, node| acc + node.heat)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 17)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

parse_world!(
    '0' => Zero
    '1' => One
    '2' => Two
    '3' => Three
    '4' => Four
    '5' => Five
    '6' => Six
    '7' => Seven
    '8' => Eight
    '9' => Nine
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Horizontal,
    Vertical,
    Neither,
}

#[derive(Debug, Clone, Copy, Eq)]
struct Node {
    pos: Position,
    entered_direction: Direction,
    heat: i32,
    f: i32,
}

impl Node {
    fn new(pos: Position, entered_direction: Direction, heat: i32) -> Self {
        Node {
            pos,
            entered_direction,
            heat,
            f: INIT_VALUE,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.entered_direction == other.entered_direction
            && self.heat == other.heat
    }
}

impl std::hash::Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.entered_direction.hash(state);
        self.heat.hash(state);
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl World {
    fn astar(
        &self,
        start: Position,
        goal: Position,
        min_step_range: usize,
        max_step_range: usize,
    ) -> Vec<Node> {
        // TODO: Generalize so it can be reused. Stuff like passing in a heuristic function and a
        // neighbor/candidate function, and returning Nodes that implement a specific trait or
        // something. I really cba writing this stuff again.
        let mut open_set = BinaryHeap::new();
        let mut start_node = Node::new(start, Direction::Neither, 0);
        start_node.f = 0;
        open_set.push(start_node);

        let mut came_from: HashMap<Node, Node> = HashMap::new();

        let mut g_score = HashMap::new();
        g_score.insert(start_node, 0);

        let mut nodes_visited = 0;
        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                let mut p = VecDeque::new();
                p.push_front(current);
                let mut current = current;
                while let Some(next) = came_from.get(&current) {
                    p.push_front(*next);
                    current = *next;
                }
                return p.into_iter().collect::<Vec<Node>>();
            }

            let cur_g_score = match g_score.get(&current) {
                Some(g) => *g,
                None => INIT_VALUE,
            };

            for mut candidate in self.get_candidates(current, min_step_range, max_step_range) {
                let candidate_g_score = match g_score.get(&candidate) {
                    Some(g) => *g,
                    None => INIT_VALUE,
                };
                let tmp_g_score = cur_g_score.checked_add(candidate.heat).unwrap();
                if tmp_g_score < candidate_g_score {
                    candidate.f = tmp_g_score;
                    came_from.insert(candidate, current);
                    g_score.insert(candidate, tmp_g_score);
                    if !open_set.iter().any(|n| n == &candidate) {
                        open_set.push(candidate);
                    }
                }
            }
        }

        unreachable!();
    }

    fn get_candidates(
        &self,
        node: Node,
        min_step_range: usize,
        max_step_range: usize,
    ) -> Vec<Node> {
        assert_ne!(min_step_range, 0);
        assert_ne!(max_step_range, 0);
        let min_step_range = min_step_range as i32;
        let max_step_range = max_step_range as i32;

        fn vertical(
            world: &World,
            node: Node,
            min_step_range: i32,
            max_step_range: i32,
        ) -> Vec<Node> {
            let width = world.tiles.first().unwrap().len() as i32;
            let row = world.tiles.get(node.pos.y as usize).unwrap();
            (-max_step_range..=-min_step_range)
                .chain(min_step_range..=max_step_range)
                .filter_map(move |dx| {
                    let mut heat = 0;
                    let mut mdx = dx;
                    while mdx != 0 {
                        // Entering horizontally
                        let x = node.pos.x + mdx;
                        if x < 0 || x >= width {
                            return None;
                        }
                        heat += row.get(x as usize).unwrap().to_value();
                        mdx -= mdx.signum();
                    }
                    Some(Node::new(
                        Position {
                            x: node.pos.x + dx,
                            y: node.pos.y,
                        },
                        Direction::Horizontal,
                        heat,
                    ))
                })
                .collect::<Vec<Node>>()
        }

        fn horizontal(
            world: &World,
            node: Node,
            min_step_range: i32,
            max_step_range: i32,
        ) -> Vec<Node> {
            let height = world.tiles.len() as i32;
            (-max_step_range..=-min_step_range)
                .chain(min_step_range..=max_step_range)
                .filter_map(move |dy| {
                    let mut heat = 0;
                    let mut mdy = dy;
                    while mdy != 0 {
                        let y = node.pos.y + mdy;
                        if y < 0 || y >= height {
                            return None;
                        }
                        heat += world
                            .tiles
                            .get(y as usize)
                            .unwrap()
                            .get(node.pos.x as usize)
                            .unwrap()
                            .to_value();
                        mdy -= mdy.signum();
                    }
                    Some(Node::new(
                        Position {
                            x: node.pos.x,
                            y: node.pos.y + dy,
                        },
                        Direction::Vertical,
                        heat,
                    ))
                })
                .collect::<Vec<Node>>()
        }

        match node.entered_direction {
            Direction::Horizontal => horizontal(self, node, min_step_range, max_step_range),
            Direction::Vertical => vertical(self, node, min_step_range, max_step_range),
            Direction::Neither => {
                let mut h = horizontal(self, node, min_step_range, max_step_range);
                h.append(&mut vertical(self, node, min_step_range, max_step_range));
                h
            }
        }
    }
}

impl Tile {
    fn to_value(&self) -> i32 {
        char::from(self) as i32 - '0' as i32
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

        assert_eq!(solver.part_one(), 1128);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 1268);
    }
}
