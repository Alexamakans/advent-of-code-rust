// TODO: Parse input
use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, _input: &str) -> u64 {
        solve((vec![59, 68, 82, 74], vec![543, 1020, 1664, 1022]))
    }

    fn part_two_driver(&self, _input: &str) -> u64 {
        solve((vec![59688274], vec![543102016641022]))
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 6)
    }
}

fn solve(races: (Vec<u64>, Vec<u64>)) -> u64 {
    let times = races.0;
    let distances = races.1;
    assert_eq!(times.len(), distances.len());

    let num_races = times.len();
    println!("Num races = {}", num_races);
    let mut result = 1;
    for i in 0..num_races {
        let time_to_beat = *times.get(i).unwrap();
        let distance = *distances.get(i).unwrap();
        let ways_to_win = (1..time_to_beat)
            .zip((1..time_to_beat).rev())
            .map(|(a, b)| a * b)
            .filter(|v| *v > distance)
            .count();
        println!("Ways to win = {}", ways_to_win);

        result *= ways_to_win as u64;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![((vec![7, 15, 30], vec![9, 40, 200]), 288)];

        for case in cases {
            assert_eq!(solve(case.0.clone()), case.1, "input = {:#?}", case.0);
        }

        assert_eq!(solver.part_one(), 275724);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![((vec![71530], vec![940200]), 71503)];

        for case in cases {
            assert_eq!(solve(case.0.clone()), case.1, "input = {:#?}", case.0);
        }

        assert_eq!(solver.part_two(), 37286485);
    }
}
