use crate::{scanf, twothousandfifteen::YEAR, utils::*};
use std::{cell::RefCell, rc::Rc};

type LocationType = Rc<RefCell<Location>>;
type ConnectionType = Rc<RefCell<Connection>>;
type Locations = Rc<RefCell<Vec<LocationType>>>;

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        let locations = parse_locations(&input);

        let mut shortest_distance = u32::MAX;
        for permutation in locations.borrow().clone().into_permutations() {
            'invalid_path: {
                let pairs = permutation.into_pairs();
                let mut total_distance = 0;
                for pair in pairs {
                    let connection = {
                        match pair.0.borrow().get_connection(pair.1.clone()) {
                            Some(c) => c,
                            None => break 'invalid_path,
                        }
                    };

                    total_distance += connection.borrow().distance;
                }

                if total_distance < shortest_distance {
                    shortest_distance = total_distance;
                }
            }
        }

        shortest_distance as i32
    }

    fn part_two_driver(&self, input: String) -> i32 {
        let locations = parse_locations(&input);

        let mut longest_distance = u32::MIN;
        for permutation in locations.borrow().clone().into_permutations() {
            'invalid_path: {
                // [1, 2, 3, 4, 5] -> [1, 2, 3, 4] zip [2, 3, 4, 5] -> [[1, 2], [2, 3], [3, 4], [4, 5]]
                let pairs = permutation.into_pairs();
                let mut total_distance = 0;
                for pair in pairs {
                    let connection = {
                        match pair.0.borrow().get_connection(pair.1.clone()) {
                            Some(c) => c,
                            None => break 'invalid_path,
                        }
                    };

                    total_distance += connection.borrow().distance;
                }

                if total_distance > longest_distance {
                    longest_distance = total_distance;
                }
            }
        }

        longest_distance as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 9)
    }
}

#[derive(Debug)]
struct Connection {
    to: Rc<RefCell<Location>>,
    distance: u32,
}

impl Connection {
    fn new(to: Rc<RefCell<Location>>, distance: u32) -> Self {
        Connection { to, distance }
    }
}

#[derive(Debug, Clone)]
struct Location {
    name: String,
    connections: Rc<RefCell<Vec<Rc<RefCell<Connection>>>>>,
}

impl Location {
    fn new(name: &str) -> Self {
        Location {
            name: name.to_owned(),
            connections: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn get_connection(&self, other: LocationType) -> Option<ConnectionType> {
        for c in self.connections.borrow().iter() {
            if c.borrow().to.borrow().name == other.borrow().name {
                return Some(c.clone());
            }
        }
        
        for c in other.borrow().connections.borrow().iter() {
            if c.borrow().to.borrow().name == self.name {
                return Some(c.clone());
            }
        }

        None
    }
}

fn get_location(
    locations: Rc<RefCell<Vec<Rc<RefCell<Location>>>>>,
    name: &str,
) -> Rc<RefCell<Location>> {
    for location in locations.borrow().iter() {
        if location.borrow().name == name {
            return location.clone();
        }
    }

    locations
        .borrow_mut()
        .push(Rc::new(RefCell::new(Location::new(name))));
    get_location(locations, name)
}

fn parse_locations(input: &str) -> Locations {
    let locations = Rc::new(RefCell::new(Vec::new()));
    for line in input.lines() {
        let parsed = scanf!(
            line,
            char::is_whitespace,
            String, // A
            String, // ->
            String, // B
            String, // =
            u32     // 123
        );
        let (a_name, b_name, distance) = (parsed.0, parsed.2, parsed.4);
        let a = get_location(locations.clone(), &a_name);
        let b = get_location(locations.clone(), &b_name);

        a.clone()
            .borrow()
            .connections
            .borrow_mut()
            .push(Rc::new(RefCell::new(Connection::new(b, distance))));
    }
    locations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#,
            605,
        )];

        for case in cases {
            assert_eq!(
                solver.part_one_driver(String::from(case.0)),
                case.1,
                "input = {}",
                case.0
            );
        }

        assert_eq!(solver.part_one(), 251);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#,
            982,
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(String::from(case.0)), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 898);
    }
}
