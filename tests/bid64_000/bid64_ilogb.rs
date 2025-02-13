use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(-12, bid64_ilogb(d64("2.22507E-12"), &mut flags));
  eqf(EXE_CLEAR, flags);
  assert_eq!(1, bid64_ilogb(d64("22.200"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
