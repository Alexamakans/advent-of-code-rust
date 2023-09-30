#![feature(impl_trait_in_assoc_type)]

mod utils;
mod twothousandfifteen;
mod twothousandsixteen;

const USAGE: &'static str = "<bin-name> <year> <day> <part>";

fn main() {
    let mut args = std::env::args();
    _ = args.next();
    let year = args.next().expect(USAGE).parse::<u16>().expect(USAGE);
    let day = args.next().expect(USAGE).parse::<usize>().expect(USAGE);
    assert!(day > 0 && day < 26);
    let part = args.next().expect(USAGE).parse::<u8>().expect(USAGE);
    assert!(part == 1 || part == 2);

    let result = match year {
        2015 => twothousandfifteen::get_days().get((day as i32-1) as usize).unwrap().evaluate(part),
        2016 => twothousandsixteen::get_days().get((day as i32-1) as usize).unwrap().evaluate(part),
        _ => panic!("no module for year {}", year),
    };

    println!("Result is {} for Year {} Day {} Part {}", result, year, day, part);
}
