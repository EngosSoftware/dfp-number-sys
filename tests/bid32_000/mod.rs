mod bid32_from_string;
mod bid32_is_nan;
mod bid32_is_normal;
mod bid32_miscellaneous;
mod bid32_to_string;

use super::*;

fn eq(expected: &str, actual: BID32) {
  let mut flags = EXE_CLEAR;
  assert_eq!(expected, bid32_to_string(actual, &mut flags));
  assert_eq!(EXE_CLEAR, flags);
}

fn eqf(expected: u32, actual: u32) {
  assert_eq!(expected, actual);
}

/// Returns a positive subnormal value for testing purposes.
fn bid32_positive_subnormal() -> BID32 {
  BID32::new(0x00000001)
}

/// Returns a negative subnormal value for testing purposes.
fn bid32_negative_subnormal() -> BID32 {
  BID32::new(0x80000001)
}
