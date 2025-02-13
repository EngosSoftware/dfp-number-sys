use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+3000000E-6", bid32_cbrt(d32("27.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1912931E-6", bid32_cbrt(d32("7.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
