use super::*;

#[test]
fn _0001() {
  let x = bid64_infinite();
  assert!(bid64_is_infinite(x));
  assert!(!bid64_is_finite(x));
}
