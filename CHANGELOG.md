# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Crate is now `#![no_std]` compatible

### Changed
- All methods no longer return `Result` and simply panic on failure
- All `try_into` methods in `BitVec` has been renamed to `into`, and now consumes the vector to follow conventions

### Removed
- Removed methods that are redundant to existing methods
  - For `BitManip`:
    - `bit_set`, `bit_get`, `bit_on`, `bit_off`, `bit_tog`: Simply use existing bitshift operators
    - `bit_rev`: Use `reverse_bits()` instead, functions are completely equivalent
    - `bit_len`: Use `type::BITS` instead, works for all primitive integer types
  - For `BitVec`:
    - `set`, `set_on`, `set_off`, `toggle`: Simply use manual assigning
- `dotbits::Error` no longer exists - functions will now simply panic on failure

## [0.2.0] - 2021-11-14
### Added
- `signed_left_shift` and `signed_right_shift` methods as helpers for signed shifting

### Removed
- `bit_first_one` and `bit_first_zero` method - redundant to existing `trailing_zeros` and `trailing_ones` methods respectively

## [0.1.2] - 2021-11-14
### Added
- `bit_first_one` and `bit_first_zero` methods to reduce excessive vec creation

## [0.1.1] - 2021-11-14
### Added
- `bit_ones` and `bit_zeroes` methods to shortcut getting on/off bits of a number

## [0.1.0] - 2021-11-12
### Added
- `BitManip` trait that adds various direct bit manipulation functions
- `BitVec` trait to add helpers for manipulating a `Vec<bool>`
- `Error` enum to handle all possible errors from this library
