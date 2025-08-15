use crate::binary::Binary;

/// An iterator over the 1-bits in a binary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ones<T: Binary> {
    /// The current value.
    n: T,
}

impl<T: Binary> Ones<T> {
    /// Returns an iterator over the 1-bits in the binary value.
    pub const fn new(n: T) -> Self {
        Self { n }
    }
}

impl<T: Binary> Iterator for Ones<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 1s.
        (self.n != T::ZERO).then(|| {
            // Get the least significant 1 and toggle it to 0.
            let b = self.n.lowest_one();
            self.n ^= b;

            b
        })
    }
}

impl<T: Binary> DoubleEndedIterator for Ones<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 1s.
        (self.n != T::ZERO).then(|| {
            // Get the most significant 1 and toggle it to 0.
            let b = self.n.highest_one();
            self.n ^= b;

            b
        })
    }
}
