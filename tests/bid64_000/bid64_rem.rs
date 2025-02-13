use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid64_rem(d64("10"), d64("3"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
