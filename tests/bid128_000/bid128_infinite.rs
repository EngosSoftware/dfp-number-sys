use super::*;

#[test]
fn _0001() {
  let x = bid128_infinite();
  assert!(bid128_is_infinite(x));
  assert!(!bid128_is_finite(x));
}
