use super::*;

#[test]
fn _0001() {
  assert_eq!("+1E+0", bid128_to_string(bid128_from_int32(1), flags!()));
}

#[test]
fn _0002() {
  assert_eq!("-9223372036854775808E+0", bid128_to_string(bid128_from_int64(i64::MIN), flags!()));
}

#[test]
fn _0003() {
  assert_eq!("+9223372036854775807E+0", bid128_to_string(bid128_from_int64(i64::MAX), flags!()));
}
