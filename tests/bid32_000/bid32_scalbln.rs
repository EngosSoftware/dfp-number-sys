use super::*;

#[test]
fn _0001() {
  let x = bid32_scalbln(d32("2356789100"), -9);
  eq("+2356789E-6", x);
}

#[test]
fn _0002() {
  let x = bid32_scalbln(d32("2356789100"), -9);
  let y = bid32_scalbln(x, 2);
  eq("+2356789E-4", y);
}

#[test]
fn _0003() {
  let x = bid32_scalbln(d32("1"), 96);
  eq("+1000000E+90", x);
}

#[test]
fn _0005() {
  let x = bid32_scalbln(d32("1"), 97);
  eq("+1000000E+90", x);
}

#[test]
fn _0006() {
  let x = bid32_scalbln(d32("1"), -101);
  eq("+1E-101", x);
}

#[test]
fn _0007() {
  let x = bid32_scalbln(d32("1"), -102);
  eq("+1E-101", x);
}
