use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+8E+0", bid32_pow(d32("2"), d32("3"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
