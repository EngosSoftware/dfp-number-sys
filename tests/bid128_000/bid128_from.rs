use super::*;

#[test]
fn _int64() {
  eq("-9223372036854775808E+0", bid128_from_int64(i64::MIN));
  eq("-10E+0", bid128_from_int64(-10));
  eq("-1E+0", bid128_from_int64(-1));
  eq("+0E+0", bid128_from_int64(0));
  eq("+1E+0", bid128_from_int64(1));
  eq("+10E+0", bid128_from_int64(10));
  eq("+9223372036854775807E+0", bid128_from_int64(i64::MAX));
}

#[test]
fn _uint64() {
  eq("+0E+0", bid128_from_uint64(0));
  eq("+1E+0", bid128_from_uint64(1));
  eq("+10E+0", bid128_from_uint64(10));
  eq("+18446744073709551615E+0", bid128_from_uint64(u64::MAX));
}

#[test]
fn _int32() {
  eq("-2147483648E+0", bid128_from_int32(i32::MIN));
  eq("-10E+0", bid128_from_int32(-10));
  eq("-1E+0", bid128_from_int32(-1));
  eq("+0E+0", bid128_from_int32(0));
  eq("+1E+0", bid128_from_int32(1));
  eq("+10E+0", bid128_from_int32(10));
  eq("+2147483647E+0", bid128_from_int32(i32::MAX));
}

#[test]
fn _uint32() {
  eq("+0E+0", bid128_from_uint32(0));
  eq("+1E+0", bid128_from_uint32(1));
  eq("+10E+0", bid128_from_uint32(10));
  eq("+4294967295E+0", bid128_from_uint32(u32::MAX));
}
