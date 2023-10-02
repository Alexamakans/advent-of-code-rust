#[cfg(feature = "combinatorials")]
pub mod combinatorials;
#[cfg(feature = "conway")]
pub mod conway;
#[cfg(feature = "iterators")]
pub mod iterators;
#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "md5")]
pub mod md5;
#[cfg(feature = "parsing")]
pub mod parsing;
#[cfg(feature = "cuda-md5")]
pub mod cuda_md5;

#[cfg(feature = "combinatorials")]
pub use combinatorials::*;
#[cfg(feature = "conway")]
pub use conway::*;
#[cfg(feature = "iterators")]
pub use iterators::*;
#[cfg(feature = "math")]
pub use math::*;
#[cfg(feature = "md5")]
pub use md5::*;
#[cfg(feature = "parsing")]
pub use parsing::*;
#[cfg(feature = "cuda-md5")]
pub use cuda_md5::*;

#[cfg(feature = "challenges")]
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

#[cfg(feature = "challenges")]
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
