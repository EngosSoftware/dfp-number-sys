use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid128_rem(d128("10"), d128("3"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
