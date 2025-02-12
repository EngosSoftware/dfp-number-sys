//! # Common definitions

use crate::bid128_000::bid128_to_string;
use crate::bid32_000::bid32_to_string;
use crate::bid64_000::bid64_to_string;
use libc::{c_double, c_float, c_int, c_long, c_longlong, c_uint, c_ulong, c_ulonglong};
use std::fmt;
use std::fmt::{Debug, Display};

/// A structure representing 128-bit floating-point decimal number.
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct BID128(pub(crate) [u64; 2]);

impl BID128 {
  /// Creates a new [BID128] value from raw data.
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
    let mut flags = EXE_CLEAR;
    write!(f, "{}", bid128_to_string(*self, &mut flags))
  }
}

/// A structure representing 64-bit floating-point decimal number.
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct BID64(pub(crate) u64);

impl BID64 {
  /// Creates a new [BID64] value from raw data.
  pub fn new(d: u64) -> Self {
    BID64(d)
  }
}

impl Debug for BID64 {
  /// Implements [Debug] trait for [BID64].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:016X}]", self.0)
  }
}

impl Display for BID64 {
  /// Implements [Display] trait for [BID64].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut flags = EXE_CLEAR;
    write!(f, "{}", bid64_to_string(*self, &mut flags))
  }
}

/// A structure representing 32-bit floating-point decimal number.
#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct BID32(pub(crate) u32);

impl BID32 {
  /// Creates a new [BID32] value from raw data.
  pub fn new(d: u32) -> Self {
    BID32(d)
  }
}

impl Debug for BID32 {
  /// Implements [Debug] trait for [BID32].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:08X}]", self.0)
  }
}

impl Display for BID32 {
  /// Implements [Display] trait for [BID32].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut flags = EXE_CLEAR;
    write!(f, "{}", bid32_to_string(*self, &mut flags))
  }
}

pub type ExcFlags = c_uint;

pub type RndMode = c_uint;

pub type Signed = c_int;

pub type Unsigned = c_uint;

pub type Long = c_long;

pub type ULong = c_ulong;

pub type LongLong = c_longlong;

pub type ULongLong = c_ulonglong;

pub type Float = c_float;

pub type Double = c_double;

/// Exception flag `Invalid`.
pub const EXE_INVALID: u32 = 1;

/// Exception flag `ZeroDivide`.
pub const EXE_ZERO_DIVIDE: u32 = 4;

/// Exception flag `Overflow`.
pub const EXE_OVERFLOW: u32 = 8;

/// Exception flag `Underflow`.
pub const EXE_UNDERFLOW: u32 = 16;

/// Exception flag `Inexact`.
pub const EXE_INEXACT: u32 = 32;

/// Exception flag `AllClear`.
pub const EXE_CLEAR: u32 = 0;

/// Rounding mode `NearestEven`.
pub const RND_NEAREST_EVEN: u32 = 0;

/// Rounding mode `Downward`.
pub const RND_DOWNWARD: u32 = 1;

/// Rounding mode `Upward`.
pub const RND_UPWARD: u32 = 2;

/// Rounding mode `TowardZero`.
pub const RND_TOWARD_ZERO: u32 = 3;

/// Rounding mode `NearestAway`.
pub const RND_NEAREST_AWAY: u32 = 4;

/// Number classes.
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
  /// Panics if `value` is out of range `[0..9]`.
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
