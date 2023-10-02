use std::str::Chars;

pub trait IntoPairsIterator {
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
    fn into_pairs(self) -> Self::IntoIter;
}

impl<T> IntoPairsIterator for Vec<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = PairsIterator<impl Iterator<Item = T>, impl Iterator<Item = T>>;

    fn into_pairs(self) -> Self::IntoIter {
        PairsIterator {
            a: self.clone().into_iter().take(self.len() - 1),
            b: self.into_iter().skip(1),
        }
    }
}

impl<'a> IntoPairsIterator for Chars<'a>
{
    type Item = char;
    type IntoIter = PairsIterator<impl Iterator<Item = char>, impl Iterator<Item = char>>;

    fn into_pairs(self) -> Self::IntoIter {
        PairsIterator {
            a: self.clone().into_iter().take(self.clone().count() - 1),
            b: self.into_iter().skip(1),
        }
    }
}

pub struct PairsIterator<A, B>
{
    a: A,
    b: B,
}

impl<T, A, B> Iterator for PairsIterator<A, B>
where
    T: Clone,
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.a.next(), self.b.next());
        if result.0.is_none() || result.1.is_none() {
            None
        } else {
            Some((result.0.unwrap(), result.1.unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_works() {
        let values = vec!["A", "B", "C"];
        let mut pairs = values.into_pairs();

        assert_eq!(pairs.next().unwrap(), ("A", "B"));
        assert_eq!(pairs.next().unwrap(), ("B", "C"));
    }
}
