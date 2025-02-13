//! # `bid64_000` bindings
//!
//! ```text
//! bid64 _ 0 0 0
//! ─┬───   ┬ ┬ ┬
//!  │      │ │ │
//!  │      │ │ └── status flags passed as a separate argument
//!  │      │ └── rounding mode passed as a separate argument
//!  │      └── result returned by value
//!  └─────── 64-bit decimal in BID format
//! ```

use crate::{Class, Double, ExcFlags, Float, Long, LongLong, RndMode, Signed, BID128, BID32, BID64};
use libc::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulonglong, c_ushort};
use std::ffi::{CStr, CString};

pub const BID64_MIN: BID64 = BID64(0xF7FB86F26FC0FFFF);
pub const BID64_MAX: BID64 = BID64(0x77FB86F26FC0FFFF);
pub const BID64_ZERO: BID64 = BID64(0x31C0000000000000);
pub const BID64_MINUS_ZERO: BID64 = BID64(0xB1C0000000000000);
pub const BID64_ONE: BID64 = BID64(0x31C0000000000001);
pub const BID64_MINUS_ONE: BID64 = BID64(0xB1C0000000000001);
pub const BID64_TWO: BID64 = BID64(0x31C0000000000002);
pub const BID64_MINUS_TWO: BID64 = BID64(0xB1C0000000000002);
pub const BID64_ONE_TENTH: BID64 = BID64(0x31A0000000000001);
pub const BID64_INF: BID64 = BID64(0x7800000000000000);
pub const BID64_MINUS_INF: BID64 = BID64(0xF800000000000000);
pub const BID64_BILLION: BID64 = BID64(0x31C000003B9ACA00);
pub const BID64_MINUS_BILLION: BID64 = BID64(0xB1C000003B9ACA00);

