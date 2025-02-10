use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  let value = bid128_erf(d128("0"), RM_NEAREST_EVEN, &mut flags);
  eqf(FB_CLEAR, flags);
  eq("+0E-33", value);
  assert!(bid128_is_zero(value));
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+5204998778130465376827466538919645E-34", bid128_erf(d128("0.5"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-5204998778130465376827466538919645E-34", bid128_erf(d128("-0.5"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}
