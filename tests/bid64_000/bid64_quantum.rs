use super::*;

#[test]
fn _0001() {
  eq("+1E-4", bid64_quantum(d64("2.3456")));
}

#[test]
fn _0002() {
  eq("+1E-7", bid64_quantum(d64("122.4567000")));
}

#[test]
fn _0003() {
  eq("+1E+0", bid64_quantum(d64("122000")));
}
