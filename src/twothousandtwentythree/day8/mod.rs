use super::{super::utils::*, YEAR};


pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
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

    fn part_two_driver(&self, input: &str) -> i32 {
        let (steps, nodes) = parse_input(input);
        let mut current_nodes = nodes
            .iter()
            .filter(|e| e.value.chars().last().unwrap() == 'A')
            .collect::<Vec<&Node>>();
        let mut cycles = vec![0; current_nodes.len()];
        let mut result = 0;
        loop {
            let mut index = 0;
            result += 1;
            for current_node in current_nodes.iter_mut() {
                if cycles.get(index).unwrap() != &0 {
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
                    *cycles.get_mut(index).unwrap() = result;
                }
                index += 1;
            }

            if cycles.iter().all(|e| e != &0) {
                break;
            }
        }

        result
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 8)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    let mut x;
    let mut y;
    if a > b {
        x = a;
        y = b;
    } else {
        x = b;
        y = a;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    a * b / y
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
