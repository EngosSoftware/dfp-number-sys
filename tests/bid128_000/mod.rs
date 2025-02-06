mod bid128_0000;
mod bid128_abs;
mod bid128_acos;
mod bid128_acosh;
mod bid128_asin;
mod bid128_asinh;
mod bid128_atan;
mod bid128_atan2;
mod bid128_atanh;
mod bid128_cbrt;
mod bid128_constants;
mod bid128_cos;
mod bid128_cosh;
mod bid128_is_finite;
mod bid128_miscellaneous;
mod bid128_sin;
mod bid128_sinh;
mod bid128_tan;
mod bid128_tanh;
mod bid128_to_string;

use super::*;
use dfp_number_sys::bid128_000::*;
use dfp_number_sys::*;

fn eq(expected: &str, actual: BID128) {
  assert_eq!(expected, bid128_quiet_to_string(actual));
}

fn eqf(expected: u32, actual: u32) {
  assert_eq!(expected, actual);
}

fn d128(s: &str) -> BID128 {
  let mut flags = FB_CLEAR;
  let x = bid128_from_string(s, RM_NEAREST_EVEN, &mut flags);
  assert_eq!(FB_CLEAR, flags);
  x
}
