use crate::Error;

macro_rules! bitmanip_impl {
    ($($t:ty),*) => {$(
        impl BitManip for $t {
            #[inline]
            fn bits(&self) -> Vec<bool> {
                let mut v: Vec<bool> = Vec::new();

                for i in 0..Self::bit_len() {
                    v.push(self.bit_get(i).unwrap());
                }

                v
            }

            #[inline]
            fn bit_len() -> usize {
                Self::BITS as usize
            }

            #[inline]
            fn bit_get(&self, pos: usize) -> Result<bool, Error> {
                if pos >= Self::bit_len() {
                    return Err(Error::PosOutOfBounds);
                }

                Ok(self & (1 << pos) != 0)
            }

            #[inline]
            fn bit_set(&mut self, pos: usize, val: bool) -> Result<&mut Self, Error> {
                if pos >= Self::bit_len() {
                    return Err(Error::PosOutOfBounds);
                }

                *self ^= (Self::MIN.wrapping_sub(val.into()) ^ *self) & (1 << pos);

                Ok(self)
            }
        }
    )*}
}

/// A trait that provides methods for simple bit manipulation.
pub trait BitManip {
    /// Converts the implementor type into a `Vec<bool>`.
    ///
    /// The resulting vector is formatted in little-endian (least significant bit first). To get a
    /// vector in big-endian, `reverse()` the resulting vector.
    fn bits(&self) -> Vec<bool>;

    /// Gets the length of the implementor type in bits.
    fn bit_len() -> usize;

    /// Gets the bit at a specific position.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// Self::bit_len()`.
    fn bit_get(&self, pos: usize) -> Result<bool, Error>;

    /// Sets the bit at a specific position.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// Self::bit_len()`.
    fn bit_set(&mut self, pos: usize, val: bool) -> Result<&mut Self, Error>;

    /// Equivalent to `bit_set(&mut self, pos: usize, true)`.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// Self::bit_len()`.
    #[inline]
    fn bit_on(&mut self, pos: usize) -> Result<&mut Self, Error> {
        self.bit_set(pos, true)
    }

    /// Equivalent to `bit_set(&mut self, pos: usize, false)`.
    ///
    /// # Errors
    ///
    /// Will return `Err(Error::PosOutOfBounds)` if the index is out of bounds, e.g. `pos >=
    /// Self::bit_len()`.
    #[inline]
    fn bit_off(&mut self, pos: usize) -> Result<&mut Self, Error> {
        self.bit_set(pos, false)
    }
}

bitmanip_impl!(u8, u16, u32, u64, u128, usize);
