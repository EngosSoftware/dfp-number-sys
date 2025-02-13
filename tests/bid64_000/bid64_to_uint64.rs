use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_u64, bid64_to_uint64_ceil(d64("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_floor(d64("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_int(d64("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_rnint(d64("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_rninta(d64("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_u64, bid64_to_uint64_xceil(d64("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_xfloor(d64("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_xint(d64("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_xrnint(d64("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u64, bid64_to_uint64_xrninta(d64("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}
