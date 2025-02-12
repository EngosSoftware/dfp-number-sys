use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let mut int = BID128_ZERO;
  eq("+56789E-5", bid128_modf(d128("1234.56789"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("+1234E+0", int);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let mut int = BID128_ZERO;
  eq("+NaN", bid128_modf(d128("NaN"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("+NaN", int);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let mut int = BID128_ZERO;
  eq("-0E+6111", bid128_modf(d128("-Inf"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("-Inf", int);
}
