use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2962496068407370508673062189341839E-33", bid128_cbrt(d128("26.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1912931182772389101199116839548760E-33", bid128_cbrt(d128("7.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
