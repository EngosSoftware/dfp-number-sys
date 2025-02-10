use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+1E+0", bid128_erfc(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+4795001221869534623172533461080355E-34", bid128_erfc(d128("0.5"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+1520499877813046537682746653891965E-33", bid128_erfc(d128("-0.5"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}
