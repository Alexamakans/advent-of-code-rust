use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        todo!();
    }

    fn part_two_driver(&self, input: String) -> i32 {
        todo!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 5)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_one_driver(String::from(case.0)), case.1, "input = {}", case.0);
        // }
    }

    #[test]
    fn part_2_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(String::from(case.0)), case.1, "input = {}", case.0);
        // }
    }
}