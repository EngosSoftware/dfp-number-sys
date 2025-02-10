use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+5E+0", bid128_hypot(d128("3"), d128("4"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E-6176", bid128_hypot(d128("0"), d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
