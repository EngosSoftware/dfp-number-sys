use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid64_exp(d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+2718281828459045E-15", bid64_exp(d64("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+1218249396070347E-14", bid64_exp(d64("2.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
