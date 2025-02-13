use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(-308, bid128_ilogb(d128("2.22507E-308"), &mut flags));
  eqf(EXE_CLEAR, flags);
  assert_eq!(1, bid128_ilogb(d128("22.200"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
