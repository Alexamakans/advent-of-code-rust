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

    #[test]
    fn grid_index_to_flat_index_works() {
        let mut indices = Vec::new();
        for row in 1..=6 {
            indices.push(grid_index_to_flat_index(row, 1));
        }
        assert_eq!(indices, vec![0, 1, 3, 6, 10, 15]);

        let mut indices = Vec::new();
        for column in 1..=6 {
            indices.push(grid_index_to_flat_index(1, column));
        }
        assert_eq!(indices, vec![0, 2, 5, 9, 14, 20]);

        assert_eq!(grid_index_to_flat_index(2, 2), 4, "row 2 column 2");
        assert_eq!(grid_index_to_flat_index(3, 3), 12, "row 3 column 3");
        assert_eq!(grid_index_to_flat_index(4, 4), 24, "row 4 column 4");
        assert_eq!(grid_index_to_flat_index(5, 5), 40, "row 5 column 5");
        assert_eq!(grid_index_to_flat_index(6, 6), 60, "row 6 column 6");
        assert_eq!(grid_index_to_flat_index(9, 3), 57, "row 9 column 3");
    }

    #[test]
    fn asd() {
        for i in 0..21 {
            println!("{:?}", flat_index_to_grid_index(i));
        }
        for i in 0..10 {
            let (row, column) = flat_index_to_grid_index(i);
            assert_eq!(grid_index_to_flat_index(row, column), i);
        }
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
    let target_index = grid_index_to_flat_index(target_row, target_column);

    let first_value = lines
        .nth(8)
        .unwrap()
        .split_whitespace()
        .nth(7)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let first_index = grid_index_to_flat_index(first_value_row, first_value_column);

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

// TODO: This no work good
/// row and column start at 1, returned flat index starts at 0.
pub fn grid_index_to_flat_index(row: usize, column: usize) -> usize {
    let column = column;
    let row = row - 1;

    let column = column + row;

    let triangular_number = column * (column + 1) / 2;
    println!("triangular_number={}  row={}", triangular_number, row);
    let index = triangular_number - row - 1; // - 1 for zero indexing
    index
}

/// Transposed version of https://stackoverflow.com/a/9674523
///
/// Returns (row, column) where row and column start at 1.
pub fn flat_index_to_grid_index(index: usize) -> (usize, usize) {
    let row = (-0.5 + ((0.25_f32 + 2_f32 * index as f32) as f32).sqrt()) as i32;
    let triangular_number = row * (row + 1) / 2;
    let column = index as i32 - triangular_number;
    (1 + (row - column) as usize, 1 + column as usize) // + 1 for one indexing to conform to puzzle example
}
