use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1414213562373095048801688724209698E-33", bid128_sqrt(d128("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
