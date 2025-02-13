use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid32_fma(d32("5"), d32("3"), d32("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1542E-2", bid32_fma(d32("3.3"), d32("5.4"), d32("-2.4"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
