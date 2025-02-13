use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid64_atan(d64("0.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+0E+0", result);
  bid64_is_zero(result);
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7853981633974483E-16", bid64_atan(d64("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-7853981633974483E-16", bid64_atan(d64("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1262627255716650E-15", bid64_atan(d64("3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-1262627255716650E-15", bid64_atan(d64("-3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+1003884821913039E-15", bid64_atan(d64("1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-1003884821913039E-15", bid64_atan(d64("-1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
