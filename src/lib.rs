#![feature(impl_trait_in_assoc_type)]

use std::fmt::Display;

pub mod twothousandfifteen;
pub mod twothousandsixteen;
pub mod twothousandtwentythree;
pub mod utils;

macro_rules! add_module_evaluate {
    ($module_name:ident, $day:expr, $part:expr) => {
        $module_name::get_days()
            .get(($day - 1) as usize)
            .unwrap()
            .evaluate($part)
    };
}

macro_rules! add_module_evaluate_input {
    ($module_name:ident, $day:expr, $part:expr, $input:expr) => {
        $module_name::get_days()
            .get(($day - 1) as usize)
            .unwrap()
            .evaluate_input($part, $input.trim())
    };
}

pub fn run_challenge(year: u16, day: usize, part: u8) -> Box<dyn Display> {
    match year {
        2015 => add_module_evaluate!(twothousandfifteen, day, part),
        2016 => add_module_evaluate!(twothousandsixteen, day, part),
        2023 => add_module_evaluate!(twothousandtwentythree, day, part),
        _ => panic!("no module for year {}", year),
    }
}

pub fn run_challenge_with_input(year: u16, day: usize, part: u8, input: &str) -> Box<dyn Display> {
    match year {
        2015 => add_module_evaluate_input!(twothousandfifteen, day, part, &input),
        2016 => add_module_evaluate_input!(twothousandsixteen, day, part, &input),
        2023 => add_module_evaluate_input!(twothousandtwentythree, day, part, &input),
        _ => panic!("no module for year {}", year),
    }
}
