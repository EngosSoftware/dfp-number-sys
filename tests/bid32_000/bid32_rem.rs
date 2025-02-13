use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid32_rem(d32("10"), d32("3"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
