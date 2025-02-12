use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+5E+0", bid128_hypot(d128("3"), d128("4"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E-6176", bid128_hypot(d128("0"), d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
