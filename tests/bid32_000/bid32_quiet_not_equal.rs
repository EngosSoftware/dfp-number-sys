#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  let (x, y) = (d32("2"), d32("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let (x, y) = (d32("3"), d32("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let (x, y) = (d32("2"), d32("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let (x, y) = (d32("NaN"), d32("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let (x, y) = (d32("3"), d32("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let (x, y) = (d32("NaN"), d32("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let (x, y) = (d32("SNaN"), d32("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0008() {
  let (x, y) = (d32("3"), d32("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0009() {
  let (x, y) = (d32("NaN"), d32("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid32_quiet_not_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}
