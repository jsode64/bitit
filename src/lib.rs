//! # Bitit
//!
//! A library for bitwise iteration over Rust integers.
//!
//! # Examples
//! ```
//! fn main() {
//!     use bitit::BitIter;
//!
//!     let x: u8 = 0b10101100;
//!
//!     // Get ones as singular bits:
//!     print!("1s:\t");
//!     for b in x.ones() {
//!         print!("{b} ");
//!     }
//!     println!();
//!
//!     // Or zeros:
//!     print!("0s:\t");
//!     for b in x.zeros() {
//!         print!("{b} ");
//!     }
//!     println!();
//!
//!     // Or as indices:
//!     print!("1s at:\t");
//!     for i in x.one_indices() {
//!         print!("{i} ");
//!     }
//!     println!();
//!
//!     // Or see them all as bools:
//!     print!("Bits:\t");
//!     for b in x.bits_rev() {
//!         // Using `bits_rev` so the bits are printed in the same order as defined.
//!         print!("{}", b as u8);
//!     }
//!     println!();
//! }
//! ```
//!
//! Output:
//! ```text
//! 1s:     4 8 32 128
//! 0s:     1 2 16 64
//! 1s at:  2 3 5 7
//! Bits:   10101100
//! ```

mod binary;
mod bit_iter;
mod iters;

pub use bit_iter::BitIter;
