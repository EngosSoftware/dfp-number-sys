use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2256E-3", bid64_max_num(d64("1.234"), d64("2.256"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1234E-3", bid64_max_num(d64("1.234"), d64("NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+2256E-3", bid64_max_num(d64("NaN"), d64("2.256"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_max_num(d64("1.234"), d64("SNaN"), &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_max_num(d64("SNaN"), d64("2.256"), &mut flags));
  eqf(EXE_INVALID, flags);
}
