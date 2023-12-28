mod bid128_0000;
mod bid128_abs;
mod bid128_constants;
mod bid128_miscellaneous;
mod bid128_to_string;

use super::*;
use dfp_number_sys::bid128_000::*;
use dfp_number_sys::*;

fn eq(expected: &str, actual: BID128) {
  assert_eq!(expected, bid128_quiet_to_string(actual));
}

fn d128(s: &str) -> BID128 {
  let mut flags = FB_CLEAR;
  let x = bid128_from_string(s, RM_NEAREST_EVEN, &mut flags);
  assert_eq!(FB_CLEAR, flags);
  x
}
