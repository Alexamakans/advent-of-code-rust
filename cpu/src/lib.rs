#![cfg_attr(feature = "iterators", feature(impl_trait_in_assoc_type))]

pub mod utils;

#[cfg(feature = "challenges")]
use std::fmt::Display;
#[cfg(feature = "challenges")]
pub mod twothousandfifteen;
#[cfg(feature = "challenges")]
pub mod twothousandsixteen;

#[cfg(feature = "challenges")]
macro_rules! add_module_evaluate {
    ($module_name:ident, $day:expr, $part:expr) => {
        $module_name::get_days()
            .get(($day - 1) as usize)
            .unwrap()
            .evaluate($part)
    };
}

#[cfg(feature = "challenges")]
macro_rules! add_module_evaluate_input {
    ($module_name:ident, $day:expr, $part:expr, $input:expr) => {
        $module_name::get_days()
            .get(($day - 1) as usize)
            .unwrap()
            .evaluate_input($part, $input)
    };
}

#[cfg(feature = "challenges")]
pub fn run_challenge(year: u16, day: usize, part: u8) -> Box<dyn Display> {
    match year {
        2015 => add_module_evaluate!(twothousandfifteen, day, part),
        2016 => add_module_evaluate!(twothousandsixteen, day, part),
        _ => panic!("no module for year {}", year),
    }
}

#[cfg(feature = "challenges")]
pub fn run_challenge_with_input(year: u16, day: usize, part: u8, input: &str) -> Box<dyn Display> {
    match year {
        2015 => add_module_evaluate_input!(twothousandfifteen, day, part, &input),
        2016 => add_module_evaluate_input!(twothousandsixteen, day, part, &input),
        _ => panic!("no module for year {}", year),
    }
}
