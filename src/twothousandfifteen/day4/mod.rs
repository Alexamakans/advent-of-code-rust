use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
    thread,
};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        let num_threads = 10;
        let chunk_size = 10_000;
        let mut start_i = 1;
        let should_exit = Arc::new(RwLock::new(false));
        let input = input.bytes().collect::<Vec<u8>>();

        let mut handles = VecDeque::new();
        while handles.len() < num_threads {
            let input = input.clone();
            let should_exit = should_exit.clone();
            handles.push_back(thread::spawn(move || loop {
                for i in start_i..start_i + chunk_size {
                    if *should_exit.read().unwrap() {
                        return None;
                    }
                    let hash = md5::calculate_hash_bytes(
                        input
                            .clone()
                            .into_iter()
                            .chain(i.to_string().bytes().into_iter())
                            .collect::<Vec<u8>>(),
                    );
                    if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
                        *should_exit.write().unwrap() = true;
                        return Some(i);
                    }
                }
                start_i += chunk_size * num_threads;
            }));
            start_i += chunk_size;
        }

        while handles.len() > 0 {
            let handle = handles.pop_front().unwrap();
            match handle.join() {
                Ok(opt_result) => match opt_result {
                    Some(result) => {
                        for handle in handles {
                            _ = handle.join(); // be nice and let the threads finish
                        }
                        return result;
                    }
                    None => (),
                },
                Err(e) => panic!("Join failed: {:?}", e),
            }
        }
        unreachable!();
    }

    fn part_two_driver(&self, input: &str) -> usize {
        let num_threads = 10;
        let chunk_size = 100_000;
        let mut start_i = 1;
        let should_exit = Arc::new(RwLock::new(false));
        let input = input.bytes().collect::<Vec<u8>>();

        let mut handles = VecDeque::new();
        while handles.len() < num_threads {
            let input = input.clone();
            let should_exit = should_exit.clone();
            handles.push_back(thread::spawn(move || loop {
                for i in start_i..start_i + chunk_size {
                    if *should_exit.read().unwrap() {
                        return None;
                    }
                    let hash = md5::calculate_hash_bytes(
                        input
                            .clone()
                            .into_iter()
                            .chain(i.to_string().bytes().into_iter())
                            .collect::<Vec<u8>>(),
                    );
                    if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                        *should_exit.write().unwrap() = true;
                        return Some(i);
                    }
                }
                start_i += chunk_size * num_threads;
            }));
            start_i += chunk_size;
        }

        while handles.len() > 0 {
            let handle = handles.pop_front().unwrap();
            match handle.join() {
                Ok(opt_result) => match opt_result {
                    Some(result) => {
                        for handle in handles {
                            _ = handle.join(); // be nice and let the threads finish
                        }
                        return result;
                    }
                    None => (),
                },
                Err(e) => panic!("Join failed: {:?}", e),
            }
        }
        unreachable!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        // let cases = vec![("abcdef", 609043), ("pqrstuv", 1048970)];

        // for case in cases {
        // assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_one(), 282749);
    }

    #[test]
    fn part_two_works() {
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        let solver = Solver {};
        assert_eq!(solver.part_two(), 9962624);
    }
}
