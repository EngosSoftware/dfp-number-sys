#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  let (x, y) = (d64("2"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let (x, y) = (d64("2"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let (x, y) = (d64("2"), d64("Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let (x, y) = (d64("2"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0005() {
  let (x, y) = (d64("2"), d64("QNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0006() {
  let (x, y) = (d64("2"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0007() {
  let (x, y) = (d64("3"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let (x, y) = (d64("3"), d64("Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0009() {
  let (x, y) = (d64("3"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0010() {
  let (x, y) = (d64("3"), d64("QNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0011() {
  let (x, y) = (d64("3"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0012() {
  let (x, y) = (d64("Inf"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0013() {
  let (x, y) = (d64("Inf"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0014() {
  let (x, y) = (d64("Inf"), d64("Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0015() {
  let (x, y) = (d64("-Inf"), d64("-Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0016() {
  let (x, y) = (d64("Inf"), d64("-Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(true, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0017() {
  let (x, y) = (d64("Inf"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0018() {
  let (x, y) = (d64("Inf"), d64("QNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0019() {
  let (x, y) = (d64("Inf"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0020() {
  let (x, y) = (d64("NaN"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0021() {
  let (x, y) = (d64("NaN"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0022() {
  let (x, y) = (d64("NaN"), d64("Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0023() {
  let (x, y) = (d64("NaN"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0024() {
  let (x, y) = (d64("NaN"), d64("QNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0025() {
  let (x, y) = (d64("NaN"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0026() {
  let (x, y) = (d64("QNaN"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0027() {
  let (x, y) = (d64("QNaN"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0028() {
  let (x, y) = (d64("QNaN"), d64("Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0029() {
  let (x, y) = (d64("QNaN"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0030() {
  let (x, y) = (d64("QNaN"), d64("QNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0031() {
  let (x, y) = (d64("QNaN"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0032() {
  let (x, y) = (d64("SNaN"), d64("2"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0033() {
  let (x, y) = (d64("SNaN"), d64("3"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0034() {
  let (x, y) = (d64("SNaN"), d64("Inf"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0035() {
  let (x, y) = (d64("SNaN"), d64("NaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0036() {
  let (x, y) = (d64("SNaN"), d64("QNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0037() {
  let (x, y) = (d64("SNaN"), d64("SNaN"));
  let mut flags = EXE_CLEAR;
  assert_eq!(false, bid64_signaling_greater_equal(x, y, &mut flags));
  eqf(EXE_INVALID, flags);
}
