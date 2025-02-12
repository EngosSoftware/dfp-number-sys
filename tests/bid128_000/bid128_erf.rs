use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let value = bid128_erf(d128("0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  eq("+0E-33", value);
  assert!(bid128_is_zero(value));
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+5204998778130465376827466538919645E-34", bid128_erf(d128("0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-5204998778130465376827466538919645E-34", bid128_erf(d128("-0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
