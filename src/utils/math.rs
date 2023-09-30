pub fn factorial(value: u32) -> u32 {
    if value == 0 {
        return 1;
    }

    let mut result = 1;
    for factor in 2..=value {
        result *= factor;
    }
    result
}

pub trait IntoPermutationIterator {
    type Item;
    type IntoIter;

    fn into_permutations(self) -> Self::IntoIter;
}

impl<T> IntoPermutationIterator for Vec<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = PermutationIterator<T>;

    fn into_permutations(self) -> Self::IntoIter {
        PermutationIterator {
            items: self.clone(),
            increase: 0,
            indices: (0..self.len()).collect::<Vec<usize>>(),
            i: 0,
        }
    }
}

pub struct PermutationIterator<T>
where
    T: Clone,
{
    items: Vec<T>,
    increase: usize,
    indices: Vec<usize>,
    i: i32,
}

impl<T> PermutationIterator<T>
where
    T: Clone,
{
    fn output(&self) -> Vec<T> {
        let mut result = Vec::new();
        for index in self.indices.iter() {
            result.push(self.items.get(*index).unwrap().clone());
        }
        result
    }
}

impl<T> Iterator for PermutationIterator<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // https://www.geeksforgeeks.org/iterative-approach-to-print-all-permutations-of-an-array/
        if self.increase == self.indices.len() - 1 {
            return None;
        }

        if self.i == 0 {
            self.i += 1;
            return Some(self.output());
        }

        if self.increase == 0 {
            self.indices.swap(self.increase, self.increase + 1);
            self.increase += 1;
            while self.increase < self.indices.len() - 1
                && self.indices.get(self.increase).unwrap()
                    > self.indices.get(self.increase + 1).unwrap()
            {
                self.increase += 1;
            }
        } else {
            if self.indices.get(self.increase + 1).unwrap() > self.indices.get(0).unwrap() {
                self.indices.swap(self.increase + 1, 0);
            } else {
                let mut start = 0;
                let mut end = self.increase as i32;
                let mut mid = ((start + end) / 2) as usize;
                let t_val = self.indices.get(self.increase + 1).unwrap();
                while !(self.indices.get(mid).unwrap() < t_val
                    && self.indices.get(mid - 1).unwrap() > t_val)
                {
                    if self.indices.get(mid).unwrap() < t_val {
                        end = (mid - 1) as i32;
                    } else {
                        start = (mid + 1) as i32;
                    }
                    mid = ((start + end) / 2) as usize;
                }

                self.indices.swap(self.increase + 1, mid);
            }

            for i in 0..=self.increase / 2 {
                self.indices.swap(i, self.increase - i);
            }

            self.increase = 0;
        }

        self.i += 1;
        Some(self.output())
    }
}

pub trait IntoSequenceGenerationIterator {
    type Item;
    type IntoIter;

    fn into_sequence_generator(self) -> Self::IntoIter;
}

impl<T> IntoSequenceGenerationIterator for Vec<T>
where
    T: Clone,
{
    type Item = Vec<T>;
    type IntoIter = SequenceGenerationIterator<T>;

    fn into_sequence_generator(self) -> Self::IntoIter {
        SequenceGenerationIterator {
            current_indices: Vec::new(),
            max_index: self.len() - 1,
            values: self,
        }
    }
}

pub struct SequenceGenerationIterator<T>
where
    T: Clone,
{
    current_indices: Vec<usize>,
    max_index: usize,
    values: Vec<T>,
}

impl<T> SequenceGenerationIterator<T>
where
    T: Clone,
{
    fn output(&self) -> Vec<T> {
        let mut result = Vec::new();
        for index in self.current_indices.iter() {
            result.push(self.values.get(*index).unwrap().clone());
        }
        result
    }

    pub fn set_indices(&mut self, indices: Vec<usize>) {
        self.current_indices = indices.clone();
    }
}

impl<T> Iterator for SequenceGenerationIterator<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = self.current_indices.len() as i32 - 1;

        while index >= 0 {
            let i = self.current_indices.get_mut(index as usize).unwrap();
            if *i == self.max_index {
                *i = 0;
                index -= 1;
            } else {
                *i += 1;
                break;
            }
        }

        if index == -1 {
            self.current_indices.insert(0, 0);
        }

        Some(self.output())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_works() {
        let v = vec![0, 1, 2];
        let permutations = v.clone().into_permutations().collect::<Vec<Vec<i32>>>();
        assert_eq!(permutations.len(), factorial(v.len() as u32) as usize);
        assert_eq!(
            permutations,
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![0, 2, 1],
                vec![2, 0, 1],
                vec![1, 2, 0],
                vec![2, 1, 0],
            ]
        );
    }

    #[test]
    fn test_sequence_generator_works() {
        let values = vec!["A", "B", "C"];
        let mut sequence_generator = values.into_sequence_generator();

        assert_eq!(sequence_generator.next().unwrap(), vec!["A"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["B"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["C"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["A", "A"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["A", "B"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["A", "C"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["B", "A"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["B", "B"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["B", "C"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["C", "A"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["C", "B"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["C", "C"]);
        assert_eq!(sequence_generator.next().unwrap(), vec!["A", "A", "A"]);
    }
}
