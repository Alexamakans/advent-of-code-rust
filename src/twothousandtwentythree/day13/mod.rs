use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        let mut result = 0;
        for chunk in input.replace("\r", "").split("\n\n") {
            let score = match get_horizontal_mirror_row_index(chunk) {
                Some(row_index) => 100 * row_index,
                None => match get_vertical_mirror_column_index(chunk) {
                    Some(column_index) => column_index,
                    None => todo!(),
                },
            };

            result += score;
        }

        result
    }

    fn part_two_driver(&self, input: &str) -> usize {
        let mut result = 0;
        for chunk in input.replace("\r", "").split("\n\n") {
            let score = match get_horizontal_mirror_row_index_part_2(chunk) {
                Some(row_index) => 100 * row_index,
                None => match get_vertical_mirror_column_index_part_2(chunk) {
                    Some(column_index) => column_index,
                    None => continue,
                },
            };
            result += score;
        }

        result
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 13)
    }
}

fn get_horizontal_mirror_row_index(s: &str) -> Option<usize> {
    let lines = s.lines().collect::<Vec<&str>>();
    'next_line: for i in 0..lines.len() - 1 {
        let line = lines.get(i).unwrap();
        if line == lines.get(i + 1).unwrap() {
            let mut index_pairs = Vec::new();
            let mut next_index_1 = i as i32 - 1;
            let mut next_index_2 = i + 2;
            loop {
                if next_index_1 >= 0 && next_index_2 < lines.len() {
                    index_pairs.push((next_index_1, next_index_2));
                    next_index_1 -= 1;
                    next_index_2 += 1;
                } else {
                    break;
                }
            }

            for indices in index_pairs {
                if lines.get(indices.0 as usize).unwrap() != lines.get(indices.1).unwrap() {
                    continue 'next_line;
                }
            }

            return Some(i + 1);
        }
    }

    None
}

fn get_vertical_mirror_column_index(s: &str) -> Option<usize> {
    let lines = s.lines().collect::<Vec<&str>>();
    let line_length = lines.get(0).unwrap().len() - 1;
    'next_line: for i in 0..line_length {
        let is_mirror_line_candidate = 'exit: {
            for row in 0..lines.len() {
                if lines.get(row).unwrap().chars().nth(i).unwrap()
                    != lines.get(row).unwrap().chars().nth(i + 1).unwrap()
                {
                    break 'exit false;
                }
            }
            true
        };
        if is_mirror_line_candidate {
            let mut index_pairs = Vec::new();
            let mut next_index_1 = i as i32 - 1;
            let mut next_index_2 = i + 2;
            loop {
                if next_index_1 >= 0 && next_index_2 < line_length {
                    index_pairs.push((next_index_1, next_index_2));
                    next_index_1 -= 1;
                    next_index_2 += 1;
                } else {
                    break;
                }
            }

            for indices in index_pairs {
                for row in 0..lines.len() {
                    if lines
                        .get(row)
                        .unwrap()
                        .chars()
                        .nth(indices.0 as usize)
                        .unwrap()
                        != lines.get(row).unwrap().chars().nth(indices.1).unwrap()
                    {
                        continue 'next_line;
                    }
                }
            }

            return Some(i + 1);
        }
    }

    None
}

fn get_horizontal_mirror_row_index_part_2(chunk: &str) -> Option<usize> {
    let num_lines = chunk.lines().count();
    for row_index in 0..num_lines - 1 {
        let mut diffs = 0;
        let idx = row_index + 1;

        let mut index_pairs = Vec::new();
        let mut next_index_1 = row_index as i32;
        let mut next_index_2 = idx;
        loop {
            if next_index_1 >= 0 && next_index_2 < num_lines {
                index_pairs.push((next_index_1, next_index_2));
                next_index_1 -= 1;
                next_index_2 += 1;
            } else {
                break;
            }
        }
        for pair in index_pairs {
            diffs += horizontal_line_diff(
                chunk.lines().nth(pair.0 as usize).unwrap(),
                chunk.lines().nth(pair.1).unwrap(),
            );
        }

        if diffs == 1 {
            return Some(idx);
        }
    }

    None
}

fn horizontal_line_diff(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(c1, c2)| c1 != c2).count()
}

fn get_vertical_mirror_column_index_part_2(chunk: &str) -> Option<usize> {
    let num_columns = chunk.lines().nth(0).unwrap().len();
    for row_index in 0..num_columns - 1 {
        let mut diffs = 0;
        let idx = row_index + 1;

        let mut index_pairs = Vec::new();
        let mut next_index_1 = row_index as i32;
        let mut next_index_2 = idx;
        loop {
            if next_index_1 >= 0 && next_index_2 < num_columns {
                index_pairs.push((next_index_1, next_index_2));
                next_index_1 -= 1;
                next_index_2 += 1;
            } else {
                break;
            }
        }
        for pair in index_pairs {
            diffs += vertical_line_diff(chunk, pair.0 as usize, pair.1);
        }

        if diffs == 1 {
            return Some(idx);
        }
    }

    None
}

fn vertical_line_diff(chunk: &str, idx_a: usize, idx_b: usize) -> usize {
    chunk
        .lines()
        .map(|line| line.chars().nth(idx_a).unwrap())
        .zip(chunk.lines().map(|line| line.chars().nth(idx_b).unwrap()))
        .filter(|(c1, c2)| c1 != c2)
        .count()
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

        assert_eq!(solver.part_one(), 32723);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        assert_eq!(solver.part_two(), 34536);
    }
}
