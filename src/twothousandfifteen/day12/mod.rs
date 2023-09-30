use regex::Regex;
use serde_json::Value;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: String) -> i32 {
        extract_numbers(&input).into_iter().reduce(|acc, v| acc + v).unwrap()
    }

    fn part_two_driver(&self, input: String) -> i32 {
        let root: Value = serde_json::from_str(&input).unwrap();
        get_sum_of_value(&root) as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 12)
    }
}

fn extract_numbers(s: &str) -> Vec<i32> {
    let number_regex = Regex::new(r"(-?\d+)").unwrap();
    number_regex
        .captures_iter(s)
        .map(|e| e.extract::<1>().1[0].parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn get_sum_of_value(value: &Value) -> i64 {
    let mut sum = 0;
    if let Some(obj) = value.as_object() {
        for v in obj.values() {
            if let Some(s) = v.as_str() {
                if s == "red" {
                    return 0;
                }
            } else {
                sum += get_sum_of_value(v);
            }
        }
    } else if let Some(arr) = value.as_array() {
        for v in arr {
            sum += get_sum_of_value(v);
        }
    } else if let Some(number) = value.as_number() {
        sum += number.as_i64().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            ("[1,2,3]", 6),
            (r#"{"a":2,"b":4}"#, 6),
            ("[[[3]]]", 3),
            (r#"{"a":{"b":4},"c":-1}"#, 3),
        ];

        for case in cases {
            assert_eq!(
                solver.part_one_driver(String::from(case.0)),
                case.1,
                "input = {}",
                case.0
            );
        }

        assert_eq!(solver.part_one(), 156366);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![
            ("[1,2,3]", 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ];

        for case in cases {
            assert_eq!(
                solver.part_two_driver(String::from(case.0)),
                case.1,
                "input = {}",
                case.0
            );
        }

        assert_eq!(solver.part_two(), 96852);
    }
}
