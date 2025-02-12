use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-1000000000000000000000000000000001E-33", bid128_nextdown(d128("-1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999999999999999999999999999E-34", bid128_nextdown(d128("1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+5677999999999999999999999999999999E-33", bid128_nextdown(d128("5.678"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-5678000000000000000000000000000001E-33", bid128_nextdown(d128("-5.678"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid128_nextdown(d128("-Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999999999999999999999999999E+6111", bid128_nextdown(d128("+Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_nextdown(d128("+NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("-NaN", bid128_nextdown(d128("-NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_nextdown(d128("SNaN"), &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid128_nextdown(d128("QNaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
