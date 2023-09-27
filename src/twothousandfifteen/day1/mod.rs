use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        let mut floor = 0;
        for ch in input.chars() {
            match ch {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => unreachable!(),
            };
        }
        floor
    }

    fn part_two_driver(&self, input: String) -> i32 {
        let mut floor = 0;
        for (i, ch) in input.chars().enumerate() {
            match ch {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => unreachable!(),
            };
            if floor == -1 {
                return i as i32 + 1;
            }
        }
        floor
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![
            ("(())", 0), ("()()", 0),
            ("(((", 3), ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1), ("))(", -1),
            (")))", -3), (")())())", -3)
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(String::from(case.0)), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 138);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        let cases = vec![
            (")", 1), ("()())", 5),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(String::from(case.0)), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 1771);
    }
}