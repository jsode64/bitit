use crate::binary::Binary;

/// An iterator over the 1-bits in a binary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroIndices<T: Binary> {
    /// The current value.
    n: T,
}

impl<T: Binary> ZeroIndices<T> {
    /// Returns an iterator over the 1-bits in the binary value.
    pub const fn new(n: T) -> Self {
        Self { n }
    }
}

impl<T: Binary> Iterator for ZeroIndices<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 0s.
        (self.n != T::FULL).then(|| {
            // Get the index of the least significant 0 and toggle it to 1.
            let i = self.n.trailing_ones();
            self.n ^= T::ONE << i;

            i
        })
    }
}

impl<T: Binary> DoubleEndedIterator for ZeroIndices<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 0s.
        (self.n != T::FULL).then(|| {
            // Get the index of the least significant 0 and toggle it to 1.
            let i = T::MAX_BIT_IDX - self.n.leading_ones();
            self.n ^= T::ONE << i;

            i
        })
    }
}
