use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2346E-3", bid32_quantize(d32("2.3456"), d32("0.001"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
