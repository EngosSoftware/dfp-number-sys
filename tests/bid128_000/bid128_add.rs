use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+7E+0", bid128_add(d128("2"), d128("5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7E+0", bid128_add_dd(d64("2"), d64("5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+7E+0", bid128_add_dq(d64("2"), d128("5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+7E+0", bid128_add_qd(d128("2"), d64("5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
