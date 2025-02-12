use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+0E-41", bid128_asin(d128("0.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1570796326794896619231321691639751E-33", bid128_asin(d128("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-1570796326794896619231321691639751E-33", bid128_asin(d128("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_asin(d128("3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_asin(d128("-3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_asin(d128("1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_asin(d128("-1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}
