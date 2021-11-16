use alloc::vec::Vec;

macro_rules! bitvec_into {
    ($($i:ident = $t:ty),*) => {$(
        #[inline]
        fn $i(self) -> $t {
            self.iter().enumerate().fold(0, |acc, (i, &x)| acc | (<$t>::from(x) << i))
        }
    )*}
}

/// A trait that provides additional helper methods on `Vec<bool>`.
pub trait BitVec {
    /// Returns a vector of all positions that are `true`.
    fn ones(&self) -> Vec<usize>;

    /// Returns a vector of all positions that are `false`.
    fn zeroes(&self) -> Vec<usize>;

    /// Trims any trailing `false` values from the vector.
    fn trim(&mut self) -> &mut Self;

    /// Attempt to convert the vector into a `u8`. The vector does not have to be the exact size of
    /// the type to convert successfully.
    ///
    /// # Errors
    ///
    /// Will retun an `Err` with the value [`Error::ConversionOverflow`] if the resulting value
    /// would overflow the type.
    fn into_u8(self) -> u8;

    /// Attempt to convert the vector into a `u16`. The vector does not have to be the exact size of
    /// the type to convert successfully.
    ///
    /// # Errors
    ///
    /// Will retun an `Err` with the value [`Error::ConversionOverflow`] if the resulting value
    /// would overflow the type.
    fn into_u16(self) -> u16;

    /// Attempt to convert the vector into a `u32`. The vector does not have to be the exact size of
    /// the type to convert successfully.
    ///
    /// # Errors
    ///
    /// Will retun an `Err` with the value [`Error::ConversionOverflow`] if the resulting value
    /// would overflow the type.
    fn into_u32(self) -> u32;

    /// Attempt to convert the vector into a `u64`. The vector does not have to be the exact size of
    /// the type to convert successfully.
    ///
    /// # Errors
    ///
    /// Will retun an `Err` with the value [`Error::ConversionOverflow`] if the resulting value
    /// would overflow the type.
    fn into_u64(self) -> u64;

    /// Attempt to convert the vector into a `u128`. The vector does not have to be the exact size of
    /// the type to convert successfully.
    ///
    /// # Errors
    ///
    /// Will retun an `Err` with the value [`Error::ConversionOverflow`] if the resulting value
    /// would overflow the type.
    fn into_u128(self) -> u128;

    /// Attempt to convert the vector into a `usize`. The vector does not have to be the exact size of
    /// the type to convert successfully.
    ///
    /// # Errors
    ///
    /// Will retun an `Err` with the value [`Error::ConversionOverflow`] if the resulting value
    /// would overflow the type.
    fn into_usize(self) -> usize;
}

impl BitVec for Vec<bool> {
    #[inline]
    fn ones(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter(|(_, &x)| x)
            .map(|(i, _)| i)
            .collect()
    }

    #[inline]
    fn zeroes(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter(|(_, &x)| !x)
            .map(|(i, _)| i)
            .collect()
    }

    #[inline]
    fn trim(&mut self) -> &mut Self {
        while !*self.last().unwrap_or(&true) {
            self.pop();
        }

        self
    }

    bitvec_into!(
        into_u8 = u8,
        into_u16 = u16,
        into_u32 = u32,
        into_u64 = u64,
        into_u128 = u128,
        into_usize = usize
    );
}
