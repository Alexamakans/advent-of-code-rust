use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, input: &str) -> u64 {
        let packages = input
            .lines()
            .map(|e| e.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let configurations = generate_configurations(packages);
        configurations.first().unwrap().groups[0].quantum_entanglement
    }

    fn part_two_driver(&self, input: &str) -> u64 {
        todo!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 24)
    }
}

pub fn generate_configurations(mut packages: Vec<u8>) -> Vec<Configuration> {
    packages.sort_by(|a, b| b.cmp(&a));
    let value_per_group = packages.iter().fold(0, |acc, cur| acc as u16 + *cur as u16) / 3;
    let mut best_length = usize::MAX;
    let mut best_quantum_entanglement = u64::MAX;
    let mut configurations = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start_configuration = Configuration::new(packages);
    queue.push_back(start_configuration);
    let mut iterations = 0;
    while let Some(configuration) = queue.pop_front() {
        iterations += 1;
        if (iterations & 0b11111111111111111) == 0 { // about once every 125,000 iterations
            println!("Number in queue: {}", queue.len());
        }
        let shortest_group = &configuration.groups[0];
        match shortest_group.len().cmp(&best_length) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => {
                if shortest_group.quantum_entanglement >= best_quantum_entanglement {
                    continue;
                }
            }
            std::cmp::Ordering::Greater => continue,
        };

        if configuration.is_valid(value_per_group) {
            best_length = shortest_group.len();
            best_quantum_entanglement = shortest_group.quantum_entanglement;
            configurations.push(configuration);

            configurations.sort_by(|a, b| match a.groups[0].len().cmp(&b.groups[0].len()) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => a.groups[0]
                    .quantum_entanglement
                    .cmp(&b.groups[0].quantum_entanglement),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            });
            println!("-------------------------------------------------------------------");
            for x in configurations.iter() {
                println!(
                    "{:?}  (QE={})\t{:?}\t{:?}",
                    x.groups[0].packages,
                    x.groups[0].quantum_entanglement,
                    x.groups[1].packages,
                    x.groups[2].packages
                );
            }
            continue;
        }

        let mut hasher = DefaultHasher::new();
        configuration.groups[0].hash(&mut hasher);
        if encountered_before(hasher.finish(), &mut visited) {
            continue;
        }

        for (group_index, group) in configuration.groups.iter().enumerate() {
            for package in group.packages_to_select_from.iter() {
                let mut configuration = configuration.clone();
                configuration.add_package(group_index, *package, value_per_group);
                configuration.tidy();
                // less memory used by storing hash in visited instead
                // maybe faster too since we don't allocate another Configuration
                let mut hasher = DefaultHasher::new();
                configuration.hash(&mut hasher);
                if encountered_before(hasher.finish(), &mut visited) {
                    continue;
                }

                queue.push_back(configuration);
                queue.make_contiguous().sort_unstable_by(|a, b| {
                    match a.groups[0]
                        .packages_to_select_from
                        .len()
                        .cmp(&b.groups[0].packages_to_select_from.len())
                    {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => match a.groups[1]
                            .packages_to_select_from
                            .len()
                            .cmp(&b.groups[1].packages_to_select_from.len())
                        {
                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => match a.groups[2]
                                .packages_to_select_from
                                .len()
                                .cmp(&b.groups[2].packages_to_select_from.len())
                            {
                                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
                                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                            },
                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        },
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    }
                });
            }
        }
    }

    configurations
}

#[inline]
fn encountered_before(value: u64, visited: &mut HashSet<u64>) -> bool {
    !visited.insert(value)
}

