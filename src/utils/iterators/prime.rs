use crate::utils::is_prime;

pub struct PrimeIterator {
    current: u32,
}

impl PrimeIterator {
    pub fn new() -> Self {
        PrimeIterator { current: 2 }
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(self.current) {
            // branchless way of adding 1 when we're at 2, and 2 otherwise
            self.current += (self.current & 1) + 1;
        }

        let result = Some(self.current);
        self.current += (self.current & 1) + 1;
        result
    }
}