extern "C" {
  fn __bid64_abs(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_acos(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_acosh(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_add(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_asin(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_asinh(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_atan(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_atan2(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_atanh(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_cbrt(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_copy(x: BID64) -> BID64;
  fn __bid64_copySign(x: BID64, y: BID64) -> BID64;
  fn __bid64_cos(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_cosh(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_class(x: BID64) -> c_int;
  fn __bid64_div(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_exp(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_exp10(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_exp2(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_expm1(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_erf(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_erfc(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_fdim(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_fma(x: BID64, y: BID64, z: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_fmod(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_frexp(x: BID64, exp: *mut c_int) -> BID64;
  fn __bid64_from_int32(x: c_int) -> BID64;
  fn __bid64_from_int64(x: c_longlong, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_from_uint32(x: c_uint) -> BID64;
  fn __bid64_from_uint64(x: c_ulonglong, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_hypot(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_ilogb(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_inf() -> BID64;
  fn __bid64_isCanonical(x: BID64) -> c_int;
  fn __bid64_isFinite(x: BID64) -> c_int;
  fn __bid64_isInf(x: BID64) -> c_int;
  fn __bid64_isNaN(x: BID64) -> c_int;
  fn __bid64_isNormal(x: BID64) -> c_int;
  fn __bid64_isSignaling(x: BID64) -> c_int;
  fn __bid64_isSigned(x: BID64) -> c_int;
  fn __bid64_isSubnormal(x: BID64) -> c_int;
  fn __bid64_isZero(x: BID64) -> c_int;
  fn __bid64_ldexp(x: BID64, n: c_int, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_lgamma(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_llrint(x: BID64, round: c_uint, flags: *mut c_uint) -> c_longlong;
  fn __bid64_llround(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_llquantexp(x: BID64) -> c_longlong;
  fn __bid64_log(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_log10(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_log1p(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_log2(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_logb(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_lrint(x: BID64, round: c_uint, flags: *mut c_uint) -> c_long;
  fn __bid64_lround(x: BID64, flags: *mut c_uint) -> c_long;
  fn __bid64_maxnum(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_maxnum_mag(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_minnum(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_minnum_mag(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_modf(x: BID64, int: *mut BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_mul(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_nan(s: *const c_char) -> BID64;
  fn __bid64_nearbyint(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_nextafter(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_nextdown(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_nexttoward(x: BID64, y: BID128, flags: *mut c_uint) -> BID64;
  fn __bid64_nextup(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_negate(x: BID64) -> BID64;
  fn __bid64_pow(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_quantexp(x: BID64) -> c_int;
  fn __bid64_quantum(x: BID64) -> BID64;
  fn __bid64_quantize(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_quiet_equal(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_greater(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_greater_equal(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_greater_unordered(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_less(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_less_equal(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_less_unordered(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_not_equal(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_not_greater(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_not_less(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_ordered(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_quiet_unordered(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_radix(x: BID64) -> c_int;
  fn __bid64_rem(x: BID64, y: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_round_integral_exact(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_round_integral_nearest_away(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_round_integral_nearest_even(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_round_integral_negative(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_round_integral_positive(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_round_integral_zero(x: BID64, flags: *mut c_uint) -> BID64;
  fn __bid64_sameQuantum(x: BID64, y: BID64) -> c_int;
  fn __bid64_scalbn(x: BID64, n: c_int) -> BID64;
  fn __bid64_scalbln(x: BID64, n: c_longlong) -> BID64;
  fn __bid64_signaling_greater(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_greater_equal(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_greater_unordered(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_less(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_less_equal(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_less_unordered(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_not_greater(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_signaling_not_less(x: BID64, y: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_sin(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_sinh(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_sqrt(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_sub(x: BID64, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_tan(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_tanh(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_tgamma(x: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_to_bid32(x: BID64, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid64_to_bid128(x: BID64, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid64_to_binary32(x: BID64, round: c_uint, flags: *mut c_uint) -> c_float;
  fn __bid64_to_binary64(x: BID64, round: c_uint, flags: *mut c_uint) -> c_double;
  fn __bid64_to_int16_ceil(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_floor(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_int(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_rnint(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_rninta(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_xceil(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_xfloor(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_xint(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_xrnint(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int16_xrninta(x: BID64, flags: *mut c_uint) -> c_short;
  fn __bid64_to_int32_ceil(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_floor(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_int(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_rnint(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_rninta(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_xceil(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_xfloor(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_xint(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_xrnint(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int32_xrninta(x: BID64, flags: *mut c_uint) -> c_int;
  fn __bid64_to_int64_ceil(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_floor(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_int(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_rnint(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_rninta(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_xceil(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_xfloor(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_xint(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_xrnint(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int64_xrninta(x: BID64, flags: *mut c_uint) -> c_longlong;
  fn __bid64_to_int8_ceil(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_floor(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_int(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_rnint(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_rninta(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_xceil(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_xfloor(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_xint(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_xrnint(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_int8_xrninta(x: BID64, flags: *mut c_uint) -> c_char;
  fn __bid64_to_uint16_ceil(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_floor(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_int(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_rnint(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_rninta(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_xceil(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_xfloor(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_xint(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_xrnint(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint16_xrninta(x: BID64, flags: *mut c_uint) -> c_ushort;
  fn __bid64_to_uint32_ceil(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_floor(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_int(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_rnint(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_rninta(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_xceil(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_xfloor(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_xint(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_xrnint(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint32_xrninta(x: BID64, flags: *mut c_uint) -> c_uint;
  fn __bid64_to_uint64_ceil(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_floor(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_int(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_rnint(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_rninta(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_xceil(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_xfloor(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_xint(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_xrnint(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint64_xrninta(x: BID64, flags: *mut c_uint) -> c_ulonglong;
  fn __bid64_to_uint8_ceil(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_floor(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_int(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_rnint(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_rninta(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_xceil(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_xfloor(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_xint(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_xrnint(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_uint8_xrninta(x: BID64, flags: *mut c_uint) -> c_uchar;
  fn __bid64_to_string(s: *mut c_char, x: BID64, flags: *mut c_uint);
  fn __bid64_totalOrder(x: BID64, y: BID64) -> c_int;
  fn __bid64_totalOrderMag(x: BID64, y: BID64) -> c_int;
  fn __bid64dq_add(x: BID64, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qd_add(x: BID128, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qq_add(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64dq_sub(x: BID64, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qd_sub(x: BID128, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qq_sub(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64dq_mul(x: BID64, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qd_mul(x: BID128, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qq_mul(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64dq_div(x: BID64, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qd_div(x: BID128, y: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qq_div(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64ddq_fma(x: BID64, y: BID64, z: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64dqd_fma(x: BID64, y: BID128, z: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64dqq_fma(x: BID64, y: BID128, z: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qdd_fma(x: BID128, y: BID64, z: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qdq_fma(x: BID128, y: BID64, z: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qqd_fma(x: BID128, y: BID128, z: BID64, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64qqq_fma(x: BID128, y: BID128, z: BID128, round: c_uint, flags: *mut c_uint) -> BID64;
}

pub fn bid64_abs(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_abs(x, flags) }
}

/// Returns `arcsin(x)`.
pub fn bid64_acos(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_acos(x, round, flags) }
}

/// Returns `arsinh(x)`.
pub fn bid64_acosh(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_acosh(x, round, flags) }
}

/// Returns a result of decimal floating-point addition.
pub fn bid64_add(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_add(x, y, round, flags) }
}

/// Returns `arcsin(x)`.
pub fn bid64_asin(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_asin(x, round, flags) }
}

/// Returns `arsinh(x)`.
pub fn bid64_asinh(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_asinh(x, round, flags) }
}

/// Returns `arctan(x)`.
pub fn bid64_atan(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_atan(x, round, flags) }
}

/// Returns `arctan2(x, y)`.
pub fn bid64_atan2(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_atan2(x, y, round, flags) }
}

/// Returns `artanh(x)`.
pub fn bid64_atanh(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_atanh(x, round, flags) }
}

/// Returns the cube root of the argument `x`.
pub fn bid64_cbrt(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_cbrt(x, round, flags) }
}

/// Copies a 64-bit decimal floating-point operand to a destination
/// in the same format, with no change.
pub fn bid64_copy(x: BID64) -> BID64 {
  unsafe { __bid64_copy(x) }
}

/// Copies argument `x` to destination in the same format as `x`, but with the sign of `y`.
pub fn bid64_copy_sign(x: BID64, y: BID64) -> BID64 {
  unsafe { __bid64_copySign(x, y) }
}

/// Returns `cos(x)`.
pub fn bid64_cos(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_cos(x, round, flags) }
}

/// Returns `cosh(x)`.
pub fn bid64_cosh(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_cosh(x, round, flags) }
}

/// Returns the class of the specified argument `x`.
pub fn bid64_class(x: BID64) -> Class {
  unsafe { __bid64_class(x) as u32 }.into()
}

/// Returns a result of decimal floating-point division.
pub fn bid64_div(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_div(x, y, round, flags) }
}

/// Returns the result of Gaussian error function for specified `x`.
pub fn bid64_erf(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_erf(x, round, flags) }
}

/// Returns the result of the complementary Gaussian error function for specified `x`: `erfc(x) = 1 - erf(x)`.
pub fn bid64_erfc(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_erfc(x, round, flags) }
}

/// Returns the value of `e` raised to the `x`th power.
pub fn bid64_exp(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_exp(x, round, flags) }
}

/// Returns the `10^x`.
pub fn bid64_exp10(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_exp10(x, round, flags) }
}

/// Returns the `2^x`.
pub fn bid64_exp2(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_exp2(x, round, flags) }
}

/// Returns the `e^x - 1`.
pub fn bid64_expm1(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_expm1(x, round, flags) }
}

/// Returns positive difference between `x` and `y`.
/// Result is x - y if x > y, and +0 is x <= y.
pub fn bid64_fdim(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_fdim(x, y, round, flags) }
}

/// Returns `(x * y) + z` rounded as one ternary operation.
pub fn bid64_fma(x: BID64, y: BID64, z: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_fma(x, y, z, round, flags) }
}

/// Returns the remainder of the division `x/y`.
pub fn bid64_fmod(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_fmod(x, y, flags) }
}

/// If x is not a floating-point number, the results are unspecified (this
/// implementation returns x and *exp = 0). Otherwise, the frexp function
/// returns the value res, such that res has a magnitude in the interval
/// [1/10, 1) or zero, and x = res*2^*exp. If x is zero, both parts of the
/// result are zero. `frexp` does not raise any exceptions.
pub fn bid64_frexp(x: BID64, exp: &mut i32) -> BID64 {
  unsafe { __bid64_frexp(x, exp) }
}

/// Converts 32-bit signed integer to 64-bit decimal floating-point number.
pub fn bid64_from_int32(x: i32) -> BID64 {
  unsafe { __bid64_from_int32(x) }
}

/// Converts 64-bit signed integer to 64-bit decimal floating-point number.
pub fn bid64_from_int64(x: i64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_from_int64(x, round, flags) }
}

/// Converts a number represented as string format (decimal character sequence)
/// to 64-bit decimal floating-point format (binary encoding).
pub fn bid64_from_string(s: &str, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid64_from_string(c_s.as_ptr(), round, flags) }
}

/// Converts 32-bit unsigned integer to 64-bit decimal floating-point number.
pub fn bid64_from_uint32(x: u32) -> BID64 {
  unsafe { __bid64_from_uint32(x) }
}

/// Converts 64-bit unsigned integer to 64-bit decimal floating-point number.
pub fn bid64_from_uint64(x: u64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_from_uint64(x, round, flags) }
}

/// Returns the square root of the squares of two arguments.
pub fn bid64_hypot(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_hypot(x, y, round, flags) }
}

/// Returns the exponent e of x, a signed integral value, determined as though x
/// were represented with infinite range and minimum exponent.
pub fn bid64_ilogb(x: BID64, flags: &mut ExcFlags) -> Signed {
  unsafe { __bid64_ilogb(x, flags) }
}

/// Returns `true` if and only if `x` is canonical number, infinity or NaN.
pub fn bid64_is_canonical(x: BID64) -> bool {
  unsafe { __bid64_isCanonical(x) != 0 }
}

/// Returns `true` if and only if `x` is zero, subnormal or normal (not infinite or NaN).
pub fn bid64_is_finite(x: BID64) -> bool {
  unsafe { __bid64_isFinite(x) != 0 }
}

/// Returns infinite value.
pub fn bid64_infinite() -> BID64 {
  unsafe { __bid64_inf() }
}

/// Returns `true` if x is infinite.
pub fn bid64_is_infinite(x: BID64) -> bool {
  unsafe { __bid64_isInf(x) != 0 }
}

/// Returns `true` if `x` is a `NaN`.
pub fn bid64_is_nan(x: BID64) -> bool {
  unsafe { __bid64_isNaN(x) != 0 }
}

/// Returns `true` if and only if `x` is normal (not zero, subnormal, infinite, or NaN).
pub fn bid64_is_normal(x: BID64) -> bool {
  unsafe { __bid64_isNormal(x) != 0 }
}

/// Returns `true` if and only if x has a negative sign.
pub fn bid64_is_signaling(x: BID64) -> bool {
  unsafe { __bid64_isSignaling(x) != 0 }
}

/// Returns `true` if and only if x has a negative sign.
pub fn bid64_is_signed(x: BID64) -> bool {
  unsafe { __bid64_isSigned(x) != 0 }
}

/// Returns `true` if and only if `x` is subnormal.
pub fn bid64_is_subnormal(x: BID64) -> bool {
  unsafe { __bid64_isSubnormal(x) != 0 }
}

/// Returns `true` if and only if `x` is `+0` or `-0`.
pub fn bid64_is_zero(x: BID64) -> bool {
  unsafe { __bid64_isZero(x) != 0 }
}

/// Returns `x*(10^n)`.
pub fn bid64_ldexp(x: BID64, n: i32, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_ldexp(x, n, round, flags) }
}

/// Returns natural logarithm from gamma function.
pub fn bid64_lgamma(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_lgamma(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value of
/// type [i64], rounding according to the provided rounding direction.
pub fn bid64_llrint(x: BID64, round: RndMode, flags: &mut ExcFlags) -> LongLong {
  unsafe { __bid64_llrint(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value of
/// type [i64], using rounding to nearest-away.
pub fn bid64_llround(x: BID64, flags: &mut ExcFlags) -> LongLong {
  unsafe { __bid64_llround(x, flags) }
}

/// Returns natural logarithm of `x`.
pub fn bid64_log(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_log(x, round, flags) }
}

/// Returns base 10 logarithm of `x`.
pub fn bid64_log10(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_log10(x, round, flags) }
}

/// Returns natural logarithm of `1 + x`.
pub fn bid64_log1p(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_log1p(x, round, flags) }
}

/// Returns base 2 logarithm of `x`.
pub fn bid64_log2(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_log2(x, round, flags) }
}

/// Returns the unbiased radix-independent exponent from `x`.
pub fn bid64_logb(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_logb(x, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value,
/// rounding according to the provided rounding direction.
pub fn bid64_lrint(x: BID64, round: RndMode, flags: &mut ExcFlags) -> Long {
  unsafe { __bid64_lrint(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value,
/// using rounding to nearest-away.
pub fn bid64_lround(x: BID64, flags: &mut ExcFlags) -> Long {
  unsafe { __bid64_lround(x, flags) }
}

/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise, it is either x or y, canonicalized.
pub fn bid64_max_num(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_maxnum(x, y, flags) }
}

pub fn bid64_max_num_mag(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_maxnum_mag(x, y, flags) }
}

/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise, it is either x or y, canonicalized.
pub fn bid64_min_num(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_minnum(x, y, flags) }
}

pub fn bid64_min_num_mag(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_minnum_mag(x, y, flags) }
}

/// Splits the number `x` into integral and fractional part.
pub fn bid64_modf(x: BID64, int: &mut BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_modf(x, int, flags) }
}

/// Returns a result of decimal floating-point multiplication.
pub fn bid64_mul(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_mul(x, y, round, flags) }
}

/// Returns `NaN` with payload.
pub fn bid64_nan(s: &str) -> BID64 {
  let cstring = CString::new(s).unwrap();
  unsafe { __bid64_nan(cstring.as_ptr()) }
}

pub fn bid64_nearbyint(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_nearbyint(x, round, flags) }
}

pub fn bid64_nextafter(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_nextafter(x, y, flags) }
}

pub fn bid64_nextdown(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_nextdown(x, flags) }
}

pub fn bid64_nexttoward(x: BID64, y: BID128, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_nexttoward(x, y, flags) }
}

pub fn bid64_nextup(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_nextup(x, flags) }
}

/// Returns the same value as `x` but with reversed sign.
pub fn bid64_negate(x: BID64) -> BID64 {
  unsafe { __bid64_negate(x) }
}

/// Returns decimal floating-point power.
pub fn bid64_pow(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_pow(x, y, round, flags) }
}

/// Returns the quantum of a finite argument as a signed integer value.
pub fn bid64_quantexp(x: BID64) -> Signed {
  unsafe { __bid64_quantexp(x) }
}

/// Returns the quantum of a finite argument as a signed long long integer value.
pub fn bid64_llquantexp(x: BID64) -> LongLong {
  unsafe { __bid64_llquantexp(x) }
}

/// Returns the quantum of a finite argument.
/// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
pub fn bid64_quantum(x: BID64) -> BID64 {
  unsafe { __bid64_quantum(x) }
}

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
pub fn bid64_quantize(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_quantize(x, y, round, flags) }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid64_quiet_equal(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_equal(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid64_quiet_greater(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_greater(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid64_quiet_greater_equal(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_greater_equal(x, y, flags) != 0 }
}

pub fn bid64_quiet_greater_unordered(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_greater_unordered(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid64_quiet_less(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_less(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid64_quiet_less_equal(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_less_equal(x, y, flags) != 0 }
}

pub fn bid64_quiet_less_unordered(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_less_unordered(x, y, flags) != 0 }
}

pub fn bid64_quiet_not_equal(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_not_equal(x, y, flags) != 0 }
}

pub fn bid64_quiet_not_greater(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_not_greater(x, y, flags) != 0 }
}

pub fn bid64_quiet_not_less(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_not_less(x, y, flags) != 0 }
}

pub fn bid64_quiet_ordered(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_ordered(x, y, flags) != 0 }
}

pub fn bid64_quiet_unordered(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_quiet_unordered(x, y, flags) != 0 }
}

pub fn bid64_radix(x: BID64) -> Signed {
  unsafe { __bid64_radix(x) }
}

/// Returns decimal floating-point remainder.
pub fn bid64_rem(x: BID64, y: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_rem(x, y, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the current rounding mode; signals inexact exceptions.
pub fn bid64_round_integral_exact(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_round_integral_exact(x, round, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-nearest-away** mode; does not signal inexact exceptions.
pub fn bid64_round_integral_nearest_away(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_round_integral_nearest_away(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-nearest-even** mode; does not signal inexact exceptions.
pub fn bid64_round_integral_nearest_even(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_round_integral_nearest_even(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-down** mode; does not signal inexact exceptions.
pub fn bid64_round_integral_negative(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_round_integral_negative(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-up** mode; does not signal inexact exceptions.
pub fn bid64_round_integral_positive(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_round_integral_positive(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-zero** mode; does not signal inexact exceptions.
pub fn bid64_round_integral_zero(x: BID64, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_round_integral_zero(x, flags) }
}

pub fn bid64_same_quantum(x: BID64, y: BID64) -> bool {
  unsafe { __bid64_sameQuantum(x, y) != 0 }
}

/// Returns `x * 10^n` where `n` is of type [i32].
pub fn bid64_scalbn(x: BID64, n: i32) -> BID64 {
  unsafe { __bid64_scalbn(x, n.clamp(-398_i32, 384_i32)) }
}

/// Returns `x * 10^n` where `n` is of type [i64].
pub fn bid64_scalbln(x: BID64, n: i64) -> BID64 {
  unsafe { __bid64_scalbln(x, n.clamp(-398, 384)) }
}

pub fn bid64_signaling_greater(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_greater(x, y, flags) != 0 }
}

pub fn bid64_signaling_greater_equal(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_greater_equal(x, y, flags) != 0 }
}

pub fn bid64_signaling_greater_unordered(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_greater_unordered(x, y, flags) != 0 }
}

pub fn bid64_signaling_less(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_less(x, y, flags) != 0 }
}

pub fn bid64_signaling_less_equal(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_less_equal(x, y, flags) != 0 }
}

pub fn bid64_signaling_less_unordered(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_less_unordered(x, y, flags) != 0 }
}

pub fn bid64_signaling_not_greater(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_not_greater(x, y, flags) != 0 }
}

pub fn bid64_signaling_not_less(x: BID64, y: BID64, flags: &mut ExcFlags) -> bool {
  unsafe { __bid64_signaling_not_less(x, y, flags) != 0 }
}

/// Returns `sin(x)`.
pub fn bid64_sin(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_sin(x, round, flags) }
}

/// Returns `sinh(x)`.
pub fn bid64_sinh(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_sinh(x, round, flags) }
}

/// Returns decimal floating-point square root.
pub fn bid64_sqrt(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_sqrt(x, round, flags) }
}

/// Returns a result of decimal floating-point subtraction.
pub fn bid64_sub(x: BID64, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_sub(x, y, round, flags) }
}

/// Returns `tan(x)`.
pub fn bid64_tan(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_tan(x, round, flags) }
}

/// Returns `tanh(x)`.
pub fn bid64_tanh(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_tanh(x, round, flags) }
}

pub fn bid64_tgamma(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64_tgamma(x, round, flags) }
}

pub fn bid64_to_bid32(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid64_to_bid32(x, round, flags) }
}

pub fn bid64_to_bid128(x: BID64, round: RndMode, flags: &mut ExcFlags) -> BID128 {
  unsafe { __bid64_to_bid128(x, round, flags) }
}

pub fn bid64_to_binary32(x: BID64, round: RndMode, flags: &mut ExcFlags) -> Float {
  unsafe { __bid64_to_binary32(x, round, flags) }
}

pub fn bid64_to_binary64(x: BID64, round: RndMode, flags: &mut ExcFlags) -> Double {
  unsafe { __bid64_to_binary64(x, round, flags) }
}

pub fn bid64_to_int16_ceil(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_ceil(x, flags) }
}

pub fn bid64_to_int16_floor(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_floor(x, flags) }
}

pub fn bid64_to_int16_int(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_int(x, flags) }
}

pub fn bid64_to_int16_rnint(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_rnint(x, flags) }
}

pub fn bid64_to_int16_rninta(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_rninta(x, flags) }
}

pub fn bid64_to_int16_xceil(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_xceil(x, flags) }
}

pub fn bid64_to_int16_xfloor(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_xfloor(x, flags) }
}

pub fn bid64_to_int16_xint(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_xint(x, flags) }
}

pub fn bid64_to_int16_xrnint(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_xrnint(x, flags) }
}

pub fn bid64_to_int16_xrninta(x: BID64, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid64_to_int16_xrninta(x, flags) }
}

pub fn bid64_to_int32_ceil(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_ceil(x, flags) }
}

pub fn bid64_to_int32_floor(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_floor(x, flags) }
}

pub fn bid64_to_int32_int(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_int(x, flags) }
}

pub fn bid64_to_int32_rnint(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_rnint(x, flags) }
}

pub fn bid64_to_int32_rninta(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_rninta(x, flags) }
}

pub fn bid64_to_int32_xceil(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_xceil(x, flags) }
}

pub fn bid64_to_int32_xfloor(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_xfloor(x, flags) }
}

pub fn bid64_to_int32_xint(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_xint(x, flags) }
}

pub fn bid64_to_int32_xrnint(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_xrnint(x, flags) }
}

pub fn bid64_to_int32_xrninta(x: BID64, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid64_to_int32_xrninta(x, flags) }
}

pub fn bid64_to_int64_ceil(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_ceil(x, flags) }
}

pub fn bid64_to_int64_floor(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_floor(x, flags) }
}

pub fn bid64_to_int64_int(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_int(x, flags) }
}

pub fn bid64_to_int64_rnint(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_rnint(x, flags) }
}

pub fn bid64_to_int64_rninta(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_rninta(x, flags) }
}

pub fn bid64_to_int64_xceil(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_xceil(x, flags) }
}

pub fn bid64_to_int64_xfloor(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_xfloor(x, flags) }
}

pub fn bid64_to_int64_xint(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_xint(x, flags) }
}

pub fn bid64_to_int64_xrnint(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_xrnint(x, flags) }
}

pub fn bid64_to_int64_xrninta(x: BID64, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid64_to_int64_xrninta(x, flags) }
}

pub fn bid64_to_int8_ceil(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_ceil(x, flags) }
}

pub fn bid64_to_int8_floor(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_floor(x, flags) }
}

pub fn bid64_to_int8_int(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_int(x, flags) }
}

pub fn bid64_to_int8_rnint(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_rnint(x, flags) }
}

pub fn bid64_to_int8_rninta(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_rninta(x, flags) }
}

pub fn bid64_to_int8_xceil(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_xceil(x, flags) }
}

pub fn bid64_to_int8_xfloor(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_xfloor(x, flags) }
}

pub fn bid64_to_int8_xint(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_xint(x, flags) }
}

pub fn bid64_to_int8_xrnint(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_xrnint(x, flags) }
}

pub fn bid64_to_int8_xrninta(x: BID64, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid64_to_int8_xrninta(x, flags) }
}

pub fn bid64_to_uint16_ceil(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_ceil(x, flags) }
}

pub fn bid64_to_uint16_floor(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_floor(x, flags) }
}

pub fn bid64_to_uint16_int(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_int(x, flags) }
}

pub fn bid64_to_uint16_rnint(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_rnint(x, flags) }
}

pub fn bid64_to_uint16_rninta(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_rninta(x, flags) }
}

pub fn bid64_to_uint16_xceil(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_xceil(x, flags) }
}

pub fn bid64_to_uint16_xfloor(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_xfloor(x, flags) }
}

pub fn bid64_to_uint16_xint(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_xint(x, flags) }
}

pub fn bid64_to_uint16_xrnint(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_xrnint(x, flags) }
}

pub fn bid64_to_uint16_xrninta(x: BID64, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid64_to_uint16_xrninta(x, flags) }
}

pub fn bid64_to_uint32_ceil(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_ceil(x, flags) }
}

pub fn bid64_to_uint32_floor(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_floor(x, flags) }
}

pub fn bid64_to_uint32_int(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_int(x, flags) }
}

pub fn bid64_to_uint32_rnint(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_rnint(x, flags) }
}

pub fn bid64_to_uint32_rninta(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_rninta(x, flags) }
}

pub fn bid64_to_uint32_xceil(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_xceil(x, flags) }
}

pub fn bid64_to_uint32_xfloor(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_xfloor(x, flags) }
}

pub fn bid64_to_uint32_xint(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_xint(x, flags) }
}

pub fn bid64_to_uint32_xrnint(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_xrnint(x, flags) }
}

pub fn bid64_to_uint32_xrninta(x: BID64, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid64_to_uint32_xrninta(x, flags) }
}

pub fn bid64_to_uint64_ceil(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_ceil(x, flags) }
}

pub fn bid64_to_uint64_floor(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_floor(x, flags) }
}

pub fn bid64_to_uint64_int(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_int(x, flags) }
}

pub fn bid64_to_uint64_rnint(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_rnint(x, flags) }
}

pub fn bid64_to_uint64_rninta(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_rninta(x, flags) }
}

pub fn bid64_to_uint64_xceil(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_xceil(x, flags) }
}

pub fn bid64_to_uint64_xfloor(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_xfloor(x, flags) }
}

pub fn bid64_to_uint64_xint(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_xint(x, flags) }
}

pub fn bid64_to_uint64_xrnint(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_xrnint(x, flags) }
}

pub fn bid64_to_uint64_xrninta(x: BID64, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid64_to_uint64_xrninta(x, flags) }
}

pub fn bid64_to_uint8_ceil(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_ceil(x, flags) }
}

pub fn bid64_to_uint8_floor(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_floor(x, flags) }
}

pub fn bid64_to_uint8_int(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_int(x, flags) }
}

pub fn bid64_to_uint8_rnint(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_rnint(x, flags) }
}

pub fn bid64_to_uint8_rninta(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_rninta(x, flags) }
}

pub fn bid64_to_uint8_xceil(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_xceil(x, flags) }
}

pub fn bid64_to_uint8_xfloor(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_xfloor(x, flags) }
}

pub fn bid64_to_uint8_xint(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_xint(x, flags) }
}

pub fn bid64_to_uint8_xrnint(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_xrnint(x, flags) }
}

pub fn bid64_to_uint8_xrninta(x: BID64, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid64_to_uint8_xrninta(x, flags) }
}

/// Converts 64-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
pub fn bid64_to_string(x: BID64, flags: &mut ExcFlags) -> String {
  let mut buf = [0_u8; 1024];
  unsafe {
    __bid64_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
  }
}

pub fn bid64_total_order(x: BID64, y: BID64) -> bool {
  unsafe { __bid64_totalOrder(x, y) != 0 }
}

pub fn bid64_total_order_mag(x: BID64, y: BID64) -> bool {
  unsafe { __bid64_totalOrderMag(x, y) != 0 }
}

pub fn bid64_add_qq(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qq_add(x, y, round, flags) }
}

pub fn bid64_add_dq(x: BID64, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64dq_add(x, y, round, flags) }
}

pub fn bid64_add_qd(x: BID128, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qd_add(x, y, round, flags) }
}

pub fn bid64_sub_qq(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qq_sub(x, y, round, flags) }
}

pub fn bid64_sub_dq(x: BID64, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64dq_sub(x, y, round, flags) }
}

pub fn bid64_sub_qd(x: BID128, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qd_sub(x, y, round, flags) }
}

pub fn bid64_mul_qq(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qq_mul(x, y, round, flags) }
}

pub fn bid64_mul_dq(x: BID64, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64dq_mul(x, y, round, flags) }
}

pub fn bid64_mul_qd(x: BID128, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qd_mul(x, y, round, flags) }
}

pub fn bid64_div_qq(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qq_div(x, y, round, flags) }
}

pub fn bid64_div_dq(x: BID64, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64dq_div(x, y, round, flags) }
}

pub fn bid64_div_qd(x: BID128, y: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qd_div(x, y, round, flags) }
}

pub fn bid64_fma_ddq(x: BID64, y: BID64, z: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64ddq_fma(x, y, z, round, flags) }
}

pub fn bid64_fma_dqd(x: BID64, y: BID128, z: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64dqd_fma(x, y, z, round, flags) }
}

pub fn bid64_fma_dqq(x: BID64, y: BID128, z: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64dqq_fma(x, y, z, round, flags) }
}

pub fn bid64_fma_qdd(x: BID128, y: BID64, z: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qdd_fma(x, y, z, round, flags) }
}

pub fn bid64_fma_qdq(x: BID128, y: BID64, z: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qdq_fma(x, y, z, round, flags) }
}

pub fn bid64_fma_qqd(x: BID128, y: BID128, z: BID64, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qqd_fma(x, y, z, round, flags) }
}

pub fn bid64_fma_qqq(x: BID128, y: BID128, z: BID128, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  unsafe { __bid64qqq_fma(x, y, z, round, flags) }
}
