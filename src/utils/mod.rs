pub mod math;
pub mod md5;
pub mod parsing;

pub use math::*;
pub use md5::*;
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
    std::fs::read_to_string(&path).expect(&format!(
        "Expected file '{}' to exist, but it doesn't",
        path
    ))
}

pub trait DaySolver<T> {
    fn read_input(&self) -> String;
    fn part_one(&self) -> T {
        self.part_one_driver(self.read_input())
    }
    fn part_one_driver(&self, input: String) -> T;

    fn part_two(&self) -> T {
        self.part_two_driver(self.read_input())
    }
    fn part_two_driver(&self, input: String) -> T;
}
