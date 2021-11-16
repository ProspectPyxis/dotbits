use alloc::vec::Vec;

macro_rules! bitmanip_impl {
    ($($t:ty),*) => {$(
        impl BitManip for $t {
            #[inline]
            fn bits(&self) -> Vec<bool> {
                let mut v: Vec<bool> = Vec::with_capacity(Self::BITS as usize);

                for i in 0..Self::BITS {
                    v.push(self & 1 << i != 0);
                }

                v
            }

            #[inline]
            fn bit_ones(&self) -> Vec<u32> {
                let mut v: Vec<u32> = Vec::new();
                let mut looper = *self;

                while looper != 0 {
                    let shift = looper.trailing_zeros();
                    v.push(shift);
                    looper &= !(1 << shift);
                }

                v
            }

            #[inline]
            fn bit_zeros(&self) -> Vec<u32> {
                let mut v: Vec<u32> = Vec::new();
                let mut looper = *self;

                while looper != Self::MAX {
                    let shift = looper.trailing_ones();
                    v.push(shift);
                    looper |= 1 << shift;
                }

                v
            }

            #[inline]
            fn signed_left_shift(&self, rhs: i32) -> Self {
                if rhs.is_negative() { self >> rhs.abs() } else { self << rhs }
            }

            #[inline]
            fn signed_right_shift(&self, rhs: i32) -> Self {
                if rhs.is_negative() { self << rhs.abs() } else { self >> rhs }
            }
        }
    )*}
}

/// A trait that provides utility methods for simple bit manipulation.
pub trait BitManip {
    /// Converts the implementor type into a `Vec<bool>`.
    fn bits(&self) -> Vec<bool>;

    /// Returns a vector of every "on" position in the number.
    fn bit_ones(&self) -> Vec<u32>;

    /// Returns a vector of every "off" position in the number.
    fn bit_zeros(&self) -> Vec<u32>;

    /// Computes `self << rhs` if `rhs` is positive, or `self >> rhs` if `rhs` is negative.
    fn signed_left_shift(&self, rhs: i32) -> Self;

    /// Computes `self >> rhs` if `rhs` is positive, or `self << rhs` if `rhs` is negative.
    fn signed_right_shift(&self, rhs: i32) -> Self;
}

bitmanip_impl!(u8, u16, u32, u64, u128, usize);
