use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_i8, bid128_to_int8_ceil(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_ceil(d128("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_floor(d128("1.2"), &mut flags));
  assert_eq!(-2_i8, bid128_to_int8_floor(d128("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_int(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_int(d128("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_rnint(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_rnint(d128("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_rninta(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_rninta(d128("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_i8, bid128_to_int8_xceil(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_xceil(d128("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_xfloor(d128("1.2"), &mut flags));
  assert_eq!(-2_i8, bid128_to_int8_xfloor(d128("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_xint(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_xint(d128("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_xrnint(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_xrnint(d128("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i8, bid128_to_int8_xrninta(d128("1.2"), &mut flags));
  assert_eq!(-1_i8, bid128_to_int8_xrninta(d128("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}
