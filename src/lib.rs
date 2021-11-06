//! Utilities for convenient bit manipulation.

#![warn(missing_docs)]

mod bit_manip;

pub use crate::bit_manip::BitManip;

/// A trait that provides additional helper methods on `Vec<bool>`.
pub trait BitVec {
    /// Returns a vector of all positions that are `true`.
    fn ones(&self) -> Vec<usize>;

    /// Returns a vector of all positions that are `false`.
    fn zeroes(&self) -> Vec<usize>;

    /// Attempts to convert into a `u8`. This returns an error if the converted value would
    /// overflow the type.
    fn try_into_u8(&self) -> Result<u8, &'static str>;

    /// Attempts to convert into a `u16`. This returns an error if the converted value would
    /// overflow the type.
    fn try_into_u16(&self) -> Result<u16, &'static str>;

    /// Attempts to convert into a `u32`. This returns an error if the converted value would
    /// overflow the type.
    fn try_into_u32(&self) -> Result<u32, &'static str>;

    /// Attempts to convert into a `u64`. This returns an error if the converted value would
    /// overflow the type.
    fn try_into_u64(&self) -> Result<u64, &'static str>;

    /// Attempts to convert into a `u128`. This returns an error if the converted value would
    /// overflow the type.
    fn try_into_u128(&self) -> Result<u128, &'static str>;

    /// Attempts to convert into a `usize`. This returns an error if the converted value would
    /// overflow the type.
    fn try_into_usize(&self) -> Result<usize, &'static str>;
}

macro_rules! bitvec_try_into {
    ($($i:ident = $t:ty),*) => {$(
        fn $i(&self) -> Result<$t, &'static str> {
            if self.iter().skip(<$t>::BITS as usize).any(|&x| x) {
                return Err("overflow");
            }

            let mut v: $t = 0;
            for (i, b) in self.iter().enumerate() {
                if *b {
                    v |= 1 << i;
                }
            }

            Ok(v)
        }
    )*}
}

impl BitVec for Vec<bool> {
    #[inline]
    fn ones(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();

        for (i, b) in self.iter().enumerate() {
            if *b {
                v.push(i);
            }
        }

        v
    }

    #[inline]
    fn zeroes(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();

        for (i, b) in self.iter().enumerate() {
            if !*b {
                v.push(i);
            }
        }

        v
    }

    bitvec_try_into!(
        try_into_u8 = u8,
        try_into_u16 = u16,
        try_into_u32 = u32,
        try_into_u64 = u64,
        try_into_u128 = u128,
        try_into_usize = usize
    );
}
