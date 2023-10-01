use regex::Regex;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        get_matching_sue_index_from_lines(
            input,
            r"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1",
        )
    }

    fn part_two_driver(&self, input: &str) -> usize {
        get_matching_sue_index_from_lines_two(
            input,
            r"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1",
        )
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 16)
    }
}

macro_rules! compare_sue_field_continue_if_false {
    ($field_name:ident, $sue:expr, $my_sue:expr, $comparator:tt) => {
        if let Some($field_name) = $sue.$field_name {
            if !($field_name $comparator $my_sue.$field_name.unwrap()) {
                continue;
            }
        }
    };
}

fn get_matching_sue_index_from_lines(input: &str, my_sue_string: &str) -> usize {
    let sues = input
        .lines()
        .map(|line| Sue::from(line))
        .collect::<Vec<Sue>>();
    let my_sue = Sue::from(my_sue_string);
    for (sue_index, sue) in sues.iter().enumerate() {
        compare_sue_field_continue_if_false!(children, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(cats, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(samoyeds, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(pomeranians, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(akitas, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(vizslas, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(goldfish, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(trees, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(cars, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(perfumes, sue, my_sue, ==);
        return sue_index + 1;
    }
    unreachable!();
}

fn get_matching_sue_index_from_lines_two(input: &str, my_sue_string: &str) -> usize {
    let sues = input
        .lines()
        .map(|line| Sue::from(line))
        .collect::<Vec<Sue>>();
    let my_sue = Sue::from(my_sue_string);
    for (sue_index, sue) in sues.iter().enumerate() {
        compare_sue_field_continue_if_false!(children, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(cats, sue, my_sue, >);
        compare_sue_field_continue_if_false!(samoyeds, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(pomeranians, sue, my_sue, <);
        compare_sue_field_continue_if_false!(akitas, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(vizslas, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(goldfish, sue, my_sue, <);
        compare_sue_field_continue_if_false!(trees, sue, my_sue, >);
        compare_sue_field_continue_if_false!(cars, sue, my_sue, ==);
        compare_sue_field_continue_if_false!(perfumes, sue, my_sue, ==);
        return sue_index + 1;
    }
    unreachable!();
}

struct Sue {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

fn get_field_value_from_str(s: &str, field_name: &str) -> Option<u8> {
    let regex = Regex::new(&format!(r"{}: (\d+)", field_name)).unwrap();
    match regex.captures(s) {
        Some(m) => Some(m.extract::<1>().1[0].parse::<u8>().unwrap()),
        None => None,
    }
}

impl From<&str> for Sue {
    fn from(value: &str) -> Self {
        Sue {
            children: get_field_value_from_str(value, "children"),
            cats: get_field_value_from_str(value, "cats"),
            samoyeds: get_field_value_from_str(value, "samoyeds"),
            pomeranians: get_field_value_from_str(value, "pomeranians"),
            akitas: get_field_value_from_str(value, "akitas"),
            vizslas: get_field_value_from_str(value, "vizslas"),
            goldfish: get_field_value_from_str(value, "goldfish"),
            trees: get_field_value_from_str(value, "trees"),
            cars: get_field_value_from_str(value, "cars"),
            perfumes: get_field_value_from_str(value, "perfumes"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        assert_eq!(solver.part_one(), 103);
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
