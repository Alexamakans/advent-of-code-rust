use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        let target_presents = input.parse::<u32>().unwrap();
        let num_houses = 3_000_000;
        let presents_per_elf = 10;
        // 10 + e * 10 because all house numbers are divisible by 1 and itself
        let mut houses = (0..num_houses).map(|e| presents_per_elf + e * presents_per_elf).collect::<Vec<u32>>();

        for elf_number in 2..num_houses {
             // first house is account for in our initialization, so we can skip the house it starts at
            let mut house_number = elf_number * 2;
            loop {
                if house_number >= houses.len() as u32 {
                    break;
                }

                let presents = houses.get_mut(house_number as usize).unwrap();
                *presents += elf_number * presents_per_elf;
                house_number += elf_number;
            }
        }

        for (house_number, presents) in houses.into_iter().enumerate() {
            if presents >= target_presents {
                return house_number;
            }
        }

        panic!("increase size of houses vector");
    }

    fn part_two_driver(&self, input: &str) -> usize {
        let target_presents = input.parse::<u32>().unwrap();
        let num_houses = 3_000_000;
        let presents_per_elf = 11;
        // 11 + e * 11 because all house numbers are divisible by 1 and itself
        let mut houses = (0..num_houses).map(|e| presents_per_elf + e * presents_per_elf).collect::<Vec<u32>>();

        for elf_number in 2..num_houses {
             // first house is account for in our initialization, so we can skip the house it starts at
            let mut house_number = elf_number * 2;
            let mut houses_left_to_visit = 49;
            while houses_left_to_visit > 0 {
                if house_number >= houses.len() as u32 {
                    break;
                }

                let presents = houses.get_mut(house_number as usize).unwrap();
                *presents += elf_number * presents_per_elf;
                house_number += elf_number;
                houses_left_to_visit -= 1;
            }
        }

        for (house_number, presents) in houses.into_iter().enumerate() {
            if presents >= target_presents {
                return house_number;
            }
        }

        panic!("increase size of houses vector");
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![("100", 6)];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 665280);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        assert_eq!(solver.part_two(), 705600);
    }
}
