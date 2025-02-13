use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2962496068407371E-15", bid64_cbrt(d64("26.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1912931182772389E-15", bid64_cbrt(d64("7.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
