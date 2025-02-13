use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid32_fmod(d32("5"), d32("3"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+33E-1", bid32_fmod(d32("3.3"), d32("5.4"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-25E-1", bid32_fmod(d32("-5.2"), d32("2.7"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_fmod(d32("1.0"), d32("0"), &mut flags));
  eqf(EXE_INVALID, flags);
}
