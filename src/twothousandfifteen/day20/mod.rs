use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u32> for Solver {
    fn part_one_driver(&self, input: &str) -> u32 {
        let target_presents = input.parse::<u32>().unwrap();
        let mut house_number = 500000;
        loop {
            let presents = sum_of_factors(house_number) * 10;
            if presents >= target_presents {
                return house_number;
            }
            house_number += 1;

            if house_number % 16384 == 0 {
                println!("{}", house_number);
            }
        }
    }

    fn part_two_driver(&self, input: &str) -> u32 {
        todo!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![("100", 6)];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 665280);
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
