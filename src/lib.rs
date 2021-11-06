//! Utilities for convenient bit manipulation.

#![warn(missing_docs)]

/// A trait that provides methods for simple bit manipulation.
pub trait BitManip {
    /// Converts the implementor type into a `Vec<bool>`.
    fn bits(&self) -> Vec<bool>;

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
}

macro_rules! bitmanip_impl {
    ($($t:ty),*) => {$(
        impl BitManip for $t {
            #[inline]
            fn bits(&self) -> Vec<bool> {
                let mut v: Vec<bool> = Vec::new();

                for i in 0..Self::BITS {
                    v.push(self.bit_get(i as usize));
                }

                v
            }

            #[inline]
            fn bit_get(&self, pos: usize) -> bool {
                assert!(pos < Self::BITS as usize);

                *self & (1 << pos) != 0
            }

            #[inline]
            fn bit_set(&mut self, pos: usize, val: bool) -> &mut Self {
                assert!(pos < Self::BITS as usize);

                *self ^= (Self::MIN.wrapping_sub(val.into()) ^ *self) & (1 << pos);

                self
            }
        }
    )*}
}

bitmanip_impl!(u8, u16, u32, u64, u128, usize);
