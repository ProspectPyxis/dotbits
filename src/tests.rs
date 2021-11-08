use crate::{BitManip, BitVec};

#[test]
fn bit_get() {
    assert!(0b01010101u8.bit_get(2).unwrap());
    assert!(!0b01010101u8.bit_get(1).unwrap());
}

#[test]
fn bit_get_err() {
    assert!(0u8.bit_get(10).is_err());
    assert!(0u16.bit_get(16).is_err());
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
fn bit_tog() {
    assert_eq!(*0b00011100u8.bit_tog(3).unwrap(), 0b00010100u8);
    assert_eq!(
        *0b11001100u8.bit_tog(2).unwrap().bit_tog(3).unwrap(),
        0b11000000u8
    );
}

#[test]
fn bit_tog_err() {
    assert!(0u8.bit_tog(8).is_err());
    assert!(0u32.bit_tog(32).is_err());
}

#[test]
fn bit_rev() {
    assert_eq!(*0b00001111u8.bit_rev(), 0b11110000u8);
    assert_eq!(*0b00110011u8.bit_rev(), 0b11001100u8);
    assert_eq!(*0b10101010u8.bit_rev(), 0b01010101u8);
}

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
    assert_eq!(0b11001100u8.bits().zeroes(), vec![0, 1, 4, 5]);
    assert_eq!(0b01001111u8.bits().zeroes(), vec![4, 5, 7]);
    assert_eq!(0b00001111u8.bits().zeroes(), vec![4, 5, 6, 7]);
}

#[test]
fn bit_vec_set() {
    assert_eq!(
        *0b01010101u8.bits().set(3, true).unwrap(),
        0b01011101u8.bits()
    );
    assert_eq!(
        *0b11001100u8.bits().set(2, false).unwrap(),
        0b11001000u8.bits()
    );
    assert_eq!(*0b00001111u8.bits().set_on(4).unwrap(), 0b00011111u8.bits());
    assert_eq!(
        *0b00001111u8.bits().set_off(3).unwrap(),
        0b00000111u8.bits()
    );
}

#[test]
fn bit_vec_try_into() {
    assert_eq!(
        vec![false, false, false, false, false, false, false, true]
            .try_into_u8()
            .expect("conversion failed"),
        128
    );
    assert_eq!(
        vec![true, false, true, false, false, false, true, true]
            .try_into_u8()
            .expect("conversion failed"),
        197
    );
    assert_eq!(
        vec![false, true, false, true, false, false, true, false]
            .try_into_u8()
            .expect("conversion failed"),
        74
    );
}

#[test]
fn bit_vec_try_into_unequal_bits() {
    assert_eq!(
        128u32.bits().try_into_u8().expect("conversion failed"),
        128u8
    );
    assert_eq!(
        vec![true, true, true, true, false, false, false, false, false, false]
            .try_into_u8()
            .expect("conversion failed"),
        0b00001111u8
    );
    assert_eq!(
        vec![false, true, false, true]
            .try_into_u8()
            .expect("conversion failed"),
        0b1010u8
    );
}

#[test]
fn bit_vec_try_into_error() {
    assert!(u32::MAX.bits().try_into_u8().is_err());
    assert!(u32::MAX.bits().try_into_u16().is_err());
    assert!((u32::MAX as u64 + 1).bits().try_into_u32().is_err());
    assert!(
        vec![false, false, false, false, false, false, false, false, true]
            .try_into_u8()
            .is_err()
    );
}
