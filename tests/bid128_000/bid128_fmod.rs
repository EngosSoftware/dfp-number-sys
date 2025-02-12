use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid128_fmod(d128("5"), d128("3"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+33E-1", bid128_fmod(d128("3.3"), d128("5.4"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-25E-1", bid128_fmod(d128("-5.2"), d128("2.7"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_fmod(d128("1.0"), d128("0"), &mut flags));
  eqf(EXE_INVALID, flags);
}
