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
//! [`Vec.reverse()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse) function if
//! you're not running in a `no_std` environment. With primitive integer types, use the built-in
//! `reverse_bits()` function.
//!
//! ## Examples
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
