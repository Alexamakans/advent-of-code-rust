use clap::Parser;
use std::time::{Duration, Instant};

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

fn main() {
    let start = Instant::now();
    let args = Args::parse();
    assert!(args.day > 0 && args.day < 26);
    assert!(args.part == 1 || args.part == 2);

    let result = match get_stdin_if_available() {
        Some(input) => aoclib::run_challenge_with_input(args.year, args.day, args.part, &input),
        None => aoclib::run_challenge(args.year, args.day, args.part),
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
