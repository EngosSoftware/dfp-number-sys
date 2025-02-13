use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2346E-3", bid64_quantize(d64("2.3456"), d64("0.001"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
