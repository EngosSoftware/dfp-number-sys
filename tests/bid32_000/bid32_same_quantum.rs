#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  let (x, y) = (d32("2"), d32("2"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0002() {
  let (x, y) = (d32("2.1"), d32("3"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0003() {
  let (x, y) = (d32("2"), d32("Inf"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0004() {
  let (x, y) = (d32("2"), d32("NaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0005() {
  let (x, y) = (d32("2"), d32("QNaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0006() {
  let (x, y) = (d32("2"), d32("SNaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0007() {
  let (x, y) = (d32("3"), d32("2"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0008() {
  let (x, y) = (d32("3"), d32("Inf"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0009() {
  let (x, y) = (d32("3"), d32("NaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0010() {
  let (x, y) = (d32("3"), d32("QNaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0011() {
  let (x, y) = (d32("3"), d32("SNaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0012() {
  let (x, y) = (d32("Inf"), d32("2"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0013() {
  let (x, y) = (d32("Inf"), d32("3"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0014() {
  let (x, y) = (d32("Inf"), d32("Inf"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0015() {
  let (x, y) = (d32("Inf"), d32("NaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0016() {
  let (x, y) = (d32("Inf"), d32("QNaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0017() {
  let (x, y) = (d32("Inf"), d32("SNaN"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0018() {
  let (x, y) = (d32("NaN"), d32("2"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0019() {
  let (x, y) = (d32("NaN"), d32("3"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0020() {
  let (x, y) = (d32("NaN"), d32("Inf"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0021() {
  let (x, y) = (d32("NaN"), d32("NaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0022() {
  let (x, y) = (d32("NaN"), d32("QNaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0023() {
  let (x, y) = (d32("NaN"), d32("SNaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0024() {
  let (x, y) = (d32("QNaN"), d32("2"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0025() {
  let (x, y) = (d32("QNaN"), d32("3"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0026() {
  let (x, y) = (d32("QNaN"), d32("Inf"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0027() {
  let (x, y) = (d32("QNaN"), d32("NaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0028() {
  let (x, y) = (d32("QNaN"), d32("QNaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0029() {
  let (x, y) = (d32("QNaN"), d32("SNaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0030() {
  let (x, y) = (d32("SNaN"), d32("2"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0031() {
  let (x, y) = (d32("SNaN"), d32("3"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0032() {
  let (x, y) = (d32("SNaN"), d32("Inf"));
  assert_eq!(false, bid32_same_quantum(x, y));
}

#[test]
fn _0033() {
  let (x, y) = (d32("SNaN"), d32("NaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0034() {
  let (x, y) = (d32("SNaN"), d32("QNaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}

#[test]
fn _0035() {
  let (x, y) = (d32("SNaN"), d32("SNaN"));
  assert_eq!(true, bid32_same_quantum(x, y));
}
