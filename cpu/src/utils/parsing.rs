#[macro_export]
macro_rules! scanf {
    ($string:expr, $sep:expr, $( $x:ty ),+) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()).unwrap(),) *)
    }}
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
