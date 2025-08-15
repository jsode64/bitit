use crate::binary::Binary;

/// An iterator over the bits in a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitsRev<T: Binary> {
    /// The current value.
    n: T,

    /// The current bit.
    b: T,
}

impl<T: Binary> BitsRev<T> {
    /// Returns an iterator over the given value's bits in reverse.
    pub const fn new(n: T) -> Self {
        Self { n, b: T::TOP }
    }
}

impl<T: Binary> Iterator for BitsRev<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if all bits were processed.
        (self.b != T::ZERO).then(|| {
            // Get the current bit and push the mask.
            let x = self.n & self.b;
            self.b >>= 1;

            x != T::ZERO
        })
    }
}
