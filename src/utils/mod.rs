pub mod combinatorials;
pub mod conway;
pub mod crypto;
pub mod iterators;
pub mod math;
pub mod matrix;
pub mod parsing;

pub use combinatorials::*;
pub use conway::*;
pub use crypto::*;
pub use iterators::*;
pub use math::*;
pub use matrix::*;
pub use parsing::*;

pub fn read_input(year: u16, day: u8) -> String {
    if day == 0 || day > 25 {
        panic!("Bad day '{}'", day);
    }

    let path = format!(
        "{}/inputs/{}-{}.txt",
        std::env::current_dir().unwrap().to_str().unwrap(),
        year,
        day,
    );
    std::fs::read_to_string(&path)
        .expect(&format!(
            "Expected file '{}' to exist, but it doesn't",
            path
        ))
        .trim()
        .to_string()
}

pub trait DaySolver<T> {
    fn read_input(&self) -> String;
    fn part_one(&self) -> T {
        self.part_one_driver(&self.read_input())
    }
    fn part_one_driver(&self, input: &str) -> T;

    fn part_two(&self) -> T {
        self.part_two_driver(&self.read_input())
    }
    fn part_two_driver(&self, input: &str) -> T;
}
