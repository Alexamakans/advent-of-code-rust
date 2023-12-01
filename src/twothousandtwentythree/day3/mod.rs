use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut num_valid_triangles = 0;
        for line in input.lines() {
            let mut numbers = line.split_whitespace().map(|e| e.parse::<i32>().unwrap());
            let a = numbers.next().unwrap();
            let b = numbers.next().unwrap();
            let c = numbers.next().unwrap();
            if a + b > c && a + c > b && b + c > a {
                num_valid_triangles += 1;
            }
        }
        num_valid_triangles
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut num_valid_triangles = 0;
        let mut chunk = Vec::new();
        for line in input.lines() {
            if chunk.len() < 3 {
                chunk.push(line.split_whitespace().collect::<Vec<&str>>());
            }

            if chunk.len() == 3 {
                for index in 0..3 {
                    let line = format!(
                        "{} {} {}",
                        chunk[0][index], chunk[1][index], chunk[2][index]
                    );
                    let mut numbers = line.split_whitespace().map(|e| e.parse::<i32>().unwrap());
                    let a = numbers.next().unwrap();
                    let b = numbers.next().unwrap();
                    let c = numbers.next().unwrap();
                    if a + b > c && a + c > b && b + c > a {
                        num_valid_triangles += 1;
                    }
                }
                chunk.clear();
            }
        }
        num_valid_triangles
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"5 10 25
5 5 5",
            1,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 1050);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        assert_eq!(solver.part_two(), 1921);
    }
}
