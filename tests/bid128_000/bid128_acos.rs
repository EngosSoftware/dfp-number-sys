use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+1570796326794896619231321691639751E-33", bid128_acos(d128("0.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E-6176", bid128_acos(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_CLEAR);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+3141592653589793238462643383279503E-33", bid128_acos(d128("-1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_acos(d128("1.1"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INVALID | FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_acos(d128("-1.1"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INVALID | FB_INEXACT);
}
