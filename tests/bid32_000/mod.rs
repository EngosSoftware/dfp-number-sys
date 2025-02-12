mod bid32_from_string;
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

// /// Returns a positive subnormal value for testing purposes.
// fn bid128_positive_subnormal() -> BID128 {
//   BID128::new(0x07a63158fbd6b32f, 0x0002000000000000)
// }
//
// /// Returns a negative subnormal value for testing purposes.
// fn bid128_negative_subnormal() -> BID128 {
//   BID128::new(0x07a63158fbd6b32f, 0x8002000000000000)
// }
