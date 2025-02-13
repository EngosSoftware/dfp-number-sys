use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let mut int = BID64_ZERO;
  eq("+56789E-5", bid64_modf(d64("1234.56789"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("+1234E+0", int);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let mut int = BID64_ZERO;
  eq("+NaN", bid64_modf(d64("NaN"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("+NaN", int);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let mut int = BID64_ZERO;
  eq("-0E+369", bid64_modf(d64("-Inf"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("-Inf", int);
}
