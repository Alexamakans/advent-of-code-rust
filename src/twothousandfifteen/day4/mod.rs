use std::{
    thread, collections::VecDeque,
};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let num_threads = 10;
        let chunk_size = 5_000;
        let mut start_i = 1;

        let mut handles = VecDeque::new();
        loop {
            let input_clone = input.to_string();
            handles.push_back(thread::spawn(move || {
                for i in start_i..start_i+chunk_size {
                    let s = format!("{}{}", input_clone, i);
                    let hash = md5::calculate_hash_bytes(&s);
                    if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
                        return Some(i);
                    }
                }
                None
            }));
            start_i += chunk_size;
            if handles.len() >= num_threads {
                let handle = handles.pop_front().unwrap();
                match handle.join() {
                    Ok(opt_result) => match opt_result {
                        Some(result) => {
                            for handle in handles {
                                _ = handle.join(); // be nice and let the threads finish
                            }
                            return result;
                        },
                        None => (),
                    },
                    Err(e) => panic!("Join failed: {:?}", e),
                }
            }
        }
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let num_threads = 10;
        let chunk_size = 10_000;
        let mut start_i = 1;

        let mut handles = VecDeque::new();
        loop {
            let input_clone = input.to_string();
            handles.push_back(thread::spawn(move || {
                for i in start_i..start_i+chunk_size {
                    let s = format!("{}{}", input_clone, i);
                    let hash = md5::calculate_hash_bytes(&s);
                    if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                        return Some(i);
                    }
                }
                None
            }));
            start_i += chunk_size;
            if handles.len() >= num_threads {
                let handle = handles.pop_front().unwrap();
                match handle.join() {
                    Ok(opt_result) => match opt_result {
                        Some(result) => {
                            for handle in handles {
                                _ = handle.join(); // be nice and let the threads finish
                            }
                            return result;
                        },
                        None => (),
                    },
                    Err(e) => panic!("Join failed: {:?}", e),
                }
            }
        }
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
        let cases = vec![("abcdef", 609043), ("pqrstuv", 1048970)];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

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
