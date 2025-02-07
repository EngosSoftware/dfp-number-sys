use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+30E-1", bid128_cbrt(d128("27.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1912931182772389101199116839548760E-33", bid128_cbrt(d128("7.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}
