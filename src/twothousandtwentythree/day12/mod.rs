use std::collections::HashMap;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        input
            .lines()
            .into_iter()
            .map(parse_line)
            .map(|(states, groups)| count_configurations(states, groups))
            .sum::<usize>()
    }

    fn part_two_driver(&self, input: &str) -> usize {
        input
            .lines()
            .into_iter()
            .map(parse_line)
            .map(|(mut states, groups)| {
                let orig_states = states.clone();
                for _ in 0..4 {
                    states.push(State::Superposition);
                    orig_states.iter().for_each(|s| states.push(*s));
                }

                count_configurations(states, groups.repeat(5))
            })
            .sum::<usize>()
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 12)
    }
}

fn count_configurations(states: Vec<State>, groups: Vec<usize>) -> usize {
    let num_states = states.len();
    let num_groups = groups.len();
    let mut d = vec![vec![vec![0; num_states + 1]; num_groups + 1]; num_states + 1];

    d[num_states][num_groups][0] = 1;
    d[num_states][num_groups - 1][groups[num_groups - 1]] = 1;

    for pos in (0..num_states).rev() {
        for (group_index, &max_size) in groups.iter().enumerate() {
            for size in 0..=max_size {
                for &state in &[State::Empty, State::Filled] {
                    if states[pos] == state || states[pos] == State::Superposition {
                        if state == State::Empty && size == 0 {
                            d[pos][group_index][size] += d[pos + 1][group_index][0];
                        } else if state == State::Empty
                            && group_index < num_groups
                            && groups[group_index] == size
                        {
                            d[pos][group_index][size] += d[pos + 1][group_index + 1][0];
                        } else if state == State::Filled {
                            d[pos][group_index][size] += d[pos + 1][group_index][size + 1];
                        }
                    }
                }
            }
        }

        if matches!(states[pos], State::Empty | State::Superposition) {
            d[pos][num_groups][0] += d[pos + 1][num_groups][0];
        }
    }

    d[0][0][0]
}

fn parse_line(line: &str) -> (Vec<State>, Vec<usize>) {
    let mut parts = line.split_whitespace();
    let states = parts
        .next()
        .unwrap()
        .chars()
        .map(State::from)
        .collect::<Vec<State>>();
    let group_sizes = parts
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (states, group_sizes)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Empty,
    Filled,
    Superposition,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Filled,
            '?' => Self::Superposition,
            _ => unreachable!(),
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

        assert_eq!(solver.part_one(), 6958);
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
