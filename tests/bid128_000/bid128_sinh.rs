use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+0E-41", bid128_sinh(d128("0.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1175201193643801456882381850595601E-33", bid128_sinh(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-1175201193643801456882381850595601E-33", bid128_sinh(d128("-1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+1154873936201284599199302147356923E-32", bid128_sinh(d128("3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("-1154873936201284599199302147356923E-32", bid128_sinh(d128("-3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+2301298902821935862436221229051563E-33", bid128_sinh(d128("1.570796327"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  eq("-2301298902821935862436221229051563E-33", bid128_sinh(d128("-1.570796327"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}
