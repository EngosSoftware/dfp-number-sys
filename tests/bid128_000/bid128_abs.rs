use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid128_abs(d128("2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid128_abs(d128("-2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
