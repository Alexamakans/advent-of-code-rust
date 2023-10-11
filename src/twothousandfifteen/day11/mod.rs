use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<String> for Solver {
    fn part_one_driver(&self, input: &str) -> String {
        get_next_password(&input)
    }

    fn part_two_driver(&self, input: &str) -> String {
        get_next_password(&get_next_password(&input))
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 11)
    }
}

fn get_next_password(s: &str) -> String {
    let values = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    fn get_index_from_value<T: Eq>(values: &Vec<T>, value: T) -> Option<usize> {
        for (i, e) in values.iter().enumerate() {
            if *e == value {
                return Some(i);
            }
        }
        None
    }

    let mut increasing_sequence = values.clone().into_increasing_sequence();
    increasing_sequence.set_indices(
        s.chars()
            .map(|c| get_index_from_value(&values, &c.to_string()).unwrap())
            .collect::<Vec<usize>>(),
    );

    loop {
        let next = increasing_sequence.next().unwrap().join("");
        if is_valid_password(&next) {
            return next;
        }
    }
}

fn is_valid_password(s: &str) -> bool {
    let has_no_blacklisted_characters = !s.contains(&['i', 'o', 'l']);
    let has_increasing_straight = 'found: {
        let num_chars = s.chars().count();
        for i in 0..num_chars - 2 {
            let chars = vec![
                s.chars().nth(i).unwrap(),
                s.chars().nth(i + 1).unwrap(),
                s.chars().nth(i + 2).unwrap(),
            ];
            let mut as_u8 = chars.into_iter().map(|v| v as u8);
            let (a, b, c) = (
                as_u8.next().unwrap(),
                as_u8.next().unwrap(),
                as_u8.next().unwrap(),
            );
            if a == b - 1 && a == c - 2 {
                break 'found true;
            }
        }
        false
    };
    let has_at_least_two_distinct_non_overlapping_pairs = 'found: {
        let pairs = s.chars().into_pairs().collect::<Vec<(char, char)>>();
        let num_pairs = pairs.len();
        for i in 0..num_pairs {
            let a = pairs.get(i).unwrap();
            if a.0 != a.1 {
                continue;
            }
            for j in i + 2..num_pairs {
                let b = pairs.get(j).unwrap();
                if b.0 != b.1 {
                    continue;
                }
                if a != b {
                    break 'found true;
                }
            }
        }
        false
    };

    has_no_blacklisted_characters
        && has_increasing_straight
        && has_at_least_two_distinct_non_overlapping_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![("abcdefgh", "abcdffaa")];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), "hxbxxyzz");
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        assert_eq!(solver.part_two(), "hxcaabcc");
    }
}
