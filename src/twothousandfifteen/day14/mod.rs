use crate::scanf;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u32> for Solver {
    fn part_one_driver(&self, input: &str) -> u32 {
        get_fastest_reindeer_from_input(input, 2503)
    }

    fn part_two_driver(&self, input: &str) -> u32 {
        get_highest_points_from_input(input, 2503)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 14)
    }
}

fn get_fastest_reindeer_from_input(input: &str, duration_seconds: u32) -> u32 {
    let mut reindeer = input
        .lines()
        .map(|line| ReindeerStats::from(line))
        .collect::<Vec<ReindeerStats>>();

    reindeer
        .iter_mut()
        .map(|e| e.get_distance_traveled_after_seconds(duration_seconds))
        .max()
        .unwrap()
}

fn get_highest_points_from_input(input: &str, duration_seconds: u32) -> u32 {
    let mut reindeer = input
        .lines()
        .map(|line| ReindeerStats::from(line))
        .collect::<Vec<ReindeerStats>>();

    let mut scores = vec![0; reindeer.len()];

    for _ in 0..duration_seconds {
        for r in reindeer.iter_mut() {
            r.get_distance_traveled_after_seconds(1);
        }

        let best_distance_traveled = reindeer
            .iter()
            .map(|e| e.distance_traveled_km)
            .max()
            .unwrap();

        for (index, r) in reindeer.iter().enumerate() {
            if r.distance_traveled_km == best_distance_traveled {
                let score = scores.get_mut(index).unwrap();
                *score += 1;
            }
        }
    }

    scores.into_iter().max().unwrap()
}

#[derive(PartialEq, Eq)]
struct ReindeerStats {
    name: String,
    speed_kilometers_per_second: u32,
    stamina_seconds: u32,
    resting_time_seconds: u32,
    distance_traveled_km: u32,
    exhaustion_seconds_left: u32,
    stamina_seconds_left: u32,
}

impl ReindeerStats {
    fn get_distance_traveled_after_seconds(&mut self, seconds: u32) -> u32 {
        for _ in 0..seconds {
            if self.exhaustion_seconds_left == 0 {
                self.distance_traveled_km += self.speed_kilometers_per_second;
                self.stamina_seconds_left -= 1;
                if self.stamina_seconds_left == 0 {
                    self.exhaustion_seconds_left = self.resting_time_seconds;
                }
            } else {
                self.exhaustion_seconds_left -= 1;
                if self.exhaustion_seconds_left == 0 {
                    self.stamina_seconds_left = self.stamina_seconds;
                }
            }
        }

        self.distance_traveled_km
    }
}

impl From<&str> for ReindeerStats {
    fn from(value: &str) -> Self {
        // Comet[0] can[1] fly[2] 14[3] km/s[4] for[5] 10[6] seconds,[7] but[8] then[9] must[10] rest[11] for[12] 127[13] seconds.[14]
        let parts = scanf!(
            value, " ", String, String, String, u32, String, String, u32, String, String, String,
            String, String, String, u32, String
        );

        ReindeerStats {
            name: parts.0,
            speed_kilometers_per_second: parts.3,
            stamina_seconds: parts.6,
            resting_time_seconds: parts.13,
            distance_traveled_km: 0,
            exhaustion_seconds_left: 0,
            stamina_seconds_left: parts.6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let cases = vec![(
            r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            1120,
        )];

        for case in cases {
            assert_eq!(
                get_fastest_reindeer_from_input(case.0, 1000),
                case.1,
                "input = {}",
                case.0
            );
        }

        let solver = Solver {};
        assert_eq!(solver.part_one(), 2655);
    }

    #[test]
    fn part_two_works() {
        let cases = vec![(
            r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            689,
        )];

        for case in cases {
            assert_eq!(
                get_highest_points_from_input(case.0, 1000),
                case.1,
                "input = {}",
                case.0
            );
        }

        let solver = Solver {};
        assert_eq!(solver.part_two(), 1059);
    }
}
