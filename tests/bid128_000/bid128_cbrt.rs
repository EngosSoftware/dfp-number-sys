use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+30E-1", bid128_cbrt(d128("27.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1912931182772389101199116839548760E-33", bid128_cbrt(d128("7.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
