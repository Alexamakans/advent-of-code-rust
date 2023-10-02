use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut sum_length = 0;
        for line in input.lines() {
            let encoded = encode(&line, 40);
            sum_length += encoded.len();
        }

        sum_length as i32
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut sum_length = 0;
        for line in input.lines() {
            let encoded = encode(&line, 50);
            sum_length += encoded.len();
        }

        sum_length as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 10)
    }
}

fn encode(s: &str, iterations: u32) -> String {
    let mut result = s.to_owned();
    for _ in 0..iterations {
        let mut iteration_result = String::new();
        let sequences = extract_repeating_sequences(result.chars());
        for sequence in sequences.iter() {
            let length = sequence.0.1 - sequence.0.0;
            iteration_result.push_str(&format!("{}{}", length, sequence.1));
        }
        result = iteration_result;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![
            ("1", 1, "11"),
            ("11", 1, "21"),
            ("21", 1, "1211"),
            ("1211", 1, "111221"),
            ("111221", 1, "312211"),
        ];

        for case in cases {
            assert_eq!(encode(case.0, case.1), case.2, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 329356);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        let cases = vec![
            ("1", 1, "11"),
            ("11", 1, "21"),
            ("21", 1, "1211"),
            ("1211", 1, "111221"),
            ("111221", 1, "312211"),
        ];

        for case in cases {
            assert_eq!(encode(case.0, case.1), case.2, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 4666278);
    }
}