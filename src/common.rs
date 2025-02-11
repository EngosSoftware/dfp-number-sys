//! # Common definitions

use crate::bid128_000::bid128_quiet_to_string;
use std::fmt;
use std::fmt::{Debug, Display};

/// A structure representing 128-bit floating-point decimal number.
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct BID128(pub(crate) [u64; 2]);

impl BID128 {
  /// Creates a new BID128 value from raw data.
  pub fn new(lo: u64, hi: u64) -> Self {
    BID128([lo, hi])
  }
}

impl Debug for BID128 {
  /// Implements [Debug] trait for [BID128].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:016X}{:016X}]", self.0[1], self.0[0])
  }
}

impl Display for BID128 {
  /// Implements [Display] trait for [BID128].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", bid128_quiet_to_string(*self))
  }
}

/// A structure representing 64-bit floating-point decimal number.
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct BID64(pub(crate) u64);

/// A structure representing 32-bit floating-point decimal number.
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct BID32(pub(crate) u32);

/// Exception flag `Invalid` as [u32] value.
pub const FB_INVALID: u32 = FlagBits::Invalid as u32;

/// Exception flag `ZeroDivide` as [u32] value.
pub const FB_ZERO_DIVIDE: u32 = FlagBits::ZeroDivide as u32;

/// Exception flag `Overflow` as [u32] value.
pub const FB_OVERFLOW: u32 = FlagBits::Overflow as u32;

/// Exception flag `Underflow` as [u32] value.
pub const FB_UNDERFLOW: u32 = FlagBits::Underflow as u32;

/// Exception flag `Inexact` as [u32] value.
pub const FB_INEXACT: u32 = FlagBits::Inexact as u32;

/// Exception flag `AllClear` as [u32] value.
pub const FB_CLEAR: u32 = FlagBits::AllFlagsClear as u32;

/// Exception flags.
#[repr(u32)]
pub enum FlagBits {
  Invalid = 1,
  ZeroDivide = 4,
  Overflow = 8,
  Underflow = 16,
  Inexact = 32,
  AllFlagsClear = 0,
}

/// Rounding mode `NearestEven` as [u32] value.
pub const RM_NEAREST_EVEN: u32 = RoundingModes::NearestEven as u32;

/// Rounding mode `Downward` as [u32] value.
pub const RM_DOWNWARD: u32 = RoundingModes::Downward as u32;

/// Rounding mode `Upward` as [u32] value.
pub const RM_UPWARD: u32 = RoundingModes::Upward as u32;

/// Rounding mode `TowardZero` as [u32] value.
pub const RM_TOWARD_ZERO: u32 = RoundingModes::TowardZero as u32;

/// Rounding mode `NearestAway` as [u32] value.
pub const RM_NEAREST_AWAY: u32 = RoundingModes::NearestAway as u32;

/// Rounding modes.
#[repr(u32)]
pub enum RoundingModes {
  NearestEven = 0,
  Downward = 1,
  Upward = 2,
  TowardZero = 3,
  NearestAway = 4,
}

/// Value classes.
#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum Class {
  SignalingNaN = 0,
  QuietNaN = 1,
  NegativeInfinity = 2,
  NegativeNormal = 3,
  NegativeSubnormal = 4,
  NegativeZero = 5,
  PositiveZero = 6,
  PositiveSubnormal = 7,
  PositiveNormal = 8,
  PositiveInfinity = 9,
}

impl From<u32> for Class {
  /// Converts integer value into the [Class] enumeration.
  /// Only values 0..9 are valid.
  ///
  /// # Panics
  ///
  /// Panics if `value` is out of range [0..9].
  fn from(value: u32) -> Self {
    match value {
      0 => Class::SignalingNaN,
      1 => Class::QuietNaN,
      2 => Class::NegativeInfinity,
      3 => Class::NegativeNormal,
      4 => Class::NegativeSubnormal,
      5 => Class::NegativeZero,
      6 => Class::PositiveZero,
      7 => Class::PositiveSubnormal,
      8 => Class::PositiveNormal,
      9 => Class::PositiveInfinity,
      _ => unreachable!(),
    }
  }
}
