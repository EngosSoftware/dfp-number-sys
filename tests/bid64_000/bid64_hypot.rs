use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+5E+0", bid64_hypot(d64("3"), d64("4"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_hypot(d64("0"), d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
