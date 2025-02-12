use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1570796326794896619231321691639751E-33", bid128_acos(d128("0.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E-6176", bid128_acos(d128("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+3141592653589793238462643383279503E-33", bid128_acos(d128("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_acos(d128("1.1"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_acos(d128("-1.1"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INVALID | EXE_INEXACT);
}
