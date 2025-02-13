use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1414213562373095E-15", bid64_sqrt(d64("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
