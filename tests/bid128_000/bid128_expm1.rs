use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_expm1(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1718281828459045235360287471352663E-33", bid128_expm1(d128("1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-6321205588285576784044762298385391E-34", bid128_expm1(d128("-1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}
