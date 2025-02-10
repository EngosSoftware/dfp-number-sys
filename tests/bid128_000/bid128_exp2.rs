use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+1E+0", bid128_exp2(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+2E+0", bid128_exp2(d128("1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+5E-1", bid128_exp2(d128("-1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
