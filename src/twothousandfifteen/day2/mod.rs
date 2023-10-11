use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let lines = input.lines();
        let mut sum = 0;
        for line in lines {
            let mut parts = line.split('x');
            let a = parts.next().unwrap().parse::<i32>().unwrap();
            let b = parts.next().unwrap().parse::<i32>().unwrap();
            let c = parts.next().unwrap().parse::<i32>().unwrap();
            let area_ab = a * b;
            let area_ac = a * c;
            let area_bc = b * c;
            let smallest_area = vec![area_ab, area_ac, area_bc].into_iter().min().unwrap();

            sum += 2 * area_ab + 2 * area_ac + 2 * area_bc + smallest_area;
        }

        sum
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let lines = input.lines();
        let mut sum = 0;
        for line in lines {
            let mut parts = line.split('x');
            let a = parts.next().unwrap().parse::<i32>().unwrap();
            let b = parts.next().unwrap().parse::<i32>().unwrap();
            let c = parts.next().unwrap().parse::<i32>().unwrap();
            let largest_side = vec![a, b, c].into_iter().max().unwrap();
            let mut smallest_sides = vec![a, b, c];
            for (index, side) in smallest_sides.iter().enumerate() {
                if *side == largest_side {
                    smallest_sides.remove(index);
                    break;
                }
            }

            let ribbon_wrapping = smallest_sides
                .into_iter()
                .map(|v| 2 * v)
                .reduce(|acc, v| acc + v)
                .unwrap();
            let ribbon_bow = a * b * c;

            sum += ribbon_wrapping + ribbon_bow;
        }

        sum
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![
            ("2x3x4", 2 * 6 + 2 * 12 + 2 * 8 + 6),
            ("1x1x10", 2 * 1 + 2 * 10 + 2 * 10 + 1),
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 1606483);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![
            ("2x3x4", 2 + 2 + 3 + 3 + 2 * 3 * 4),
            ("1x1x10", 1 + 1 + 1 + 1 + 1 * 1 * 10),
        ];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 3842356);
    }
}
