use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, input: &str) -> u64 {
        let packages = input
            .lines()
            .map(|e| e.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();

        get_best_quantum_entanglement(packages, 3)
    }

    fn part_two_driver(&self, input: &str) -> u64 {
        let packages = input
            .lines()
            .map(|e| e.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();

        get_best_quantum_entanglement(packages, 4)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 24)
    }
}

pub fn get_best_quantum_entanglement(mut packages: Vec<u16>, num_groups: u16) -> u64 {
    let value_per_group = packages.iter().fold(0, |acc, cur| acc + *cur) / num_groups;
    packages.sort_by(|a, b| b.cmp(&a));
    let combinations = get_combinations_to_reach_target_value(&packages, value_per_group);
    let mut best_quantum_entanglement = u64::MAX;
    let mut best_length = usize::MAX;
    for combination in combinations {
        let combination_length = combination.len();
        if combination_length <= best_length {
            let quantum_entanglement = combination
                .into_iter()
                .fold(1, |acc, cur| acc as u64 * cur as u64);
            if quantum_entanglement < best_quantum_entanglement {
                best_length = combination_length;
                best_quantum_entanglement = quantum_entanglement;
            }
        }
    }

    best_quantum_entanglement
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

        assert_eq!(solver.part_one(), 10723906903);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 74850409);
    }
}
