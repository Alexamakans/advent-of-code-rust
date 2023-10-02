use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut sum_literal_len = 0;
        let mut sum_data_len = 0;
        for line in input.lines() {
            let literal_len = line.len();
            let mut data_len = 0;
            let mut index = 0;
            while index < literal_len {
                let c = line.chars().nth(index).unwrap();
                match c {
                    '\\' => {
                        match line.chars().nth(index+1) {
                            Some(next_ch) => {
                                match next_ch {
                                    'x' => {
                                        data_len += 1;
                                        index += 4;
                                    },
                                    '"' => {
                                        data_len += 1;
                                        index += 2;
                                    },
                                    '\\' => {
                                        data_len += 1;
                                        index += 2;
                                    },
                                    _ => unreachable!(),
                                }
                            },
                            None => unreachable!(),
                        }
                    },
                    '"' => index += 1,
                    _ => {
                        data_len += 1;
                        index += 1;
                    }
                }
            }

            println!("line = '{}'", line);
            println!("literal_len = {}", literal_len);
            println!("data_len = {}", data_len);
            sum_literal_len += literal_len; // remove the outer double quotes, they don't count
            sum_data_len += data_len;
        }

        sum_literal_len as i32 - sum_data_len as i32
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut sum_raw = 0;
        let mut sum_encoded = 0;
        for line in input.lines() {
            sum_raw += line.len() as i32;
            sum_encoded += encode(line).len() as i32;
        }

        sum_encoded - sum_raw
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 8)
    }
}

fn encode(s: &str) -> String {
    let mut result = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => {
                result.push('\\');
                result.push('"');
            },
            '\\' => {
                result.push('\\');
                result.push('\\');
            },
            _ => result.push(c),
        }
    }
    result.push('"');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![
            ("\"\"", 2-0),
            ("\"abc\"", 5-3),
            ("\"aaa\\\"aaa\"", 10-7),
            ("\"\\x27\"", 6-1),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 1333);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        let cases = vec![
            ("\"\"", 6-2),
            ("\"abc\"", 9-5),
            ("\"aaa\\\"aaa\"", 16-10),
            ("\"\\x27\"", 11-6),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 2046);
    }

    #[test]
    fn encode_works() {
        let cases = vec![
            ("\"\"", "\"\\\"\\\"\""),
            ("\"abc\"", "\"\\\"abc\\\"\""),
            ("\"aaa\\\"aaa\"", "\"\\\"aaa\\\\\\\"aaa\\\"\""),
            ("\"\\x27\"", "\"\\\"\\\\x27\\\"\""),
        ];

        for case in cases {
            assert_eq!(encode(case.0), case.1, "input = {}", case.0);
        }
    }
}