use super::*;

#[test]
fn _0001() {
  eq("+1E+0", bid64_copy_sign(d64("1"), d64("1")));
}

#[test]
fn _0002() {
  eq("-1E+0", bid64_copy_sign(d64("1"), d64("-1")));
}

#[test]
fn _0003() {
  eq("+1E+0", bid64_copy_sign(d64("-1"), d64("1")));
}

#[test]
fn _0004() {
  eq("-1E+0", bid64_copy_sign(d64("-1"), d64("-1")));
}
