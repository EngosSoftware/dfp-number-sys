use super::*;

#[test]
fn _0001() {
  let x = bid32_infinite();
  assert!(bid32_is_infinite(x));
  assert!(!bid32_is_finite(x));
}
