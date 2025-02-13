use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1414214E-6", bid32_sqrt(d32("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
