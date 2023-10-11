use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
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

    fn part_two_driver(&self, input: &str) -> i32 {
        fn is_nice_string(s: &str) -> bool {
            let mut pairs = Vec::new();
            for index in 1..s.chars().count() {
                let a = s.chars().nth(index - 1).unwrap();
                let b = s.chars().nth(index).unwrap();
                // Store indices so we can check that they don't overlap
                pairs.push((index - 1, index, (a, b)));
            }

            let has_non_overlapping_pairs = 'found: {
                for i in 0..pairs.len() {
                    for j in (i + 1)..pairs.len() {
                        if pairs[i].2 == pairs[j].2 && pairs[i].1 != pairs[j].0 {
                            break 'found true;
                        }
                    }
                }
                false
            };

            let has_sandwiched_letter = 'found: {
                for i in 1..s.chars().count() - 1 {
                    let prev = s.chars().nth(i - 1).unwrap();
                    let me = s.chars().nth(i).unwrap();
                    let next = s.chars().nth(i + 1).unwrap();
                    if me != prev && prev == next {
                        break 'found true;
                    }
                }
                false
            };

            has_non_overlapping_pairs && has_sandwiched_letter
        }

        input.lines().filter(|line| is_nice_string(line)).count() as i32
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
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 258);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![
            ("qjhvhtzxzqqjkmpb", 1),
            ("xxyxx", 1),
            ("uurcxstgmygtbstg", 0),
            ("ieodomkazucvgmuy", 0),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 53);
    }
}
