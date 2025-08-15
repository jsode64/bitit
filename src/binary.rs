use std::{
    mem::size_of,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

/// A pure binary integer (`u8`, `i8`, `u16`, etc.).
pub trait Binary:
    Sized
    + Copy
    + PartialEq
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + AddAssign
    + SubAssign
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + ShlAssign<u32>
    + ShrAssign<u32>
{
    /// The value where all bits are set to zero.
    const ZERO: Self;

    /// The value's one.
    const ONE: Self;

    /// The value where all bits are set to one.
    const FULL: Self;

    /// The value with only the most significant bit set to one.
    const TOP: Self;

    /// The value's maximum bit index (one less than size in bits).
    const MAX_BIT_IDX: u32;

    /// Returns the number of trailing zeros.
    fn trailing_zeros(self) -> u32;

    /// Returns the number of trailing ones.
    fn trailing_ones(self) -> u32;

    /// Returns the number of leading zeros.
    fn leading_zeros(self) -> u32;

    /// Returns the number of leading ones.
    fn leading_ones(self) -> u32;

    /// Returns the value's least significant 0-bit, or `0` if there is none.
    fn lowest_zero(self) -> Self;

    /// Returns the value's least significant 1-bit, or `0` if there is none.
    fn lowest_one(self) -> Self;

    /// Returns the value's most significant 0-bit, or `0` if there is none.
    fn highest_zero(self) -> Self;

    /// Returns the value's most significant 1-bit, or `0` if there is none.
    fn highest_one(self) -> Self;
}

macro_rules! impl_bits {
    ($($t:ident),*) => {
        $(
            impl Binary for $t {
                const ZERO: Self = 0;

                const ONE: Self = 1;

                const FULL: Self = !Self::ZERO;

                const TOP: Self = (Self::FULL >> 1) + 1;

                const MAX_BIT_IDX: u32 = ((size_of::<$t>() * 8) - 1) as _;

                #[inline(always)]
                fn trailing_zeros(self) -> u32 {
                    self.trailing_zeros()
                }

                #[inline(always)]
                fn trailing_ones(self) -> u32 {
                    self.trailing_ones()
                }

                #[inline(always)]
                fn leading_zeros(self) -> u32 {
                    self.leading_zeros()
                }

                #[inline(always)]
                fn leading_ones(self) -> u32 {
                    self.leading_ones()
                }

                #[inline(always)]
                fn lowest_zero(self) -> Self {
                    (1 as Self).unbounded_shl(self.trailing_ones())
                }

                #[inline(always)]
                fn lowest_one(self) -> Self {
                    (1 as Self).unbounded_shl(self.trailing_zeros())
                }

                #[inline(always)]
                fn highest_zero(self) -> Self {
                    Self::TOP.unbounded_shr(self.leading_ones())
                }

                #[inline(always)]
                fn highest_one(self) -> Self {
                    Self::TOP.unbounded_shr(self.leading_zeros())
                }
            }
        )*
    };
}

impl_bits!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
