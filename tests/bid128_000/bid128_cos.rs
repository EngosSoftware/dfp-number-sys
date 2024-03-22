use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+1000000000000000000000000000000000E-33", bid128_cos(d128("0.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+5403023058681397174009366074429766E-34", bid128_cos(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+5403023058681397174009366074429766E-34", bid128_cos(d128("-1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("-9999999999999999999158652063945171E-34", bid128_cos(d128("3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("-2051033807686783083588105303506853E-43", bid128_cos(d128("1.570796327"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}
