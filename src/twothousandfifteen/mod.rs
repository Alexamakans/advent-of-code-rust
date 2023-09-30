const YEAR: u16 = 2015;

mod day1;
use std::fmt::Display;

use day1::Solver as DayOne;
mod day2;
use day2::Solver as DayTwo;
mod day3;
use day3::Solver as DayThree;
mod day4;
use day4::Solver as DayFour;
mod day5;
use day5::Solver as DayFive;
mod day6;
use day6::Solver as DaySix;
mod day7;
use day7::Solver as DaySeven;
mod day8;
use day8::Solver as DayEight;
mod day9;
use day9::Solver as DayNine;
mod day10;
use day10::Solver as DayTen;
mod day11;
use day11::Solver as DayEleven;
mod day12;
use day12::Solver as DayTwelve;
mod day13;
use day13::Solver as DayThirteen;
mod day14;
use day14::Solver as DayFourteen;
mod day15;
use day15::Solver as DayFifteen;
mod day16;
use day16::Solver as DaySixteen;
mod day17;
use day17::Solver as DaySeventeen;
mod day18;
use day18::Solver as DayEighteen;
mod day19;
use day19::Solver as DayNineteen;
mod day20;
use day20::Solver as DayTwenty;
mod day21;
use day21::Solver as DayTwentyOne;
mod day22;
use day22::Solver as DayTwentyTwo;
mod day23;
use day23::Solver as DayTwentyThree;
mod day24;
use day24::Solver as DayTwentyFour;
mod day25;
use day25::Solver as DayTwentyFive;

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
            DaySolverEnum::One(v) => Box::new(Box::new(if part == 1 { v.part_one() } else { v.part_two() })),
            DaySolverEnum::Two(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Three(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Four(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Five(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Six(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Seven(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Eight(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Nine(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Ten(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Eleven(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Twelve(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Thirteen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Fourteen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Fifteen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Sixteen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Seventeen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Eighteen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Nineteen(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::Twenty(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::TwentyOne(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::TwentyTwo(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::TwentyThree(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::TwentyFour(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
            DaySolverEnum::TwentyFive(v) => Box::new(if part == 1 { v.part_one() } else { v.part_two() }),
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
