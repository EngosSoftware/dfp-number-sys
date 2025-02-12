use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1234E-3", bid128_min_num(d128("1.234"), d128("2.256"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1234E-3", bid128_min_num(d128("1.234"), d128("NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+2256E-3", bid128_min_num(d128("NaN"), d128("2.256"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_min_num(d128("1.234"), d128("SNaN"), &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_min_num(d128("SNaN"), d128("2.256"), &mut flags));
  eqf(EXE_INVALID, flags);
}
