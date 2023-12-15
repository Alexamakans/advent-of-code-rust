use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, input: &str) -> u64 {
        input
            .trim()
            .split(',')
            .fold(0, |acc, s| acc + encode_str(s))
    }

    fn part_two_driver(&self, input: &str) -> u64 {
        #[derive(Clone)]
        struct Entry<'a> {
            label: &'a str,
            focal: usize,
        }
        let mut hm: Vec<Vec<Entry>> = vec![vec![]; 256];
        for op in input.trim().split(',') {
            let (label, index, focal) = decode_hash_op(op);

            let current_index = {
                let e = hm
                    .get(index)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .find(|(_, e)| e.label == label);
                if let Some(e) = e {
                    Some(e.0)
                } else {
                    None
                }
            };

            let exists = current_index.is_some();

            if let Some(focal) = focal {
                if exists {
                    *hm.get_mut(index)
                        .unwrap()
                        .get_mut(current_index.unwrap())
                        .unwrap() = Entry { label, focal };
                } else {
                    hm.get_mut(index).unwrap().push(Entry { label, focal });
                }
            } else {
                if exists {
                    hm.get_mut(index).unwrap().remove(current_index.unwrap());
                }
            }
        }

        hm.into_iter()
            .enumerate()
            .map(move |(box_index, entries)| {
                entries.into_iter().enumerate().map(move |(slot, entry)| {
                    println!(
                        "{} * {} * {} = {}  ;  {}",
                        box_index + 1,
                        slot + 1,
                        entry.focal,
                        (box_index + 1) * (slot + 1) * entry.focal,
                        entry.label
                    );
                    (box_index + 1) * (slot + 1) * entry.focal
                })
            })
            .flatten()
            .fold(0, |acc, cur| acc + cur as u64)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 15)
    }
}

fn encode_char(value: &mut u64, c: char) {
    *value += c as u64;
    *value *= 17;
    *value %= 256;
}

fn encode_str(s: &str) -> u64 {
    let mut value = 0;
    s.chars().for_each(|c| encode_char(&mut value, c));
    value
}

fn decode_hash_op(s: &str) -> (&str, usize, Option<usize>) {
    if s.ends_with('-') {
        let label = &s[0..s.len() - 1];
        let index = encode_str(label) as usize;
        (label, index, None)
    } else {
        let (label, focal_str) = s.split_once('=').unwrap();
        let index = encode_str(label) as usize;
        let focal = focal_str.parse::<usize>().unwrap();

        (label, index, Some(focal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_one(), 515210);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 246762);
    }
}
