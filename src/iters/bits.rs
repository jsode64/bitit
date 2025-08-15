use crate::binary::Binary;

/// An iterator over the bits in a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bits<T: Binary> {
    /// The current value.
    n: T,

    /// The current bit.
    b: T,
}

impl<T: Binary> Bits<T> {
    /// Returns an iterator over the given value's bits.
    pub const fn new(n: T) -> Self {
        Self { n, b: T::ONE }
    }
}

impl<T: Binary> Iterator for Bits<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop if all bits were processed.
        (self.b != T::ZERO).then(|| {
            // Get the current bit and push the mask.
            let x = self.n & self.b;
            self.b <<= 1;

            x != T::ZERO
        })
    }
}
