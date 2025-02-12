use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid64_atanh(d64("0.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+0E-398", result);
  bid64_is_zero(result);
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let result = bid64_atanh(d64("1.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+Inf", result);
  bid64_is_infinite(result);
  assert_eq!(flags, EXE_ZERO_DIVIDE);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let result = bid64_atanh(d64("-1.0"), RND_TOWARD_ZERO, &mut flags);
  eq("-Inf", result);
  bid64_is_infinite(result);
  assert_eq!(flags, EXE_ZERO_DIVIDE);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+5493061443340548E-16", bid64_atanh(d64("0.5"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-5493061443340548E-16", bid64_atanh(d64("-0.5"), RND_TOWARD_ZERO, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
