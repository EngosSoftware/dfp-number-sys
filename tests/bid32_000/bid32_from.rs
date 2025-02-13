use super::*;

#[test]
fn _int32() {
  eq("-2147483E+3", bid32_from_int32(i32::MIN, RND_TOWARD_ZERO, flags!()));
  eq("-10E+0", bid32_from_int32(-10, RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid32_from_int32(-1, RND_TOWARD_ZERO, flags!()));
  eq("+0E+0", bid32_from_int32(0, RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_from_int32(1, RND_TOWARD_ZERO, flags!()));
  eq("+10E+0", bid32_from_int32(10, RND_TOWARD_ZERO, flags!()));
  eq("+2147483E+3", bid32_from_int32(i32::MAX, RND_TOWARD_ZERO, flags!()));
}

#[test]
fn _uint32() {
  eq("+0E+0", bid32_from_uint32(0, RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_from_uint32(1, RND_TOWARD_ZERO, flags!()));
  eq("+10E+0", bid32_from_uint32(10, RND_TOWARD_ZERO, flags!()));
  eq("+4294967E+3", bid32_from_uint32(u32::MAX, RND_TOWARD_ZERO, flags!()));
}

#[test]
fn _int64() {
  eq("-9223372E+12", bid32_from_int64(i64::MIN, RND_TOWARD_ZERO, flags!()));
  eq("-10E+0", bid32_from_int64(-10, RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid32_from_int64(-1, RND_TOWARD_ZERO, flags!()));
  eq("+0E+0", bid32_from_int64(0, RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_from_int64(1, RND_TOWARD_ZERO, flags!()));
  eq("+10E+0", bid32_from_int64(10, RND_TOWARD_ZERO, flags!()));
  eq("+9223372E+12", bid32_from_int64(i64::MAX, RND_TOWARD_ZERO, flags!()));
}

#[test]
fn _uint64() {
  eq("+0E+0", bid32_from_uint64(0, RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_from_uint64(1, RND_TOWARD_ZERO, flags!()));
  eq("+10E+0", bid32_from_uint64(10, RND_TOWARD_ZERO, flags!()));
  eq("+1844674E+13", bid32_from_uint64(u64::MAX, RND_TOWARD_ZERO, flags!()));
}
