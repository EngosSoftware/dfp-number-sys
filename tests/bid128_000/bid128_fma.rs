use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+17E+0", bid128_fma(d128("5"), d128("3"), d128("2"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1542E-2", bid128_fma(d128("3.3"), d128("5.4"), d128("-2.4"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
