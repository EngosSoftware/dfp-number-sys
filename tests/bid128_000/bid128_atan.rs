use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid128_atan(d128("0.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+0E-1", result);
  bid128_is_zero(result);
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7853981633974483096156608458198756E-34", bid128_atan(d128("1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-7853981633974483096156608458198756E-34", bid128_atan(d128("-1.0"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1262627255716650569456366021230203E-33", bid128_atan(d128("3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-1262627255716650569456366021230203E-33", bid128_atan(d128("-3.141592654"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+1003884821913039119226123665393729E-33", bid128_atan(d128("1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-1003884821913039119226123665393729E-33", bid128_atan(d128("-1.570796327"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
