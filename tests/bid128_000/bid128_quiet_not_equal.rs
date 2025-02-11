#![allow(clippy::bool_assert_comparison)]

use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let (x, y) = (d128("2"), d128("2"));
  let mut flags = FB_CLEAR;
  assert_eq!(false, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let (x, y) = (d128("3"), d128("2"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let (x, y) = (d128("2"), d128("3"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let (x, y) = (d128("NaN"), d128("3"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0005() {
  let (x, y) = (d128("3"), d128("NaN"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0006() {
  let (x, y) = (d128("NaN"), d128("NaN"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0007() {
  let (x, y) = (d128("SNaN"), d128("3"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_INVALID, flags);
}

#[test]
fn _0008() {
  let (x, y) = (d128("3"), d128("SNaN"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_INVALID, flags);
}

#[test]
fn _0009() {
  let (x, y) = (d128("NaN"), d128("SNaN"));
  let mut flags = FB_CLEAR;
  assert_eq!(true, bid128_quiet_not_equal(x, y, &mut flags));
  eqf(FB_INVALID, flags);
}
