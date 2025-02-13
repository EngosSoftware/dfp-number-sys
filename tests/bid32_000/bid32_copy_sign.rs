use super::*;

#[test]
fn _0001() {
  eq("+1E+0", bid32_copy_sign(d32("1"), d32("1")));
}

#[test]
fn _0002() {
  eq("-1E+0", bid32_copy_sign(d32("1"), d32("-1")));
}

#[test]
fn _0003() {
  eq("+1E+0", bid32_copy_sign(d32("-1"), d32("1")));
}

#[test]
fn _0004() {
  eq("-1E+0", bid32_copy_sign(d32("-1"), d32("-1")));
}
