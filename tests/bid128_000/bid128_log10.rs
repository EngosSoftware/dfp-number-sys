use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid128_log10(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid128_log10(d128("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+3E+0", bid128_log10(d128("1000.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid128_log10(d128("10.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+1372543800759070330613848971932462E-33", bid128_log10(d128("23.58"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid128_log10(d128("+Inf"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
