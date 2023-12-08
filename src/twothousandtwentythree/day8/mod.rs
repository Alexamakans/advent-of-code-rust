use std::collections::HashSet;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, input: &str) -> u64 {
        let (steps, nodes) = parse_input(input);
        let mut current_node = nodes.iter().find(|e| e.value == "AAA").unwrap();
        let mut result = 0;
        loop {
            match current_node.value.as_str() {
                "ZZZ" => {
                    break;
                }
                _ => {
                    let step_index = result as usize % steps.len();
                    match &steps[step_index] {
                        Step::Left => {
                            for node in nodes.iter() {
                                if node.value == current_node.left {
                                    current_node = node;
                                    break;
                                }
                            }
                        }
                        Step::Right => {
                            for node in nodes.iter() {
                                if node.value == current_node.right {
                                    current_node = node;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            result += 1;
        }

        result
    }

    fn part_two_driver(&self, input: &str) -> u64 {
        let (steps, nodes) = parse_input(input);
        let mut current_nodes = nodes
            .iter()
            .filter(|e| e.value.chars().last().unwrap() == 'A')
            .collect::<Vec<&Node>>();
        println!("found {} start nodes", current_nodes.len());
        let mut cycles = vec![0; current_nodes.len()];
        let mut result = 0;
        loop {
            let mut index = 0;
            result += 1;
            for current_node in current_nodes.iter_mut() {
                if *cycles.get(index).unwrap() != 0 {
                    index += 1;
                    continue;
                }

                let step_index = (result - 1) as usize % steps.len();
                match &steps[step_index] {
                    Step::Left => {
                        for node in nodes.iter() {
                            if node.value == current_node.left {
                                *current_node = node;
                                break;
                            }
                        }
                    }
                    Step::Right => {
                        for node in nodes.iter() {
                            if node.value == current_node.right {
                                *current_node = node;
                                break;
                            }
                        }
                    }
                }

                if current_node.value.chars().last().unwrap() == 'Z' {
                    *cycles.get_mut(index).unwrap() = result as u64;
                    println!("{},", result);
                }
                index += 1;
            }

            if cycles.iter().all(|e| *e != 0) {
                println!();
                break;
            }
        }

        cycles.iter().fold(1, |a, b| lcm(a, *b))
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 8)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * (b / gcd(a, b));
}

struct Node {
    pub value: String,
    pub left: String,
    pub right: String,
}

enum Step {
    Left,
    Right,
}

fn parse_input(s: &str) -> (Vec<Step>, Vec<Node>) {
    let clean_string = s.replace("\r", "");
    let (steps_input, nodes_input) = clean_string.split_once("\n\n").unwrap();
    let steps = steps_input
        .chars()
        .map(|c| match c {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<Step>>();

    let nodes = nodes_input
        .lines()
        .map(|line| {
            let clean_line = line
                .replace("= ", "")
                .replace(&['(', ',', ')'], "")
                .replace(&['\n', '\r'], "");
            let mut parts = clean_line.split_whitespace();
            Node {
                value: parts.next().unwrap().to_string(),
                left: parts.next().unwrap().to_string(),
                right: parts.next().unwrap().to_string(),
            }
        })
        .collect::<Vec<Node>>();

    (steps, nodes)
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
