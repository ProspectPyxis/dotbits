use crate::{BitManip, BitVec};

#[test]
fn bit_get() {
    assert!(0b01010101u8.bit_get(2).unwrap());
    assert!(!0b01010101u8.bit_get(1).unwrap());
}

#[test]
fn bit_get_err() {
    assert!(0u8.bit_get(10).is_err());
}

#[test]
fn bit_set() {
    assert_eq!(*0b00001000u8.bit_set(0, true).unwrap(), 0b00001001u8);
    assert_eq!(*0b00001100u8.bit_set(2, false).unwrap(), 0b00001000u8);
    assert_eq!(*0b01000000u8.bit_on(5).unwrap(), 0b01100000u8);
    assert_eq!(*0b01010000u8.bit_off(4).unwrap(), 0b01000000u8);
    assert_eq!(*0b00100000u8.bit_off(5).unwrap(), 0);
}

#[test]
fn bit_set_err() {
    assert!(0u8.bit_set(8, true).is_err());
    assert!(0u8.bit_on(9).is_err());
    assert!(0u8.bit_off(10).is_err());
}

#[test]
fn bit_vec_ones() {
    assert_eq!(0b00111000u8.bits().ones(), vec![3, 4, 5]);
    assert_eq!(0b10000000u8.bits().ones(), vec![7]);
    assert_eq!(0b01000010u8.bits().ones(), vec![1, 6]);
}

#[test]
fn bit_vec_zeroes() {
    assert_eq!(0b11001100u8.bits().zeroes(), vec![0, 1, 4, 5]);
    assert_eq!(0b01001111u8.bits().zeroes(), vec![4, 5, 7]);
    assert_eq!(0b00001111u8.bits().zeroes(), vec![4, 5, 6, 7]);
}

#[test]
fn bit_vec_try_into() {
    assert_eq!(128u8.bits().try_into_u8().unwrap(), 128u8);
    assert_eq!(128u16.bits().try_into_u16().unwrap(), 128u16);
    assert_eq!(128u32.bits().try_into_u32().unwrap(), 128u32);
    assert_eq!(128u64.bits().try_into_u64().unwrap(), 128u64);
    assert_eq!(128u128.bits().try_into_u128().unwrap(), 128u128);
}
