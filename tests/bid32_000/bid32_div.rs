use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+4E-1", bid32_div(d32("2"), d32("5"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
