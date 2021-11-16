//! Traits that add convenient bit manipulation methods to unsigned integers (`u8`, `u16`, `u32`,
//! `u64`, `u128`, `usize`) and boolean vectors (`Vec<bool>`).
//!
//! ## Endianness
//!
//! Unless stated otherwise, all default implementations assume little-endianness (least
//! significant bit first). For example, position `0` of the value `0b00001111u8` would be equal to
//! `1`/`true`, not `0`/`false`. If big-endianness is desired, you must reverse the value.
//!
//! With a `Vec<bool>`, you can use the built-in
//! [`Vec.reverse()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse) function.
//! With primitive integer types, the [`BitManip`] default implementations have a
//! [`bit_rev()`](BitManip::bit_rev()) method, which will reverse all the bits of the value in
//! place.
//!
//! ## Examples
//!
//! Getting the first "on" bit of a number (roughly equivalent to `number.trailing_zeroes()`):
//!
//! ```
//! use dotbits::BitManip;
//! use dotbits::BitVec;
//!
//! assert_eq!(*0b10110100u8.bits().ones().first().unwrap(), 2);
//! assert_eq!(*128u8.bits().ones().first().unwrap(), 7);
//!
//! // Or alternatively, using .bit_ones():
//!
//! assert_eq!(*0b10110100u8.bit_ones().first().unwrap(), 2);
//! assert_eq!(*128u8.bit_ones().first().unwrap(), 7);
//! ```
//!
//! Bit shifts using negative numbers:
//!
//! ```
//! use dotbits::BitManip;
//!
//! assert_eq!(24u32.signed_left_shift(2), 24u32 << 2);
//! assert_eq!(24u32.signed_left_shift(-2), 24u32 >> 2);
//! assert_eq!(24u32.signed_right_shift(2), 24u32 >> 2);
//! assert_eq!(24u32.signed_right_shift(-2), 24u32 << 2);
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod bit_manip;
mod bit_vec;

pub use crate::bit_manip::BitManip;
pub use crate::bit_vec::BitVec;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;
