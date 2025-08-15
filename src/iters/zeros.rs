use crate::binary::Binary;

/// An iterator over the 1-bits in a binary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zeros<T: Binary> {
    /// The current value.
    n: T,
}

impl<T: Binary> Zeros<T> {
    /// Returns an iterator over the 1-bits in the binary value.
    pub const fn new(n: T) -> Self {
        Self { n }
    }
}

impl<T: Binary> Iterator for Zeros<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 0s.
        (self.n != T::FULL).then(|| {
            // Get the least significant 0 and toggle it to 1.
            let b = self.n.lowest_zero();
            self.n ^= b;

            b
        })
    }
}

impl<T: Binary> DoubleEndedIterator for Zeros<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 0s.
        (self.n != T::FULL).then(|| {
            // Get the most significant 0 and toggle it to 1.
            let b = self.n.highest_zero();
            self.n ^= b;

            b
        })
    }
}
