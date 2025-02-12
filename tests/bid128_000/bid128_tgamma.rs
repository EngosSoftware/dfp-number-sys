use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid128_tgamma(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid128_tgamma(d128("1.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+3628800000000000000000000000000014E-28", bid128_tgamma(d128("10.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+2363271801207354703064223311121527E-33", bid128_tgamma(d128("-1.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
