use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid128_exp2(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid128_exp2(d128("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+5E-1", bid128_exp2(d128("-1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
