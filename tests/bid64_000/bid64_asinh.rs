use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+0E-398", bid64_asinh(d64("0.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+8813735870195430E-16", bid64_asinh(d64("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-8813735870195430E-16", bid64_asinh(d64("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1862295743435269E-15", bid64_asinh(d64("3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-1862295743435269E-15", bid64_asinh(d64("-3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+1233403117621363E-15", bid64_asinh(d64("1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-1233403117621363E-15", bid64_asinh(d64("-1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
