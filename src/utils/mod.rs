use std::str::FromStr;

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
    std::fs::read_to_string(path).expect("it exists")
}

pub fn read_input_as_lines_vec(year: u16, day: u8) -> Vec<String> {
    read_input(year, day)
        .lines()
        .map(ToString::to_string)
        .collect()
}

pub fn read_input_as_lines_parsed<T>(year: u16, day: u8) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: core::fmt::Debug,
{
    read_input_as_lines_vec(year, day)
        .iter()
        .map(|e| e.parse::<T>().expect("parsed"))
        .collect::<Vec<T>>()
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
