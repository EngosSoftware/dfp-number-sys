use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2346E-3", bid128_quantize(d128("2.3456"), d128("0.001"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
