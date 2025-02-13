use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let value = bid32_erf(d32("0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  eq("+0E+0", value);
  assert!(bid32_is_zero(value));
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+5204999E-7", bid32_erf(d32("0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-5204999E-7", bid32_erf(d32("-0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
