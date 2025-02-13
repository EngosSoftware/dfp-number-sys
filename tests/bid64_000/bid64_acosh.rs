use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_acosh(d64("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_acosh(d64("0.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_acosh(d64("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1316957896924817E-15", bid64_acosh(d64("2.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+5991458297049387E-15", bid64_acosh(d64("200.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
