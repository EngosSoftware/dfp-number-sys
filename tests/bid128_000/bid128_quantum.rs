use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  eq("+1E-4", bid128_quantum(d128("2.3456")));
}

#[test]
fn _0002() {
  eq("+1E-7", bid128_quantum(d128("122.4567000")));
}

#[test]
fn _0003() {
  eq("+1E+0", bid128_quantum(d128("122000")));
}
