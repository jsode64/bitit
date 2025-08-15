use crate::{
    binary::Binary,
    iters::{Bits, BitsRev, OneIndices, Ones, ZeroIndices, Zeros},
};

/// A trait for iterating a binary value's bits.
pub trait BitIter: Binary {
    /// Returns an iterator over the value's zero-bits.
    ///
    /// Contains values with one bit set to one, that being the zero-bit it's representing.
    ///
    /// # Examples
    /// ```
    /// use bitit::BitIter;
    ///
    /// let x: u8 = 0b10101100;
    /// let mut iter = x.zeros();
    ///
    /// assert_eq!(iter.next(), Some(0b00000001));
    /// assert_eq!(iter.next(), Some(0b00000010));
    /// assert_eq!(iter.next(), Some(0b00010000));
    /// assert_eq!(iter.next(), Some(0b01000000));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn zeros(self) -> Zeros<Self> {
        Zeros::new(self)
    }

    /// Returns an iterator over the value's one-bits.
    ///
    /// Contains values with one bit set to one, that being the one-bit it's representing.
    ///
    /// # Examples
    /// ```
    /// use bitit::BitIter;
    ///
    /// let x: u8 = 0b10101100;
    /// let mut iter = x.ones();
    ///
    /// assert_eq!(iter.next(), Some(0b00000100));
    /// assert_eq!(iter.next(), Some(0b00001000));
    /// assert_eq!(iter.next(), Some(0b00100000));
    /// assert_eq!(iter.next(), Some(0b10000000));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn ones(self) -> Ones<Self> {
        Ones::new(self)
    }

    /// Returns an iterator over the indices of the value's zero-bits.
    ///
    /// Contains a `u32` in range `0..N` where `N` is the value's size in bits.
    /// The index `0` is the least significant bit, with `N - 1` being the most significant.
    ///
    /// # Examples
    /// ```
    /// use bitit::BitIter;
    ///
    /// let x: u8 = 0b10101100;
    /// let mut iter = x.zero_indices();
    ///
    /// assert_eq!(iter.next(), Some(0));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(4));
    /// assert_eq!(iter.next(), Some(6));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn zero_indices(self) -> ZeroIndices<Self> {
        ZeroIndices::new(self)
    }

    /// Returns an iterator over the indices of the value's one-bits.
    ///
    /// Contains a `u32` in range `0..N` where `N` is the value's size in bits.
    /// The index `0` is the least significant bit, with `N - 1` being the most significant.
    ///
    /// # Examples
    /// ```
    /// use bitit::BitIter;
    ///
    /// let x: u8 = 0b10101100;
    /// let mut iter = x.one_indices();
    ///
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), Some(7));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn one_indices(self) -> OneIndices<Self> {
        OneIndices::new(self)
    }

    /// Returns an iterator over the value's bits.
    ///
    /// Contains a `bool`, with `true` meaning the bit is one and `false` meaning it is zero.
    ///
    /// # Examples
    /// ```
    /// use bitit::BitIter;
    ///
    /// let x: u8 = 0b10101100;
    /// let mut iter = x.bits();
    ///
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn bits(self) -> Bits<Self> {
        Bits::new(self)
    }

    /// Returns an iterator over the value's bits reversed.
    ///
    /// Contains a `bool`, with `true` meaning the bit is one and `false` meaning it is zero.
    ///
    /// # Examples
    /// ```
    /// use bitit::BitIter;
    ///
    /// let x: u8 = 0b10101100;
    /// let mut iter = x.bits_rev();
    ///
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn bits_rev(self) -> BitsRev<Self> {
        BitsRev::new(self)
    }
}

impl<T: Binary> BitIter for T {}
