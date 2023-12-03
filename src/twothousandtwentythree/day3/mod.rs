use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let world = parse_input(input);
        let mut result = 0;
        for number in world.numbers {
            for part in world.parts.iter() {
                if number.contains_part(part) {
                    result += number.number.unwrap();
                    break;
                }
            }
        }

        result
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let world = parse_input(input);
        let mut part_numbers = Vec::new();
        for number in world.numbers {
            for part in world.parts.iter() {
                if number.contains_part(part) {
                    part_numbers.push(number);
                    break;
                }
            }
        }

        let mut result = 0;
        for part in world.parts.iter().filter(|p| p.content == "*") {
            let mut num_count = 0;
            let mut product = 1;
            for number in part_numbers.iter() {
                if number.contains_part(part) {
                    product *= number.number.unwrap();
                    num_count += 1;
                }
            }

            if num_count == 2 {
                result += product;
            }
        }

        result
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 3)
    }
}

fn parse_input(s: &str) -> World {
    let mut world = World {
        numbers: Vec::new(),
        parts: Vec::new(),
    };
    for (line_index, line) in s.lines().enumerate() {
        let mut chars = line.chars().enumerate();
        let mut digit_start = 0;
        let mut digits: Vec<char> = Vec::new();
        loop {
            match chars.next() {
                Some((char_index, c)) => match c {
                    '0'..='9' => {
                        if digits.len() == 0 {
                            digit_start = char_index;
                        }
                        digits.push(c);
                    }
                    _ => {
                        if digits.len() != 0 {
                            let content = digits.iter().collect::<String>();
                            world.numbers.push(Rect {
                                min: Position {
                                    x: digit_start as i32 - 1,
                                    y: line_index as i32 - 1,
                                },
                                max: Position {
                                    x: digit_start as i32 + content.len() as i32,
                                    y: line_index as i32 + 1,
                                },
                                number: Some(content.parse().unwrap()),
                                content,
                            });
                            digits.clear();
                        }
                        if c != '.' {
                            world.parts.push(Rect {
                                min: Position {
                                    x: char_index as i32,
                                    y: line_index as i32,
                                },
                                max: Position {
                                    x: char_index as i32,
                                    y: line_index as i32,
                                },
                                number: None,
                                content: c.to_string(),
                            });
                        }
                    }
                },
                None => break,
            }
        }

        if digits.len() != 0 {
            let content = digits.iter().collect::<String>();
            world.numbers.push(Rect {
                min: Position {
                    x: digit_start as i32 - 1,
                    y: line_index as i32 - 1,
                },
                max: Position {
                    x: digit_start as i32 + content.len() as i32,
                    y: line_index as i32 + 1,
                },
                number: Some(content.parse().unwrap()),
                content,
            });
            digits.clear();
        }
    }

    world
}

#[derive(Debug, Clone)]
struct Rect {
    min: Position,
    max: Position,
    number: Option<i32>,
    content: String,
}

impl Rect {
    fn contains_part(&self, other: &Rect) -> bool {
        let c = Position {
            x: other.min.x,
            y: other.min.y,
        };
        c.x >= self.min.x && c.y >= self.min.y && c.x <= self.max.x && c.y <= self.max.y
    }
}

struct World {
    numbers: Vec<Rect>,
    parts: Vec<Rect>,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            4361,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 535078);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            467835,
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 75312571);
    }
}
