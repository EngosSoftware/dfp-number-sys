use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+Inf", bid128_lgamma(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_lgamma(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+1280182748008146961120771787456671E-32", bid128_lgamma(d128("10.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+8600470153764810145109326816703568E-34", bid128_lgamma(d128("-1.5"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}
