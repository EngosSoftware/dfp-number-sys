use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+8E+0", bid64_pow(d64("2"), d64("3"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
