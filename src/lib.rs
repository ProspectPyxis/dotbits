//! Utilities for convenient bit manipulation.

#![warn(missing_docs)]

mod bit_manip;
mod bit_vec;
mod error;

pub use crate::bit_manip::BitManip;
pub use crate::bit_vec::BitVec;
pub use crate::error::Error;
