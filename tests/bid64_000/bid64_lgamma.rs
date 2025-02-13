use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid64_lgamma(d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_lgamma(d64("1.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+1280182748008147E-14", bid64_lgamma(d64("10.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+8600470153764810E-16", bid64_lgamma(d64("-1.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
