use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid32_atan2(d32("0.0"), d32("0.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+0E+0", result);
  bid32_is_zero(result);
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7853981E-7", bid32_atan2(d32("1.0"), d32("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-2356194E-6", bid32_atan2(d32("-1.0"), d32("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-7853981E-7", bid32_atan2(d32("-1.0"), d32("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+2356194E-6", bid32_atan2(d32("1.0"), d32("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
