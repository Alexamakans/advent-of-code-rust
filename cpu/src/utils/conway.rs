use std::error::Error;
use std::fmt::Display;

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
    backbuffer: Vec<Vec<bool>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World {
            width,
            height,
            cells: vec![vec![false].repeat(height); width],
            backbuffer: vec![vec![false].repeat(height); width],
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: bool) {
        *self.cells.get_mut(y).unwrap().get_mut(x).unwrap() = state;
        *self.backbuffer.get_mut(y).unwrap().get_mut(x).unwrap() = state;
    }

    pub fn from_str(s: &str, on_character: char, off_character: char) -> Result<Self, ConwayError> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut cells = vec![vec![false].repeat(width); height];
        for (y, line) in s.lines().enumerate() {
            let row = cells.get_mut(y).unwrap();
            for (x, character) in line.chars().enumerate() {
                if character == on_character {
                    *row.get_mut(x).unwrap() = true;
                } else if character != off_character {
                    return Err(ConwayError {
                        error: Failure::UnexpectedCharacter(character),
                    });
                }
            }
        }

        Ok(World {
            width,
            height,
            backbuffer: cells.clone(),
            cells,
        })
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut alive_count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        // Managed to get like a -24% run time by doing this horribleness with the dx | dy instead of an if-statement.
                        match dx | dy {
                            0 => (),
                            _ => {
                                let y = y as i32 + dy;
                                let x = x as i32 + dx;
                                match usize::try_from(y) {
                                    Ok(y) => match usize::try_from(x) {
                                        Ok(x) => match self.cells.get_mut(y) {
                                            Some(row) => match row.get_mut(x) {
                                                Some(cell) => match cell {
                                                    true => alive_count += 1,
                                                    false => (),
                                                },
                                                _ => (),
                                            },
                                            _ => (),
                                        },
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }

                match alive_count {
                    3 => *self.backbuffer.get_mut(y).unwrap().get_mut(x).unwrap() = true,
                    2 => (),
                    _ => *self.backbuffer.get_mut(y).unwrap().get_mut(x).unwrap() = false,
                }
            }
        }

        self.cells.clone_from_slice(self.backbuffer.as_slice());
    }
}

#[derive(Clone, Debug)]
pub enum Failure {
    UnexpectedCharacter(char),
}

#[derive(Clone, Debug)]
pub struct ConwayError {
    error: Failure,
}

impl Error for ConwayError {}
impl Display for ConwayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error {
            Failure::UnexpectedCharacter(character) => write!(
                f,
                "encountered unexpected character '{}' while parsing world from string",
                character
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_from_str() {
        let w = World::from_str(
            r"###
...
###",
            '#',
            '.',
        );

        match w {
            Ok(w) => assert_eq!(
                w.cells,
                vec![
                    vec![true, true, true],
                    vec![false, false, false],
                    vec![true, true, true],
                ]
            ),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn test_update() {
        let mut w = World::from_str(
            r"###
...
###",
            '#',
            '.',
        )
        .unwrap();

        w.update();
        assert_eq!(
            w.cells,
            vec![
                vec![false, true, false],
                vec![false, false, false],
                vec![false, true, false],
            ]
        );
    }
}
