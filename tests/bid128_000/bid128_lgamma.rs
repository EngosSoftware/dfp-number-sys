use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid128_lgamma(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid128_lgamma(d128("1.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+1280182748008146961120771787456671E-32", bid128_lgamma(d128("10.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+8600470153764810145109326816703568E-34", bid128_lgamma(d128("-1.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
