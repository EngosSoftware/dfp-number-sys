use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid64_atan2(d64("0.0"), d64("0.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+0E+0", result);
  bid64_is_zero(result);
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7853981633974483E-16", bid64_atan2(d64("1.0"), d64("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-2356194490192344E-15", bid64_atan2(d64("-1.0"), d64("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-7853981633974483E-16", bid64_atan2(d64("-1.0"), d64("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+2356194490192344E-15", bid64_atan2(d64("1.0"), d64("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
