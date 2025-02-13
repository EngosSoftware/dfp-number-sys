use super::*;

#[test]
fn _0001() {
  eq("+1E-4", bid32_quantum(d32("2.3456")));
}

#[test]
fn _0002() {
  eq("+1E-11", bid32_quantum(d32("122.4567E-7")));
}

#[test]
fn _0003() {
  eq("+1E+0", bid32_quantum(d32("122000")));
}
