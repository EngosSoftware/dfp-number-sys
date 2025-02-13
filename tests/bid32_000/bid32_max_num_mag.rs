use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-2256E-3", bid32_max_num_mag(d32("-1.234"), d32("-2.256"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("-1234E-3", bid32_max_num_mag(d32("-1.234"), d32("NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-2256E-3", bid32_max_num_mag(d32("NaN"), d32("-2.256"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_max_num_mag(d32("-1.234"), d32("SNaN"), &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_max_num_mag(d32("SNaN"), d32("-2.256"), &mut flags));
  eqf(EXE_INVALID, flags);
}
