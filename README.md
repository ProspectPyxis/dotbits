# dotbits

> *unsigned int <-> Vec\<bool\>, plus helper functions*

dotbits is a rust library that simplifies bit manipulation. Its primary feature is the `.bits()` function, which converts any unsigned integer type (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`) into a `Vec<bool>`. It also adds certain helper functions over `Vec<bool>`.

**WARNING:** This crate is currently in the rapid iteration phase, and **should not be considered stable.** Certain methods may get added or removed very quickly between versions - use this crate at your own risk!

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
dotbits = "0.3"
```

## Examples

Getting the first "on" bit in a `u8`:

```rust
use dotbits::BitManip;
use dotbits::BitVec;

assert_eq!(*0b10110100u8.bits().ones().first().unwrap(), 2);
assert_eq!(*128u8.bits().ones().first().unwrap(), 7);
```

## License

dotbits is distributed under the terms of both the [MIT license](LICENSE_MIT) and the [Apache License (Version 2.0)](LICENSE_APACHE).
