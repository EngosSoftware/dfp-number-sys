use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+3E+0", bid64_cbrt(d64("27.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1912931182772389E-15", bid64_cbrt(d64("7.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
