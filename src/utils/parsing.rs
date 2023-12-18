#[macro_export]
macro_rules! scanf {
    ($string:expr, $sep:expr, $( $x:ty ),+) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()).unwrap(),) *)
    }}
}

#[macro_export]
macro_rules! parse_world {
    () => {};

    (
        $( $char:literal => $name:ident )*
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        enum Tile {
            $(
                $name,
            )*
        }

        impl From<char> for Tile {
            fn from(c: char) -> Self {
                match c {
                    $(
                        $char => Self::$name,
                    )*
                    _ => unreachable!(),
                }
            }
        }

        impl From<Tile> for char {
            fn from(value: Tile) -> Self {
                match value {
                    $(
                        Tile::$name => $char,
                    )*
                }
            }
        }

        impl<'a> From<&'a Tile> for char {
            fn from(value: &'a Tile) -> Self {
                match value {
                    $(
                        Tile::$name => $char,
                    )*
                }
            }
        }

        struct World {
            tiles: Vec<Vec<Tile>>,
        }

        impl From<&str> for World {
            fn from(value: &str) -> Self {
                World {
                    tiles: value
                        .trim()
                        .lines()
                        .map(|line| line.trim().chars().map(Tile::from).collect())
                        .collect(),
                }
            }
        }

        impl std::fmt::Display for World {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for row in self.tiles.iter() {
                    writeln!(f, "{}", row.iter().map(char::from).collect::<String>()).unwrap();
                }

                Ok(())
            }
        }
    };
}

/// Given:
/// ```
/// "aaabba".chars()
/// ```
///
/// Returns:
/// ```
/// vec![
///     ((0, 3), 'a'),
///     ((3, 5), 'b'),
///     ((5, 6), 'a'),
/// ]
/// ```
pub fn extract_repeating_sequences<'a, I, T>(it: I) -> Vec<((usize, usize), T)>
where
    I: IntoIterator<Item = T>,
    T: Clone + Eq,
{
    let mut result = Vec::new();

    let mut start_index = 0 as usize;
    let mut index = 0 as usize;
    let mut prev: Option<T> = None;
    for item in it {
        match prev {
            Some(prev_unwrapped) => {
                if item != prev_unwrapped {
                    // end current sequence
                    result.push(((start_index, index), prev_unwrapped));

                    // start new sequence
                    start_index = index;
                }
            }
            None => (),
        }
        prev = Some(item);
        index += 1;
    }

    match prev {
        Some(prev_unwrapped) => result.push(((start_index, index), prev_unwrapped)),
        None => (),
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_repeating_sequences_works() {
        let cases = vec![("aaabba", vec![((0, 3), 'a'), ((3, 5), 'b'), ((5, 6), 'a')])];

        for case in cases {
            assert_eq!(
                extract_repeating_sequences(case.0.chars()),
                case.1,
                "input = {}",
                case.0
            );
        }
    }
}
