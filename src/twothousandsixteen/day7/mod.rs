use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut result = 0;
        for line in input.lines() {
            if supports_tls(line) {
                result += 1;
            }
        }
        result
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut result = 0;
        for line in input.lines() {
            if supports_ssl(line) {
                result += 1;
            }
        }
        result
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 7)
    }
}

fn supports_tls(line: &str) -> bool {
    let mut bracket_level = 0;
    let mut has_good_abba = false;
    let mut has_bad_abba = false;
    for i in 1..line.chars().count() - 2 {
        let c = line.chars().nth(i).unwrap();
        match c {
            '[' => bracket_level += 1,
            ']' => bracket_level -= 1,
            _ => {
                let mut ch = line.chars();
                let a = format!("{}{}", ch.nth(i - 1).unwrap(), ch.nth(0).unwrap());
                let b = format!("{}{}", ch.nth(0).unwrap(), ch.nth(0).unwrap());
                let b_rev = b.chars().rev().collect::<String>();
                if a == b_rev && a != b {
                    if bracket_level == 0 {
                        has_good_abba = true;
                    } else {
                        has_bad_abba = true;
                        break;
                    }
                }
            }
        }
    }

    !has_bad_abba && has_good_abba
}

fn supports_ssl(line: &str) -> bool {
    unreachable!();
    // let mut bracket_level = 0;
    // let mut has_good_abba = false;
    // let mut has_bad_abba = false;
    // for i in 1..line.chars().count() - 2 {
    //     let c = line.chars().nth(i).unwrap();
    //     match c {
    //         '[' => bracket_level += 1,
    //         ']' => bracket_level -= 1,
    //         _ => {
    //             let mut ch = line.chars();
    //             let a = format!("{}{}", ch.nth(i - 1).unwrap(), ch.nth(0).unwrap());
    //             let b = format!("{}{}", ch.nth(0).unwrap(), ch.nth(0).unwrap());
    //             let b_rev = b.chars().rev().collect::<String>();
    //             if a == b_rev && a != b {
    //                 if bracket_level == 0 {
    //                     has_good_abba = true;
    //                 } else {
    //                     has_bad_abba = true;
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    // !has_bad_abba && has_good_abba
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            ("abba[mnop]qrst", 1),
            ("abcd[bddb]xyyx", 0),
            ("aaaa[qwer]tyui", 0),
            ("ioxxoj[asdfgh]zxcvbn", 1),
            ("ioxxoj[as[abba]dfgh]zxcvbn", 0),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 115);
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
