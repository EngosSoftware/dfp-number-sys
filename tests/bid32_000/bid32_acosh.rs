use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_acosh(d32("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_acosh(d32("0.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid32_acosh(d32("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1316958E-6", bid32_acosh(d32("2.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+5991458E-6", bid32_acosh(d32("200.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
