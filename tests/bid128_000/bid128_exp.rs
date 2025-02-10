use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+1E+0", bid128_exp(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+2718281828459045235360287471352662E-33", bid128_exp(d128("1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+1218249396070347343807017595116797E-32", bid128_exp(d128("2.5"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}
