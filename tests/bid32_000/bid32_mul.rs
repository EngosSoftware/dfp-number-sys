use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+10E+0", bid32_mul(d32("2"), d32("5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
