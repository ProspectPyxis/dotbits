macro_rules! bitvec_into {
    ($($i:ident = $t:ty),*) => {$(
        #[inline]
        fn $i(self) -> $t {
            let mut v: $t = 0;
            for (i, b) in self.iter().enumerate() {
                if *b {
                    v |= 1 << i;
                }
            }

            v
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
    /// If the index would be out of bounds, the vector is resized to the appropriate size before
    /// the operation is performed.
    fn set(&mut self, pos: usize, val: bool) -> &mut Self;

    /// Equivalent to `set(&mut self, pos: usize, true)`.
    ///
    /// If the index would be out of bounds, the vector is resized to the appropriate size before
    /// the operation is performed.
    #[inline]
    fn set_on(&mut self, pos: usize) -> &mut Self {
        self.set(pos, true)
    }

    /// Equivalent to `set(&mut self, pos: usize, false)`.
    ///
    /// If the index would be out of bounds, the vector is resized to the appropriate size before
    /// the operation is performed.
    #[inline]
    fn set_off(&mut self, pos: usize) -> &mut Self {
        self.set(pos, false)
    }

    /// Trims any trailing `false` values from the vector.
    fn trim(&mut self) -> &mut Self;

    /// Toggles a particular position in the vector.
    ///
    /// If the index would be out of bounds, the vector is resized to the appropriate size before
    /// the operation is performed.
    fn toggle(&mut self, pos: usize) -> &mut Self;

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
    fn set(&mut self, pos: usize, val: bool) -> &mut Self {
        if pos >= self.len() {
            self.resize(pos, false);
        }

        self[pos] = val;
        self
    }

    #[inline]
    fn toggle(&mut self, pos: usize) -> &mut Self {
        if pos >= self.len() {
            self.resize(pos, false);
        }

        self[pos] = !self[pos];
        self
    }

    #[inline]
    fn trim(&mut self) -> &mut Self {
        for i in (0..self.len()).rev() {
            if *self.get(i).unwrap() {
                break;
            }
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
