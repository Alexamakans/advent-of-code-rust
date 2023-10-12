use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<String> for Solver {
    fn part_one_driver(&self, input: &str) -> String {
        let keys = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let (mut x, mut y) = (1, 1);

        let mut result = String::new();
        for line in input.lines() {
            for c in line.chars() {
                match c {
                    'U' => {
                        if y > 0 {
                            y -= 1
                        }
                    }
                    'R' => {
                        if x < 2 {
                            x += 1
                        }
                    }
                    'D' => {
                        if y < 2 {
                            y += 1
                        }
                    }
                    'L' => {
                        if x > 0 {
                            x -= 1
                        }
                    }
                    _ => unreachable!(),
                }
            }

            result.push_str(&keys[y][x].to_string());
        }

        result
    }

    fn part_two_driver(&self, input: &str) -> String {
        // see triangular_matrix_row_column_index_to_flat_index for why this is structured like this
        let keys = "1xx234xxxx56789xxxxxxxxABCxxxxxxxxxxxxxxDxxxx"
            .chars()
            .collect::<Vec<char>>();
        let (mut row, mut column) = (5, 1); // start at value "5"

        let mut result = String::new();
        for line in input.lines() {
            for c in line.chars() {
                match c {
                    'U' => {
                        if row > 1 && column > 1 {
                            row -= 1;
                            column -= 1;
                        }
                    }
                    'R' => {
                        if row > 1 && column < 5 {
                            row -= 1;
                            column += 1;
                        }
                    }
                    'D' => {
                        if row < 5 && column < 5 {
                            row += 1;
                            column += 1;
                        }
                    }
                    'L' => {
                        if row < 5 && column > 1 {
                            row += 1;
                            column -= 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            result.push(keys[triangular_matrix_row_column_index_to_flat_index(row, column)]);
        }

        result
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
        let cases = vec![(
            r"ULL
RRDDD
LURDL
UUUUD",
            "1985",
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), "74921");
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"ULL
RRDDD
LURDL
UUUUD",
            "5DB3",
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), "A6B35");
    }
}
