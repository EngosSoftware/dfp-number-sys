use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid128_erfc(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+4795001221869534623172533461080355E-34", bid128_erfc(d128("0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+1520499877813046537682746653891965E-33", bid128_erfc(d128("-0.5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
