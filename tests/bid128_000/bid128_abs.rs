use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+2E+0", bid128_abs(d128("2"), &mut flags));
  assert_eq!(flags, FB_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+2E+0", bid128_abs(d128("-2"), &mut flags));
  assert_eq!(flags, FB_CLEAR);
}
