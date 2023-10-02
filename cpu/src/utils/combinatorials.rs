use std::{fmt::Debug, ops::Add};

/// This is the same as N choose K when all elements are 1, there are N elements, and the target value is K.
pub fn get_combinations_to_reach_target_value<T>(
    descending_sorted_values: &Vec<T>,
    target_value: T,
) -> Vec<Vec<T>>
where
    T: Clone + Copy + Add<Output = T> + PartialOrd<T> + Debug + Default,
{
    _get_combinations_to_reach_target_value(
        descending_sorted_values,
        0,
        T::default(),
        target_value,
        &Vec::new(),
    )
}

pub fn _get_combinations_to_reach_target_value<T>(
    descending_sorted_values: &Vec<T>,
    start_index: usize,
    start_value: T,
    target_value: T,
    picked_elements: &Vec<T>,
) -> Vec<Vec<T>>
where
    T: Clone + Copy + Add<Output = T> + PartialOrd<T> + Debug,
{
    fn get_next_valid_entries<T: Clone>(
        descending_sorted_values: &Vec<T>,
        current_value: <T as Add>::Output,
        target_value: T,
        start_index: usize,
    ) -> Vec<(usize, T)>
    where
        T: Clone + Copy + Add<Output = T> + PartialOrd<T> + Debug,
    {
        let mut result = Vec::new();
        for index in start_index..descending_sorted_values.len() {
            let value = descending_sorted_values.get(index).unwrap().to_owned();
            if current_value + value <= target_value {
                result.push((index, value));
            }
        }

        result
    }

    let mut result = Vec::new();
    for index in start_index..descending_sorted_values.len() {
        let val = descending_sorted_values.get(index).unwrap().to_owned();
        let current_value = start_value + val;
        if current_value == target_value {
            result.push(Vec::from_iter(
                picked_elements
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(val)),
            ));
            continue;
        }
        let next_valid_entries = get_next_valid_entries(
            descending_sorted_values,
            current_value,
            target_value,
            index + 1,
        );
        for next_valid_entry in next_valid_entries {
            let new_value = current_value + next_valid_entry.1;
            if new_value == target_value {
                result.push(Vec::from_iter(
                    picked_elements
                        .clone()
                        .into_iter()
                        .chain([val, next_valid_entry.1]),
                ));
            } else {
                for combination in _get_combinations_to_reach_target_value(
                    descending_sorted_values,
                    next_valid_entry.0 + 1,
                    new_value,
                    target_value,
                    &picked_elements
                        .clone()
                        .into_iter()
                        .chain([val, next_valid_entry.1])
                        .collect::<Vec<T>>(),
                ) {
                    result.push(combination);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_combinations_to_reach_target_value_works() {
        let cases = vec![
            (vec![3, 2, 1], 5, vec![vec![3, 2]]),
            (vec![3, 3, 2, 1], 5, vec![vec![3, 2], vec![3, 2]]),
            (
                vec![20, 15, 10, 5, 5],
                25,
                vec![vec![20, 5], vec![20, 5], vec![15, 10], vec![15, 5, 5]],
            ),
            (
                vec![1, 1, 1, 1, 1],
                3,
                vec![
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                ],
            ),
        ];

        for case in cases {
            let got = get_combinations_to_reach_target_value(&case.0, case.1);
            let want = case.2;
            assert_eq!(got, want);
        }
    }
}
