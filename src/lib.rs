//! Utilities for convenient bit manipulation.

#![warn(missing_docs)]

/// A trait that provides methods for simple bit manipulation.
pub trait BitManip {
    /// Converts the implementor type into a `Vec<bool>`.
    ///
    /// The resulting vector is formatted in little-endian. To get the vector in big-endian,
    /// reverse the array.
    fn bits(&self) -> Vec<bool>;

    /// Gets the length of the implementor type in bits.
    fn bit_len() -> usize;

    /// Gets the bit at a specific position.
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds, e.g. `pos >= Self::bit_len()`.
    fn bit_get(&self, pos: usize) -> bool;

    /// Sets the bit at a specific position.
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds, e.g. `pos >= Self::bit_len()`.
    fn bit_set(&mut self, pos: usize, val: bool) -> &mut Self;

    /// Equivalent to `bit_set(&mut self, pos: usize, true)`.
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds, e.g. `pos >= Self::bit_len()`.
    #[inline]
    fn bit_on(&mut self, pos: usize) -> &mut Self {
        self.bit_set(pos, true)
    }

    /// Equivalent to `bit_set(&mut self, pos: usize, false)`.
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds, e.g. `pos >= Self::bit_len()`.
    #[inline]
    fn bit_off(&mut self, pos: usize) -> &mut Self {
        self.bit_set(pos, false)
    }
}

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

macro_rules! bitmanip_impl {
    ($($t:ty),*) => {$(
        impl BitManip for $t {
            #[inline]
            fn bits(&self) -> Vec<bool> {
                let mut v: Vec<bool> = Vec::new();

                for i in 0..Self::bit_len() {
                    v.push(self.bit_get(i));
                }

                v
            }

            #[inline]
            fn bit_len() -> usize {
                Self::BITS as usize
            }

            #[inline]
            fn bit_get(&self, pos: usize) -> bool {
                assert!(pos < Self::bit_len());

                *self & (1 << pos) != 0
            }

            #[inline]
            fn bit_set(&mut self, pos: usize, val: bool) -> &mut Self {
                assert!(pos < Self::bit_len());

                *self ^= (Self::MIN.wrapping_sub(val.into()) ^ *self) & (1 << pos);

                self
            }
        }
    )*}
}

bitmanip_impl!(u8, u16, u32, u64, u128, usize);
