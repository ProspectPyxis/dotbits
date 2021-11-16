use crate::{BitManip, BitVec};

#[test]
fn bit_vec() {
    assert_eq!(
        0b01010101u8.bits(),
        vec![true, false, true, false, true, false, true, false]
    );
    assert_eq!(
        0b00011001u8.bits(),
        vec![true, false, false, true, true, false, false, false]
    );
    assert_eq!(0u32.bits().len(), 32);
}

#[test]
fn bit_vec_ones() {
    assert_eq!(0b00111000u8.bits().ones(), vec![3, 4, 5]);
    assert_eq!(0b10000000u8.bits().ones(), vec![7]);
    assert_eq!(0b01000010u8.bits().ones(), vec![1, 6]);
}

#[test]
fn bit_vec_zeroes() {
    assert_eq!(0b11001100u8.bits().zeros(), vec![0, 1, 4, 5]);
    assert_eq!(0b01001111u8.bits().zeros(), vec![4, 5, 7]);
    assert_eq!(0b00001111u8.bits().zeros(), vec![4, 5, 6, 7]);
}

#[test]
fn bit_vec_into() {
    assert_eq!(
        vec![false, false, false, false, false, false, false, true].into_u8(),
        128
    );
    assert_eq!(
        vec![true, false, true, false, false, false, true, true].into_u8(),
        197
    );
    assert_eq!(
        vec![false, true, false, true, false, false, true, false].into_u8(),
        74
    );
}

#[test]
fn bit_vec_into_unequal_bits() {
    assert_eq!(128u32.bits().into_u8(), 128u8);
    assert_eq!(
        vec![true, true, true, true, false, false, false, false, false, false].into_u8(),
        0b00001111u8
    );
    assert_eq!(vec![false, true, false, true].into_u8(), 0b1010u8);
}

#[test]
#[should_panic]
fn bit_vec_into_error() {
    vec![false, false, false, false, false, false, false, false, true].into_u8();
}
