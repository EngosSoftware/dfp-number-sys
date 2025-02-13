use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+208717E-5", bid32_fdim(d32("5.43557"), d32("3.3484"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid32_fdim(d32("3.3484"), d32("5.43557"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
