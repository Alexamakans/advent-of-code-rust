use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i64> for Solver {
    fn part_one_driver(&self, input: &str) -> i64 {
        let mut result = 0;
        for line in input.lines() {
            let next_value = process_line(line);
            println!("{}   {}", line, next_value);
            result += next_value;
        }

        result
    }

    fn part_two_driver(&self, input: &str) -> i64 {
        let mut result = 0;
        for line in input.lines() {
            let next_value = process_line_part_two(line);
            println!("{}   {}", line, next_value);
            result += next_value;
        }

        result
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 9)
    }
}

fn process_line(line: &str) -> i64 {
    let mut rows = vec![line
        .split_whitespace()
        .map(|e| match e.trim().parse::<i64>() {
            Ok(v) => v,
            Err(_) => panic!("couldn't parse '{}'", e),
        })
        .collect::<Vec<i64>>()];

    let mut row_index = 0;
    loop {
        let row = rows.get(row_index).unwrap();
        println!("{:#?}", row);
        let differences = row
            .clone()
            .into_pairs()
            .map(|(a, b)| b - a)
            .collect::<Vec<i64>>();
        if differences.iter().all(|v| *v == 0) {
            break;
        } else {
            rows.push(differences);
            row_index += 1;
        }
    }

    let last_value_in_last_row = *rows.iter().last().unwrap().iter().last().unwrap();
    rows.iter_mut().last().unwrap().push(last_value_in_last_row);

    for row_index in (1..rows.len()).rev() {
        let last_value_in_row = *rows.get(row_index).unwrap().iter().last().unwrap();
        let next_row = rows.get_mut(row_index - 1).unwrap();
        let last_value_in_next_row = *next_row.iter().last().unwrap();
        next_row.push(last_value_in_row + last_value_in_next_row);
    }

    let next_value = *rows.get(0).unwrap().iter().last().unwrap();

    next_value
}

fn process_line_part_two(line: &str) -> i64 {
    let mut rows = vec![line
        .split_whitespace()
        .map(|e| match e.trim().parse::<i64>() {
            Ok(v) => v,
            Err(_) => panic!("couldn't parse '{}'", e),
        })
        .collect::<Vec<i64>>()];

    let mut row_index = 0;
    loop {
        let row = rows.get(row_index).unwrap();
        println!("{:#?}", row);
        let differences = row
            .clone()
            .into_pairs()
            .map(|(a, b)| b - a)
            .collect::<Vec<i64>>();
        if differences.iter().all(|v| *v == 0) {
            break;
        } else {
            rows.push(differences);
            row_index += 1;
        }
    }

    let first_value_in_last_row = *rows.iter().last().unwrap().first().unwrap();
    rows.iter_mut()
        .last()
        .unwrap()
        // These are all the same value so we don't have to insert it at the beginning.
        .push(first_value_in_last_row);

    for row_index in (1..rows.len()).rev() {
        let first_value_in_row = *rows.get(row_index).unwrap().first().unwrap();
        let next_row = rows.get_mut(row_index - 1).unwrap();
        let first_value_in_next_row = *next_row.first().unwrap();
        next_row.insert(0, first_value_in_next_row - first_value_in_row);
    }

    let next_value = *rows.get(0).unwrap().first().unwrap();

    next_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_one(), 123);
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
