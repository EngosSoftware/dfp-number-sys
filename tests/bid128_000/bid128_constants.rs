use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  eq("+1E+0", BID128_ONE);
}

#[test]
fn _0002() {
  eq("+1E-1", BID128_ONE_TENTH);
}
