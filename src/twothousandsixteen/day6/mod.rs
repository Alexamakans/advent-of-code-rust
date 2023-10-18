use std::collections::HashMap;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<String> for Solver {
    fn part_one_driver(&self, input: &str) -> String {
        let word_length = input.lines().nth(0).unwrap().len();
        let mut counters: Vec<HashMap<char, u32>> = Vec::new();
        for _ in 0..word_length {
            counters.push(HashMap::new());
        }

        for line in input.lines() {
            for (index, c) in line.chars().enumerate() {
                let m = counters.get_mut(index).unwrap();
                if !m.contains_key(&c) {
                    m.insert(c, 0);
                }
                let count = m.get_mut(&c).unwrap();
                *count += 1;
            }
        }

        let mut result = String::new();
        for index in 0..word_length {
            let c = counters[index].iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
            result.push(*c.0);
        }
        result
    }

    fn part_two_driver(&self, input: &str) -> String {
        let word_length = input.lines().nth(0).unwrap().len();
        let mut counters: Vec<HashMap<char, u32>> = Vec::new();
        for _ in 0..word_length {
            counters.push(HashMap::new());
        }

        for line in input.lines() {
            for (index, c) in line.chars().enumerate() {
                let m = counters.get_mut(index).unwrap();
                if !m.contains_key(&c) {
                    m.insert(c, 0);
                }
                let count = m.get_mut(&c).unwrap();
                *count += 1;
            }
        }

        let mut result = String::new();
        for index in 0..word_length {
            let c = counters[index].iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
            result.push(*c.0);
        }
        result
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
            "easter",
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), "wkbvmikb");
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
            "advent",
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), "evakwaga");
    }
}
