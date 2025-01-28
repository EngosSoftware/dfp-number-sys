use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+0E-41", bid128_asin(d128("0.0"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1570796326794896619231321691639751E-33", bid128_asin(d128("1.0"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-1570796326794896619231321691639751E-33", bid128_asin(d128("-1.0"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_asin(d128("3.141592654"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_INVALID | FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_asin(d128("-3.141592654"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_INVALID | FB_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_asin(d128("1.570796327"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_INVALID | FB_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_asin(d128("-1.570796327"), RM_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, FB_INVALID | FB_INEXACT);
}
