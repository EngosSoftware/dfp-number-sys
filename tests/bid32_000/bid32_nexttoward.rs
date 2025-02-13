use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-9999999E-7", bid32_nexttoward(d32("-1"), d128("1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+9999999E-7", bid32_nexttoward(d32("1"), d128("-1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+5678001E-6", bid32_nexttoward(d32("5.678"), d128("6.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-5678001E-6", bid32_nexttoward(d32("-5.678"), d128("-6.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-9999999E+90", bid32_nexttoward(d32("-Inf"), d128("+Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9999999E+90", bid32_nexttoward(d32("+Inf"), d128("-Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_nexttoward(d32("+NaN"), d128("1.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("-NaN", bid32_nexttoward(d32("1.0"), d128("-NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
