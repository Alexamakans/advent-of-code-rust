use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u32> for Solver {
    fn part_one_driver(&self, input: &str) -> u32 {
        let mut sum = 0;
        for line in input.lines() {
            sum += get_first_number(line) * 10 + get_last_number(line);
        }
        return sum;
    }

    fn part_two_driver(&self, input: &str) -> u32 {
        let mut sum = 0;
        for line in input.lines() {
            sum += get_first_number_part_two(line) * 10 + get_last_number_part_two(line);
        }
        return sum;
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 1)
    }
}

fn get_first_number(s: &str) -> u32 {
    match s.chars().find_map(|c| c.to_digit(10)) {
        Some(x) => x,
        None => panic!("didn't expect no numbers in line"),
    }
}

fn get_last_number(s: &str) -> u32 {
    match s.chars().rev().find_map(|c| c.to_digit(10)) {
        Some(x) => x,
        None => panic!("didn't expect no numbers in line"),
    }
}

fn get_first_number_part_two(s: &str) -> u32 {
    let digit_regex = regex::Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine|zero)")
        .expect("regex must compile");
    let res = digit_regex.find(s).expect("at least one match expected");
    match res.as_str() {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => panic!("expected exhaustive matching"),
    }
}

fn get_last_number_part_two(s: &str) -> u32 {
    let digit_regex = regex::Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez)")
        .expect("regex must compile");
    let rev_string = s.chars().rev().collect::<String>();
    let res = digit_regex
        .find(&rev_string)
        .expect("at least one match expected");
    match res.as_str() {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        "orez" => 0,
        _ => panic!("expected exhaustive matching"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 54916);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![
            ("two1nine", 29),
            ("eighttwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("cmpptgjc3qhcjxcbcqgqkxhrms", 33),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 54728);
    }
}
