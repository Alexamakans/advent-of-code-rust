use std::error::Error;
use std::fmt::Display;

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
    backbuffer: Vec<Vec<bool>>,
    on_character: char,
    off_character: char,
}

impl World {
    pub fn new(width: usize, height: usize) -> Result<Self, ConwayError> {
        if width == 0 || height == 0 {
            Err(ConwayError {
                error: Failure::InvalidSize(width, height),
            })
        } else {
            Ok(World {
                width,
                height,
                cells: vec![vec![false].repeat(height); width],
                backbuffer: vec![vec![false].repeat(height); width],
                on_character: '#',
                off_character: '.',
            })
        }
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
            on_character,
            off_character,
        })
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_characters(&mut self, on_character: char, off_character: char) {
        self.on_character = on_character;
        self.off_character = off_character;
    }

    pub fn set_cell_state(&mut self, x: usize, y: usize, state: bool) -> Result<(), ConwayError> {
        if y >= self.cells.len() || x >= self.cells[0].len() {
            Err(ConwayError {
                error: Failure::OutOfBounds(x, y, self.width, self.height),
            })
        } else {
            *self.cells.get_mut(y).unwrap().get_mut(x).unwrap() = state;
            *self.backbuffer.get_mut(y).unwrap().get_mut(x).unwrap() = state;
            Ok(())
        }
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

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        for row in self.cells.iter() {
            let mut line = String::new();
            for cell in row {
                match cell {
                    true => line.push(self.on_character),
                    false => line.push(self.off_character),
                }
            }
            lines.push(line);
        }

        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Clone, Debug)]
pub enum Failure {
    /// InvalidSize(width, height)
    InvalidSize(usize, usize),
    /// UnexpectedCharacter(the_unexpected_character)
    UnexpectedCharacter(char),
    /// OutOfBounds(x, y, width, height)
    OutOfBounds(usize, usize, usize, usize),
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
            Failure::OutOfBounds(x, y, width, height) => write!(
                f,
                "tried to index ({}, {}), valid indices are ([0..{}], [0..{}])",
                x,
                y,
                width - 1,
                height - 1
            ),
            Failure::InvalidSize(width, height) => write!(
                f,
                "tried to create a world with dimensions ({}, {}), both width and height must be > 0",
                width,
                height
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
