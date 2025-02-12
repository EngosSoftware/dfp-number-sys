use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let value = bid64_erf(d64("0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  eq("+0E+0", value);
  assert!(bid64_is_zero(value));
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+5204998778130465E-16", bid64_erf(d64("0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-5204998778130465E-16", bid64_erf(d64("-0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
