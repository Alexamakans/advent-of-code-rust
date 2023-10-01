use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u32> for Solver {
    fn part_one_driver(&self, input: &str) -> u32 {
        let mut descending_sorted_containers = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        descending_sorted_containers.sort_by(|a, b| b.cmp(a));
        get_combinations_to_reach_target_value(&descending_sorted_containers, 150).len() as u32
    }

    fn part_two_driver(&self, input: &str) -> u32 {
        let mut descending_sorted_containers = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        descending_sorted_containers.sort_by(|a, b| b.cmp(a));
        let combinations = get_combinations_to_reach_target_value(&descending_sorted_containers, 150);
        let minimum_possible = combinations.iter().min_by_key(|e| e.len()).unwrap().len();
        combinations.into_iter().filter(|e| e.len() == minimum_possible).count() as u32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 17)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(vec![20, 15, 10, 5, 5], 25, 4)];

        for case in cases {
            assert_eq!(
                get_combinations_to_reach_target_value(&case.0, case.1).len(),
                case.2,
                "containers = {:?}, liters = {}",
                case.0,
                case.1
            );
        }

        assert_eq!(solver.part_one(), 1304);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        assert_eq!(solver.part_two(), 18);
    }
}
