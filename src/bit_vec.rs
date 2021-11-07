use crate::Error;

macro_rules! bitvec_try_into {
    ($($i:ident = $t:ty),*) => {$(
        fn $i(&self) -> Result<$t, Error> {
            if self.iter().skip(<$t>::BITS as usize).any(|&x| x) {
                return Err(Error::PosOutOfBounds);
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

/// A trait that provides additional helper methods on `Vec<bool>`.
pub trait BitVec {
    /// Returns a vector of all positions that are `true`.
    fn ones(&self) -> Vec<usize>;

    /// Returns a vector of all positions that are `false`.
    fn zeroes(&self) -> Vec<usize>;

    /// Sets a particular position in the vector to a specific boolean.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// self.len()`.
    fn set(&mut self, pos: usize, val: bool) -> Result<&mut Self, Error>;

    /// Equivalent to `set(&mut self, pos: usize, true)`.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// self.len()`.
    #[inline]
    fn set_on(&mut self, pos: usize) -> Result<&mut Self, Error> {
        self.set(pos, true)
    }

    /// Equivalent to `set(&mut self, pos: usize, false)`.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// self.len()`.
    #[inline]
    fn set_off(&mut self, pos: usize) -> Result<&mut Self, Error> {
        self.set(pos, false)
    }

    /// Attempts to convert into a `u8`. The vector does not have to be the exact size to convert
    /// successfully.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::ConversionOverflow)` if the resulting value would overflow the
    /// type.
    fn try_into_u8(&self) -> Result<u8, Error>;

    /// Attempts to convert into a `u16`. The vector does not have to be the exact size to convert
    /// successfully.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::ConversionOverflow)` if the resulting value would overflow the
    /// type.
    fn try_into_u16(&self) -> Result<u16, Error>;

    /// Attempts to convert into a `u32`. The vector does not have to be the exact size to convert
    /// successfully.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::ConversionOverflow)` if the resulting value would overflow the
    /// type.
    fn try_into_u32(&self) -> Result<u32, Error>;

    /// Attempts to convert into a `u64`. The vector does not have to be the exact size to convert
    /// successfully.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::ConversionOverflow)` if the resulting value would overflow the
    /// type.
    fn try_into_u64(&self) -> Result<u64, Error>;

    /// Attempts to convert into a `u128`. The vector does not have to be the exact size to convert
    /// successfully.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::ConversionOverflow)` if the resulting value would overflow the
    /// type.
    fn try_into_u128(&self) -> Result<u128, Error>;

    /// Attempts to convert into a `usize`. The vector does not have to be the exact size to convert
    /// successfully.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::ConversionOverflow)` if the resulting value would overflow the
    /// type.
    fn try_into_usize(&self) -> Result<usize, Error>;
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

    #[inline]
    fn set(&mut self, pos: usize, val: bool) -> Result<&mut Self, Error> {
        if pos >= self.len() {
            return Err(Error::PosOutOfBounds);
        }

        self[pos] = val;
        Ok(self)
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
