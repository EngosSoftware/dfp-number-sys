use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let mut int = BID32_ZERO;
  eq("+568E-3", bid32_modf(d32("1234.56789"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("+1234E+0", int);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let mut int = BID32_ZERO;
  eq("+NaN", bid32_modf(d32("NaN"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("+NaN", int);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let mut int = BID32_ZERO;
  eq("-0E+90", bid32_modf(d32("-Inf"), &mut int, &mut flags));
  eqf(EXE_CLEAR, flags);
  eq("-Inf", int);
}
