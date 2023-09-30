#![feature(impl_trait_in_assoc_type)]

use std::time::{Duration, Instant};

use clap::Parser;

mod twothousandfifteen;
mod twothousandsixteen;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: u16,
    #[arg(short, long)]
    day: usize,
    #[arg(short, long)]
    part: u8,
}

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
            .evaluate_input($part, $input)
    };
}

fn main() {
    let start = Instant::now();
    let args = Args::parse();
    assert!(args.day > 0 && args.day < 26);
    assert!(args.part == 1 || args.part == 2);

    let result = match get_stdin_if_available() {
        Some(input) => {
            println!("Taking input from stdin");
            match args.year {
                2015 => add_module_evaluate_input!(twothousandfifteen, args.day, args.part, &input),
                2016 => add_module_evaluate_input!(twothousandsixteen, args.day, args.part, &input),
                _ => panic!("no module for year {}", args.year),
            }
        }
        None => match args.year {
            2015 => add_module_evaluate!(twothousandfifteen, args.day, args.part),
            2016 => add_module_evaluate!(twothousandsixteen, args.day, args.part),
            _ => panic!("no module for year {}", args.year),
        },
    };

    println!(
        "Result is {} for Year {} Day {} Part {}, took {:.2?}",
        result,
        args.year,
        args.day,
        args.part,
        start.elapsed(),
    );
}

fn get_stdin_if_available() -> Option<String> {
    use std::sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    };
    use std::thread;

    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    let stdin_thread = thread::spawn(move || {
        let piped_input = std::io::stdin()
            .lines()
            .map(|e| e.unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        sender.send(piped_input).expect("expected success");
    });

    let millisecond = 1000000;
    match receiver.recv_timeout(Duration::new(0, 200 * millisecond)) {
        Ok(piped_input) => {
            stdin_thread.join().expect("expected thread to behave");
            Some(piped_input)
        }
        Err(_) => None,
    }
}
