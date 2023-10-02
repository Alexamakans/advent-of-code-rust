use std::str::Chars;

pub trait IntoIncreasingSequenceIterator {
    type Item;
    type IntoIter;

    /// Takes some value, e.g. `vec![A, B, C]` and produces:
    /// ```
    /// vec![A]
    /// vec![B]
    /// vec![C]
    /// vec![A, A]
    /// vec![A, B]
    /// vec![A, C]
    /// vec![B, A]
    /// ```
    /// 
    /// and so on, forever.
    fn into_increasing_sequence(self) -> Self::IntoIter;
}

impl<T> IntoIncreasingSequenceIterator for Vec<T>
where
    T: Clone,
{
    type Item = Vec<T>;
    type IntoIter = IncreasingSequenceIterator<T>;

    fn into_increasing_sequence(self) -> Self::IntoIter {
        IncreasingSequenceIterator {
            current_indices: Vec::new(),
            max_index: self.len() - 1,
            values: self,
        }
    }
}

impl<'a> IntoIncreasingSequenceIterator for Chars<'a>
{
    type Item = Vec<char>;
    type IntoIter = IncreasingSequenceIterator<char>;

    fn into_increasing_sequence(self) -> Self::IntoIter {
        IncreasingSequenceIterator {
            current_indices: Vec::new(),
            max_index: self.clone().count() - 1,
            values: self.collect::<Vec<char>>(),
        }
    }
}

pub struct IncreasingSequenceIterator<T>
where
    T: Clone,
{
    current_indices: Vec<usize>,
    max_index: usize,
    values: Vec<T>,
}

impl<T> IncreasingSequenceIterator<T>
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

impl<T> Iterator for IncreasingSequenceIterator<T>
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
    fn test_increasing_sequence_works() {
        let values = vec!["A", "B", "C"];
        let mut increasing_sequence = values.into_increasing_sequence();

        assert_eq!(increasing_sequence.next().unwrap(), vec!["A"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["B"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["C"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["A", "A"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["A", "B"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["A", "C"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["B", "A"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["B", "B"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["B", "C"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["C", "A"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["C", "B"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["C", "C"]);
        assert_eq!(increasing_sequence.next().unwrap(), vec!["A", "A", "A"]);
    }
}
