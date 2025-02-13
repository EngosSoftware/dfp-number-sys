#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  let (x, y) = (d64("2"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let (x, y) = (d64("3"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let (x, y) = (d64("2"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let (x, y) = (d64("NaN"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let (x, y) = (d64("3"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let (x, y) = (d64("NaN"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let (x, y) = (d64("SNaN"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0008() {
  let (x, y) = (d64("3"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0009() {
  let (x, y) = (d64("NaN"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_quiet_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}
