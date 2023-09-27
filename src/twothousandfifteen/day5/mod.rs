use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        fn is_nice_string(s: &str) -> bool {
            let does_not_have_blacklisted_substrings =
                !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"));
            let num_vowels = s
                .chars()
                .filter(|c| match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => true,
                    _ => false,
                })
                .count();
            let has_double = 'has_double_check: {
                for i in 1..s.chars().count() {
                    if s.chars().nth(i).unwrap() == s.chars().nth(i - 1).unwrap() {
                        break 'has_double_check true;
                    }
                }
                false
            };

            does_not_have_blacklisted_substrings && num_vowels >= 3 && has_double
        }

        input.lines().filter(|line| is_nice_string(line)).count() as i32
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
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            ("ugknbfddgicrmopn", 1),
            ("aaa", 1),
            ("jchzalrnumimnmhp", 0),
            ("haegwjzuvuyypxyu", 0),
            ("dvszwmarrgswjxmb", 0),
        ];

        for case in cases {
            assert_eq!(
                solver.part_one_driver(String::from(case.0)),
                case.1,
                "input = {}",
                case.0
            );
        }

        assert_eq!(solver.part_one(), 258);
    }

    #[test]
    fn part_two_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(String::from(case.0)), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_two(), 123);
    }
}
