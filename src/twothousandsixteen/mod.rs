const YEAR: u16 = 2016;

use std::fmt::Display;

pub mod day1;
pub use day1::Solver as DayOne;
pub mod day2;
pub use day2::Solver as DayTwo;
pub mod day3;
pub use day3::Solver as DayThree;
pub mod day4;
pub use day4::Solver as DayFour;
pub mod day5;
pub use day5::Solver as DayFive;
pub mod day6;
pub use day6::Solver as DaySix;
pub mod day7;
pub use day7::Solver as DaySeven;
pub mod day8;
pub use day8::Solver as DayEight;
pub mod day9;
pub use day9::Solver as DayNine;
pub mod day10;
pub use day10::Solver as DayTen;
pub mod day11;
pub use day11::Solver as DayEleven;
pub mod day12;
pub use day12::Solver as DayTwelve;
pub mod day13;
pub use day13::Solver as DayThirteen;
pub mod day14;
pub use day14::Solver as DayFourteen;
pub mod day15;
pub use day15::Solver as DayFifteen;
pub mod day16;
pub use day16::Solver as DaySixteen;
pub mod day17;
pub use day17::Solver as DaySeventeen;
pub mod day18;
pub use day18::Solver as DayEighteen;
pub mod day19;
pub use day19::Solver as DayNineteen;
pub mod day20;
pub use day20::Solver as DayTwenty;
pub mod day21;
pub use day21::Solver as DayTwentyOne;
pub mod day22;
pub use day22::Solver as DayTwentyTwo;
pub mod day23;
pub use day23::Solver as DayTwentyThree;
pub mod day24;
pub use day24::Solver as DayTwentyFour;
pub mod day25;
pub use day25::Solver as DayTwentyFive;

use crate::utils::DaySolver;

pub enum DaySolverEnum {
    One(DayOne),
    Two(DayTwo),
    Three(DayThree),
    Four(DayFour),
    Five(DayFive),
    Six(DaySix),
    Seven(DaySeven),
    Eight(DayEight),
    Nine(DayNine),
    Ten(DayTen),
    Eleven(DayEleven),
    Twelve(DayTwelve),
    Thirteen(DayThirteen),
    Fourteen(DayFourteen),
    Fifteen(DayFifteen),
    Sixteen(DaySixteen),
    Seventeen(DaySeventeen),
    Eighteen(DayEighteen),
    Nineteen(DayNineteen),
    Twenty(DayTwenty),
    TwentyOne(DayTwentyOne),
    TwentyTwo(DayTwentyTwo),
    TwentyThree(DayTwentyThree),
    TwentyFour(DayTwentyFour),
    TwentyFive(DayTwentyFive),
}

impl DaySolverEnum {
    pub fn evaluate(&self, part: u8) -> Box<dyn Display> {
        match self {
            DaySolverEnum::One(v) => Box::new(Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            })),
            DaySolverEnum::Two(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Three(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Four(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Five(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Six(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Seven(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Eight(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Nine(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Ten(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Eleven(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Twelve(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Thirteen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Fourteen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Fifteen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Sixteen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Seventeen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Eighteen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Nineteen(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::Twenty(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::TwentyOne(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::TwentyTwo(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::TwentyThree(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::TwentyFour(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
            DaySolverEnum::TwentyFive(v) => Box::new(if part == 1 {
                v.part_one()
            } else {
                v.part_two()
            }),
        }
    }

    pub fn evaluate_input(&self, part: u8, input: &str) -> Box<dyn Display> {
        match self {
            DaySolverEnum::One(v) => Box::new(Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            })),
            DaySolverEnum::Two(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Three(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Four(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Five(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Six(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Seven(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Eight(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Nine(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Ten(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Eleven(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Twelve(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Thirteen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Fourteen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Fifteen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Sixteen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Seventeen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Eighteen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Nineteen(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::Twenty(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::TwentyOne(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::TwentyTwo(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::TwentyThree(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::TwentyFour(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
            DaySolverEnum::TwentyFive(v) => Box::new(if part == 1 {
                v.part_one_driver(input)
            } else {
                v.part_two_driver(input)
            }),
        }
    }
}

pub fn get_days() -> Vec<DaySolverEnum> {
    vec![
        DaySolverEnum::One(DayOne {}),
        DaySolverEnum::Two(DayTwo {}),
        DaySolverEnum::Three(DayThree {}),
        DaySolverEnum::Four(DayFour {}),
        DaySolverEnum::Five(DayFive {}),
        DaySolverEnum::Six(DaySix {}),
        DaySolverEnum::Seven(DaySeven {}),
        DaySolverEnum::Eight(DayEight {}),
        DaySolverEnum::Nine(DayNine {}),
        DaySolverEnum::Ten(DayTen {}),
        DaySolverEnum::Eleven(DayEleven {}),
        DaySolverEnum::Twelve(DayTwelve {}),
        DaySolverEnum::Thirteen(DayThirteen {}),
        DaySolverEnum::Fourteen(DayFourteen {}),
        DaySolverEnum::Fifteen(DayFifteen {}),
        DaySolverEnum::Sixteen(DaySixteen {}),
        DaySolverEnum::Seventeen(DaySeventeen {}),
        DaySolverEnum::Eighteen(DayEighteen {}),
        DaySolverEnum::Nineteen(DayNineteen {}),
        DaySolverEnum::Twenty(DayTwenty {}),
        DaySolverEnum::TwentyOne(DayTwentyOne {}),
        DaySolverEnum::TwentyTwo(DayTwentyTwo {}),
        DaySolverEnum::TwentyThree(DayTwentyThree {}),
        DaySolverEnum::TwentyFour(DayTwentyFour {}),
        DaySolverEnum::TwentyFive(DayTwentyFive {}),
    ]
}
