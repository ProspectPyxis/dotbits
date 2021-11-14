# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
