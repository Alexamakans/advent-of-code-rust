use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u32> for Solver {
    fn part_one_driver(&self, input: &str) -> u32 {
        let data = parse_input(input, 6, 6);
        println!("{:?}", data);

        let mut prev_value = data.first_value;
        let mut cur_index = data.first_index + 1;

        while cur_index <= data.target_index {
            let new_value = (prev_value.checked_mul(data.multiplier).unwrap())
                .checked_rem_euclid(data.divisor)
                .unwrap();
            prev_value = new_value;
            cur_index += 1;
        }

        prev_value as u32
    }

    fn part_two_driver(&self, input: &str) -> u32 {
        todo!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 25)
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

        assert_eq!(solver.part_one(), 19980801);
    }

    #[test]
    fn part_two_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_two(), 123);
    }
}

#[derive(Debug)]
struct InputData {
    first_value: u64,
    first_index: usize,
    multiplier: u64,
    divisor: u64,
    target_index: usize,
}

fn parse_input(input: &str, first_value_row: usize, first_value_column: usize) -> InputData {
    let mut lines = input.lines();
    let (row_str, column_str) = lines
        .next()
        .unwrap()
        .replace("Enter the code at row ", "")
        .replace(", column", "")
        .replace(".", "")
        .split_once(&[' '])
        .map(|e| (e.0.to_owned(), e.1.to_owned()))
        .unwrap();
    let target_row = row_str.parse::<usize>().unwrap();
    let target_column = column_str.parse::<usize>().unwrap();
    let target_index = triangular_matrix_row_column_index_to_flat_index(target_row, target_column);

    let first_value = lines
        .nth(8)
        .unwrap()
        .split_whitespace()
        .nth(7)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let first_index =
        triangular_matrix_row_column_index_to_flat_index(first_value_row, first_value_column);

    let multiplier = lines
        .nth(1)
        .unwrap()
        .split_once(&[' '])
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let divisor = lines
        .nth(0)
        .unwrap()
        .split_once(&[' '])
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();

    InputData {
        first_value,
        first_index,
        multiplier,
        divisor,
        target_index,
    }
}
