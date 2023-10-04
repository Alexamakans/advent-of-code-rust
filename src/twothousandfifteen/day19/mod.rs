use std::collections::{HashSet, VecDeque};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        let mappings = input
            .lines()
            .take_while(|line| !line.eq(&""))
            .map(|line| Mapping::from(line));
        let molecule = input
            .lines()
            .skip_while(|line| !line.eq(&""))
            .skip(1)
            .next()
            .unwrap();
        let mut distinct_molecules = HashSet::new();
        for mapping in mappings {
            let indices = molecule.match_indices(mapping.from);
            for (index, m) in indices {
                distinct_molecules.insert(
                    molecule
                        .chars()
                        .take(index)
                        .chain(mapping.to.chars())
                        .chain(molecule.chars().skip(index + m.len()))
                        .collect::<String>(),
                );
            }
        }

        distinct_molecules.len()
    }

    fn part_two_driver(&self, input: &str) -> usize {
        let mappings = input
            .lines()
            .take_while(|line| !line.eq(&""))
            .map(|line| Mapping::from(line));
        let target_molecule = input
            .lines()
            .skip_while(|line| !line.eq(&""))
            .skip(1)
            .next()
            .unwrap();

        solution_from_molecule_to_e(mappings, target_molecule)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 19)
    }
}

fn solution_from_molecule_to_e<'a, T>(mappings: T, target_molecule: &str) -> usize
where
    T: Iterator<Item = Mapping<'a>> + Clone,
{
    // Wish I could have done this the proper way, maybe later.
    // https://en.wikipedia.org/wiki/Context-free_grammar

    let mut result = target_molecule.to_string();
    let mut steps = 0;
    loop {
        for mapping in mappings.clone() {
            let indices = result.match_indices(mapping.to);
            for (index, m) in indices {
                result = result
                    .chars()
                    .take(index)
                    .chain(mapping.from.chars())
                    .chain(result.chars().skip(index + m.len()))
                    .collect::<String>();
                steps += 1;
                if result == "e" {
                    return steps;
                }
                break;
            }
        }
    }
}

struct Mapping<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> From<&'a str> for Mapping<'a> {
    fn from(value: &'a str) -> Self {
        let (left, right) = value.split_once(" => ").unwrap();
        Mapping {
            from: left,
            to: right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            (
                r"H => HO
H => OH
O => HH

HOH",
                4,
            ),
            (
                r"H => HO
H => OH
O => HH

HOHOHO",
                7,
            ),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 509);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![
            (
                r"e => H
e => O
H => HO
H => OH
O => HH

HOH",
                3,
            ),
            (
                r"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO",
                6,
            ),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 195);
    }
}
