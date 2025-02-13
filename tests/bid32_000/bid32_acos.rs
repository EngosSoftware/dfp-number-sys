use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1570796E-6", bid32_acos(d32("0.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E-101", bid32_acos(d32("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+3141593E-6", bid32_acos(d32("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_acos(d32("1.1"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_acos(d32("-1.1"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}
