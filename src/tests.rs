use crate::BitManip;

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