fn generate_best_first_group_packages(mut packages: Vec<u64>) -> Vec<u64> {
    fn f(packages: Vec<u64>, value: u64, value_per_group: u64) -> Option<Vec<u64>> {
        for p in packages.iter() {
            let v = value + *p;
            if v != value_per_group {
                if v < value_per_group {
                    match f(
                        packages.clone().into_iter().filter(|e| e != p).collect(),
                        v,
                        value_per_group,
                    ) {
                        Some(res) => return Some(res),
                        None => (),
                    };
                }
            } else {
                return Some(
                    packages
                        .clone()
                        .into_iter()
                        .filter(|e| e != p)
                        .collect::<Vec<u64>>(),
                );
            }
        }
        None
    }

    packages.sort_by(|a, b| b.cmp(&a));
    let value_per_group = packages.iter().fold(0, |acc, cur| acc + *cur) / 3;
    let not_taken = f(packages.clone(), 0, value_per_group).unwrap();
    packages.retain(|e| !not_taken.contains(e));
    packages
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
        // println!(
        //     "{:?}",
        //     generate_configurations(vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]).into_iter().filter(|e| {
        //         let a = e[0].clone().into_iter().reduce(|acc, cur| acc + cur).unwrap();
        //         let b = e[1].clone().into_iter().reduce(|acc, cur| acc + cur).unwrap();
        //         let c = e[2].clone().into_iter().reduce(|acc, cur| acc + cur).unwrap();

        //         a == b && b == c
        //     }).next().unwrap(),
        // );

        let configurations = generate_configurations(vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]);
        println!("final result -----------------------------");
        for x in configurations {
            println!(
                "{:?}  (QE={})\t{:?}\t{:?}",
                x.groups[0].packages,
                x.groups[0].quantum_entanglement,
                x.groups[1].packages,
                x.groups[2].packages
            );
        }
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

    #[test]
    fn test_group_works() {
        let mut group = Group::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let value_per_group = 8;

        group.add_package(5, value_per_group);
        assert_eq!(group.packages_to_select_from, vec![1, 2, 3]);
    }

    #[test]
    fn test_sort_groups_works() {
        // get group with shortest length or if equal exists then lowest quantum entanglement
        let mut configuration = Configuration {
            groups: [
                Group {
                    packages: vec![7, 5, 4, 3, 1],
                    weight: 20,
                    quantum_entanglement: 420,
                    packages_to_select_from: vec![],
                },
                Group {
                    packages: vec![10, 7, 3],
                    weight: 20,
                    quantum_entanglement: 210,
                    packages_to_select_from: vec![],
                },
                Group {
                    packages: vec![10, 8, 2],
                    weight: 20,
                    quantum_entanglement: 160,
                    packages_to_select_from: vec![],
                },
            ],
        };
        configuration
            .groups
            .sort_by(|a, b| match a.len().cmp(&b.len()) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => {
                    let a_qe = a.packages.iter().fold(1, |acc, cur| acc * cur);
                    let b_qe = b.packages.iter().fold(1, |acc, cur| acc * cur);
                    a_qe.cmp(&b_qe)
                }
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            });

        assert_eq!(
            configuration.groups[0],
            Group {
                packages: vec![10, 8, 2],
                weight: 20,
                quantum_entanglement: 160,
                packages_to_select_from: vec![],
            },
        );
    }

    #[test]
    fn test_get_best_configuration_works() {
        let mut configurations = vec![
            Configuration {
                groups: [
                    Group {
                        packages: vec![7, 5, 4, 3, 1],
                        weight: 20,
                        quantum_entanglement: 420,
                        packages_to_select_from: vec![],
                    },
                    Group {
                        packages: vec![10, 7, 3],
                        weight: 20,
                        quantum_entanglement: 210,
                        packages_to_select_from: vec![],
                    },
                    Group {
                        packages: vec![10, 8, 2],
                        weight: 20,
                        quantum_entanglement: 160,
                        packages_to_select_from: vec![],
                    },
                ],
            },
            Configuration {
                groups: [
                    Group {
                        packages: vec![20],
                        weight: 20,
                        quantum_entanglement: 20,
                        packages_to_select_from: vec![],
                    },
                    Group {
                        packages: vec![10, 7, 3],
                        weight: 20,
                        quantum_entanglement: 210,
                        packages_to_select_from: vec![],
                    },
                    Group {
                        packages: vec![10, 8, 2],
                        weight: 20,
                        quantum_entanglement: 160,
                        packages_to_select_from: vec![],
                    },
                ],
            },
        ];

        for configuration in configurations.iter_mut() {
            configuration.tidy();
        }

        let mut want = Configuration {
            groups: [
                Group {
                    packages: vec![20],
                    weight: 20,
                    quantum_entanglement: 20,
                    packages_to_select_from: vec![],
                },
                Group {
                    packages: vec![10, 7, 3],
                    weight: 20,
                    quantum_entanglement: 210,
                    packages_to_select_from: vec![],
                },
                Group {
                    packages: vec![10, 8, 2],
                    weight: 20,
                    quantum_entanglement: 160,
                    packages_to_select_from: vec![],
                },
            ],
        };

        want.tidy();

        configurations.sort_by(|a, b| match a.groups[0].len().cmp(&b.groups[0].len()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a.groups[0]
                .quantum_entanglement
                .cmp(&b.groups[0].quantum_entanglement),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        });

        assert_eq!(configurations[0], want);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Configuration {
    groups: [Group; 3],
}

impl Configuration {
    fn new(packages_to_pick_from: Vec<u8>) -> Self {
        Configuration {
            groups: [
                Group::new(packages_to_pick_from.clone()),
                Group::new(packages_to_pick_from.clone()),
                Group::new(packages_to_pick_from),
            ],
        }
    }

    fn add_package(&mut self, group_index: usize, package: u8, value_per_group: u16) {
        self.groups[group_index].add_package(package, value_per_group);
        for (i, group) in self.groups.iter_mut().enumerate() {
            if i == group_index {
                continue;
            }

            group.packages_to_select_from.retain(|e| *e != package);
        }
    }

    fn tidy(&mut self) {
        self.groups.sort_by(|a, b| match a.len().cmp(&b.len()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a.quantum_entanglement.cmp(&b.quantum_entanglement),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        });

        for group in self.groups.iter_mut() {
            group.packages.sort_by(|a, b| b.cmp(&a));
        }
    }

    fn is_valid(&self, value_per_group: u16) -> bool {
        for group in self.groups.iter() {
            if group.weight != value_per_group {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Group {
    packages: Vec<u8>,
    weight: u16,
    quantum_entanglement: u64,
    packages_to_select_from: Vec<u8>,
}

impl std::hash::Hash for Group {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.packages.hash(state);
        self.packages_to_select_from.hash(state);
    }
}

impl Group {
    fn new(packages_to_select_from: Vec<u8>) -> Self {
        Group {
            packages: Vec::new(),
            weight: 0,
            quantum_entanglement: 1,
            packages_to_select_from,
        }
    }

    fn add_package(&mut self, package: u8, value_per_group: u16) {
        self.weight += package as u16;
        self.quantum_entanglement *= package as u64;
        self.packages.push(package);
        self.packages_to_select_from
            .retain(|e| *e != package && *e as u16 + self.weight <= value_per_group);
    }

    fn len(&self) -> usize {
        self.packages.len()
    }
}
