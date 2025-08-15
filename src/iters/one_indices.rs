use crate::binary::Binary;

/// An iterator over the 1-bits in a binary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneIndices<T: Binary> {
    /// The current value.
    n: T,
}

impl<T: Binary> OneIndices<T> {
    /// Returns an iterator over the 1-bits in the binary value.
    pub const fn new(n: T) -> Self {
        Self { n }
    }
}

impl<T: Binary> Iterator for OneIndices<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 1s.
        (self.n != T::ZERO).then(|| {
            // Get the index of the least significant 1 and toggle it to 0.
            let i = self.n.trailing_zeros();
            self.n ^= T::ONE << i;

            i
        })
    }
}

impl<T: Binary> DoubleEndedIterator for OneIndices<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Stop if there are no more 1s.
        (self.n != T::ZERO).then(|| {
            // Get the index of the least significant 1 and toggle it to 0.
            let i = T::MAX_BIT_IDX - self.n.leading_zeros();
            self.n ^= T::ONE << i;

            i
        })
    }
}
