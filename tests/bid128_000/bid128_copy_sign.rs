use super::*;

#[test]
fn _0001() {
  eq("+1E+0", bid128_copy_sign(d128("1"), d128("1")));
}

#[test]
fn _0002() {
  eq("-1E+0", bid128_copy_sign(d128("1"), d128("-1")));
}

#[test]
fn _0003() {
  eq("+1E+0", bid128_copy_sign(d128("-1"), d128("1")));
}

#[test]
fn _0004() {
  eq("-1E+0", bid128_copy_sign(d128("-1"), d128("-1")));
}
