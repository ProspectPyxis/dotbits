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
            fn bit_ones(&self) -> Vec<usize> {
                let mut v: Vec<usize> = Vec::new();

                for i in 0..Self::bit_len() {
                    if self.bit_get(i).unwrap() {
                        v.push(i);
                    }
                }

                v
            }

            #[inline]
            fn bit_zeroes(&self) -> Vec<usize> {
                let mut v: Vec<usize> = Vec::new();

                for i in 0..Self::bit_len() {
                    if !self.bit_get(i).unwrap() {
                        v.push(i);
                    }
                }

                v
            }

            #[inline]
            fn bit_first_one(&self) -> Option<usize> {
                for i in 0..Self::bit_len() {
                    if self.bit_get(i).unwrap() {
                        return Some(i);
                    }
                }

                None
            }

            #[inline]
            fn bit_first_zero(&self) -> Option<usize> {
                for i in 0..Self::bit_len() {
                    if !self.bit_get(i).unwrap() {
                        return Some(i);
                    }
                }

                None
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

            #[inline]
            fn bit_tog(&mut self, pos: usize) -> Result<&mut Self, Error> {
                if pos >= Self::bit_len() {
                    return Err(Error::PosOutOfBounds);
                }

                *self ^= 1 << pos;
                Ok(self)
            }

            #[inline]
            fn bit_rev(&mut self) -> &mut Self {
                let original = *self;
                for (i, j) in (0..Self::bit_len()).rev().zip(0..Self::bit_len()) {
                    self.bit_set(i, original.bit_get(j).unwrap()).expect("should never fail");
                }

                self
            }
        }
    )*}
}

/// A trait that provides methods for simple bit manipulation.
pub trait BitManip {
    /// Converts the implementor type into a `Vec<bool>`.
    fn bits(&self) -> Vec<bool>;

    /// Returns a vector of every "on" position in the number. Functionally equivalent to
    /// `Self.bits().ones()` - it's recommended to use this over function chaining, as this avoids
    /// needing to create two vectors and is thus faster.
    fn bit_ones(&self) -> Vec<usize>;

    /// Returns a vector of every "off" position in the number. Functionally equivalent to
    /// `Self.bits().zeroes()` - it's recommended to use this over function chaining, as this
    /// avoids needing to create two vectors and is thus faster.
    fn bit_zeroes(&self) -> Vec<usize>;

    /// Returns the position of the first "on" bit in the number, or `None` if no bits are on.
    /// Equivalent to `Self.bits().ones().first()`.
    fn bit_first_one(&self) -> Option<usize>;

    /// Returns the position of the first "off" bit in the number, or `None` if no bits are on.
    /// Equivalent to `Self.bits().zeroes().first()`.
    fn bit_first_zero(&self) -> Option<usize>;

    /// Gets the length of the implementor type in bits.
    fn bit_len() -> usize;

    /// Gets the bit at a specific position.
    ///
    /// # Errors
    ///
    /// Will return an `Err` with the value [`Error::PosOutOfBounds`] if the index is out of
    /// bounds, e.g: `pos >= Self::bit_len()`.
    fn bit_get(&self, pos: usize) -> Result<bool, Error>;

    /// Sets the bit at a specific position.
    ///
    /// # Errors
    ///
    /// Will return an `Err` with the value [`Error::PosOutOfBounds`] if the index is out of
    /// bounds, e.g: `pos >= Self::bit_len()`.
    fn bit_set(&mut self, pos: usize, val: bool) -> Result<&mut Self, Error>;

    /// Equivalent to `bit_set(&mut self, pos: usize, true)`.
    ///
    /// # Errors
    ///
    /// Will return an `Err` with the value [`Error::PosOutOfBounds`] if the index is out of
    /// bounds, e.g: `pos >= Self::bit_len()`.
    #[inline]
    fn bit_on(&mut self, pos: usize) -> Result<&mut Self, Error> {
        self.bit_set(pos, true)
    }

    /// Equivalent to `bit_set(&mut self, pos: usize, false)`.
    ///
    /// # Errors
    ///
    /// Will return an `Err` with the value [`Error::PosOutOfBounds`] if the index is out of
    /// bounds, e.g: `pos >= Self::bit_len()`.
    #[inline]
    fn bit_off(&mut self, pos: usize) -> Result<&mut Self, Error> {
        self.bit_set(pos, false)
    }

    /// Toggles the bit at a specific position.
    ///
    /// # Errors
    ///
    /// Will return an `Err` with the value [`Error::PosOutOfBounds`] if the index is out of
    /// bounds, e.g: `pos >= Self::bit_len()`.
    fn bit_tog(&mut self, pos: usize) -> Result<&mut Self, Error>;

    /// Reverses all the bits of the implementor type in place.
    ///
    /// Note that this method does not ignore trailing zeroes in the value's bit representation -
    /// for example, `0b1100u8.bit_rev()` would be equal to `0b00110000u8`, not `0b00000011u8`.
    fn bit_rev(&mut self) -> &mut Self;
}

bitmanip_impl!(u8, u16, u32, u64, u128, usize);
