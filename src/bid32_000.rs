//! # `bid32_000` bindings
//!
//! ```text
//! bid32 _ 0 0 0
//! ─┬───   ┬ ┬ ┬
//!  │      │ │ │
//!  │      │ │ └── status flags passed as a separate argument
//!  │      │ └── rounding mode passed as a separate argument
//!  │      └── result returned by value
//!  └─────── 32-bit decimal in BID format
//! ```

use crate::{Class, Double, ExcFlags, Float, Long, LongLong, RndMode, Signed, BID128, BID32};
use libc::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulonglong, c_ushort};
use std::ffi::{CStr, CString};

pub const BID32_ZERO: BID32 = BID32(0x32800000);
pub const BID32_MINUS_ZERO: BID32 = BID32(0xB2800000);
pub const BID32_ONE: BID32 = BID32(0x32800001);
pub const BID32_MINUS_ONE: BID32 = BID32(0xB2800001);
pub const BID32_TWO: BID32 = BID32(0x32800002);
pub const BID32_MINUS_TWO: BID32 = BID32(0xB2800002);
pub const BID32_ONE_TENTH: BID32 = BID32(0x32000001);
pub const BID32_INF: BID32 = BID32(0x78000000);
pub const BID32_MINUS_INF: BID32 = BID32(0xF8000000);
pub const BID32_BILLION: BID32 = BID32(0x340F4240);
pub const BID32_MINUS_BILLION: BID32 = BID32(0xB40F4240);

extern "C" {
  fn __bid32_abs(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_acos(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_acosh(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_add(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_asin(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_asinh(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_atan(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_atan2(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_atanh(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_cbrt(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_copy(x: BID32) -> BID32;
  fn __bid32_copySign(x: BID32, y: BID32) -> BID32;
  fn __bid32_cos(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_cosh(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_class(x: BID32) -> c_int;
  fn __bid32_div(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_exp(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_exp10(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_exp2(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_expm1(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_erf(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_erfc(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_fdim(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_fma(x: BID32, y: BID32, z: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_fmod(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_frexp(x: BID32, exp: *mut c_int) -> BID32;
  fn __bid32_from_int32(x: c_int, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_from_int64(x: c_longlong, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_from_uint32(x: c_uint, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_from_uint64(x: c_ulonglong, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_hypot(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_ilogb(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_inf() -> BID32;
  fn __bid32_isCanonical(x: BID32) -> c_int;
  fn __bid32_isFinite(x: BID32) -> c_int;
  fn __bid32_isInf(x: BID32) -> c_int;
  fn __bid32_isNaN(x: BID32) -> c_int;
  fn __bid32_isNormal(x: BID32) -> c_int;
  fn __bid32_isSignaling(x: BID32) -> c_int;
  fn __bid32_isSigned(x: BID32) -> c_int;
  fn __bid32_isSubnormal(x: BID32) -> c_int;
  fn __bid32_isZero(x: BID32) -> c_int;
  fn __bid32_ldexp(x: BID32, n: c_int, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_lgamma(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_llrint(x: BID32, round: c_uint, flags: *mut c_uint) -> c_longlong;
  fn __bid32_llround(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_llquantexp(x: BID32) -> c_longlong;
  fn __bid32_log(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_log10(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_log1p(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_log2(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_logb(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_lrint(x: BID32, round: c_uint, flags: *mut c_uint) -> c_long;
  fn __bid32_lround(x: BID32, flags: *mut c_uint) -> c_long;
  fn __bid32_maxnum(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_maxnum_mag(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_minnum(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_minnum_mag(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_modf(x: BID32, int: *mut BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_mul(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_nan(s: *const c_char) -> BID32;
  fn __bid32_nearbyint(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_nextafter(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_nextdown(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_nexttoward(x: BID32, y: BID128, flags: *mut c_uint) -> BID32;
  fn __bid32_nextup(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_negate(x: BID32) -> BID32;
  fn __bid32_pow(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_quantexp(x: BID32) -> c_int;
  fn __bid32_quantum(x: BID32) -> BID32;
  fn __bid32_quantize(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_quiet_equal(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_greater(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_greater_equal(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_greater_unordered(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_less(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_less_equal(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_less_unordered(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_not_equal(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_not_greater(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_not_less(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_ordered(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_quiet_unordered(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_radix(x: BID32) -> c_int;
  fn __bid32_rem(x: BID32, y: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_round_integral_exact(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_round_integral_nearest_away(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_round_integral_nearest_even(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_round_integral_negative(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_round_integral_positive(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_round_integral_zero(x: BID32, flags: *mut c_uint) -> BID32;
  fn __bid32_sameQuantum(x: BID32, y: BID32) -> c_int;
  fn __bid32_scalbn(x: BID32, n: c_int) -> BID32;
  fn __bid32_scalbln(x: BID32, n: c_longlong) -> BID32;
  fn __bid32_signaling_greater(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_greater_equal(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_greater_unordered(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_less(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_less_equal(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_less_unordered(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_not_greater(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_signaling_not_less(x: BID32, y: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_sin(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_sinh(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_sqrt(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_sub(x: BID32, y: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_tan(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_tanh(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_tgamma(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_to_bid32(x: BID32, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_to_bid128(x: BID32, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid32_to_binary32(x: BID32, round: c_uint, flags: *mut c_uint) -> c_float;
  fn __bid32_to_binary64(x: BID32, round: c_uint, flags: *mut c_uint) -> c_double;
  fn __bid32_to_int16_ceil(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_floor(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_int(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_rnint(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_rninta(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_xceil(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_xfloor(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_xint(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_xrnint(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int16_xrninta(x: BID32, flags: *mut c_uint) -> c_short;
  fn __bid32_to_int32_ceil(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_floor(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_int(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_rnint(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_rninta(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_xceil(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_xfloor(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_xint(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_xrnint(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int32_xrninta(x: BID32, flags: *mut c_uint) -> c_int;
  fn __bid32_to_int64_ceil(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_floor(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_int(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_rnint(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_rninta(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_xceil(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_xfloor(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_xint(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_xrnint(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int64_xrninta(x: BID32, flags: *mut c_uint) -> c_longlong;
  fn __bid32_to_int8_ceil(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_floor(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_int(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_rnint(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_rninta(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_xceil(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_xfloor(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_xint(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_xrnint(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_int8_xrninta(x: BID32, flags: *mut c_uint) -> c_char;
  fn __bid32_to_uint16_ceil(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_floor(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_int(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_rnint(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_rninta(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_xceil(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_xfloor(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_xint(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_xrnint(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint16_xrninta(x: BID32, flags: *mut c_uint) -> c_ushort;
  fn __bid32_to_uint32_ceil(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_floor(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_int(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_rnint(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_rninta(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_xceil(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_xfloor(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_xint(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_xrnint(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint32_xrninta(x: BID32, flags: *mut c_uint) -> c_uint;
  fn __bid32_to_uint64_ceil(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_floor(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_int(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_rnint(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_rninta(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_xceil(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_xfloor(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_xint(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_xrnint(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint64_xrninta(x: BID32, flags: *mut c_uint) -> c_ulonglong;
  fn __bid32_to_uint8_ceil(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_floor(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_int(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_rnint(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_rninta(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_xceil(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_xfloor(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_xint(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_xrnint(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_uint8_xrninta(x: BID32, flags: *mut c_uint) -> c_uchar;
  fn __bid32_to_string(s: *mut c_char, x: BID32, flags: *mut c_uint);
  fn __bid32_totalOrder(x: BID32, y: BID32) -> c_int;
  fn __bid32_totalOrderMag(x: BID32, y: BID32) -> c_int;
}

pub fn bid32_abs(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_abs(x, flags) }
}

/// Returns `arcsin(x)`.
pub fn bid32_acos(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_acos(x, round, flags) }
}

/// Returns `arsinh(x)`.
pub fn bid32_acosh(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_acosh(x, round, flags) }
}

/// Returns a result of decimal floating-point addition.
pub fn bid32_add(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_add(x, y, round, flags) }
}

/// Returns `arcsin(x)`.
pub fn bid32_asin(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_asin(x, round, flags) }
}

/// Returns `arsinh(x)`.
pub fn bid32_asinh(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_asinh(x, round, flags) }
}

/// Returns `arctan(x)`.
pub fn bid32_atan(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_atan(x, round, flags) }
}

/// Returns `arctan2(x, y)`.
pub fn bid32_atan2(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_atan2(x, y, round, flags) }
}

/// Returns `artanh(x)`.
pub fn bid32_atanh(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_atanh(x, round, flags) }
}

/// Returns the cube root of the argument `x`.
pub fn bid32_cbrt(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_cbrt(x, round, flags) }
}

/// Copies a 32-bit decimal floating-point operand to a destination
/// in the same format, with no change.
pub fn bid32_copy(x: BID32) -> BID32 {
  unsafe { __bid32_copy(x) }
}

/// Copies argument `x` to destination in the same format as `x`, but with the sign of `y`.
pub fn bid32_copy_sign(x: BID32, y: BID32) -> BID32 {
  unsafe { __bid32_copySign(x, y) }
}

/// Returns `cos(x)`.
pub fn bid32_cos(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_cos(x, round, flags) }
}

/// Returns `cosh(x)`.
pub fn bid32_cosh(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_cosh(x, round, flags) }
}

/// Returns the class of the specified argument `x`.
pub fn bid32_class(x: BID32) -> Class {
  unsafe { __bid32_class(x) as u32 }.into()
}

/// Returns a result of decimal floating-point division.
pub fn bid32_div(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_div(x, y, round, flags) }
}

/// Returns the result of Gaussian error function for specified `x`.
pub fn bid32_erf(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_erf(x, round, flags) }
}

/// Returns the result of the complementary Gaussian error function for specified `x`: `erfc(x) = 1 - erf(x)`.
pub fn bid32_erfc(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_erfc(x, round, flags) }
}

/// Returns the value of `e` raised to the `x`th power.
pub fn bid32_exp(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_exp(x, round, flags) }
}

/// Returns the `10^x`.
pub fn bid32_exp10(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_exp10(x, round, flags) }
}

/// Returns the `2^x`.
pub fn bid32_exp2(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_exp2(x, round, flags) }
}

/// Returns the `e^x - 1`.
pub fn bid32_expm1(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_expm1(x, round, flags) }
}

/// Returns positive difference between `x` and `y`.
/// Result is x - y if x > y, and +0 is x <= y.
pub fn bid32_fdim(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_fdim(x, y, round, flags) }
}

/// Returns `(x * y) + z` rounded as one ternary operation.
pub fn bid32_fma(x: BID32, y: BID32, z: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_fma(x, y, z, round, flags) }
}

/// Returns the remainder of the division `x/y`.
pub fn bid32_fmod(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_fmod(x, y, flags) }
}

/// If x is not a floating-point number, the results are unspecified (this
/// implementation returns x and *exp = 0). Otherwise, the frexp function
/// returns the value res, such that res has a magnitude in the interval
/// [1/10, 1) or zero, and x = res*2^*exp. If x is zero, both parts of the
/// result are zero. `frexp` does not raise any exceptions.
pub fn bid32_frexp(x: BID32, exp: &mut i32) -> BID32 {
  unsafe { __bid32_frexp(x, exp) }
}

/// Converts 32-bit signed integer to 32-bit decimal floating-point number.
pub fn bid32_from_int32(x: i32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_from_int32(x, round, flags) }
}

/// Converts 32-bit signed integer to 32-bit decimal floating-point number.
pub fn bid32_from_int64(x: i64, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_from_int64(x, round, flags) }
}

/// Converts a number represented as string format (decimal character sequence)
/// to 32-bit decimal floating-point format (binary encoding).
pub fn bid32_from_string(s: &str, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid32_from_string(c_s.as_ptr(), round, flags) }
}

/// Converts 32-bit unsigned integer to 32-bit decimal floating-point number.
pub fn bid32_from_uint32(x: u32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_from_uint32(x, round, flags) }
}

/// Converts 64-bit unsigned integer to 32-bit decimal floating-point number.
pub fn bid32_from_uint64(x: u64, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_from_uint64(x, round, flags) }
}

/// Returns the square root of the squares of two arguments.
pub fn bid32_hypot(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_hypot(x, y, round, flags) }
}

/// Returns the exponent e of x, a signed integral value, determined as though x
/// were represented with infinite range and minimum exponent.
pub fn bid32_ilogb(x: BID32, flags: &mut ExcFlags) -> Signed {
  unsafe { __bid32_ilogb(x, flags) }
}

/// Returns `true` if and only if `x` is canonical number, infinity or NaN.
pub fn bid32_is_canonical(x: BID32) -> bool {
  unsafe { __bid32_isCanonical(x) != 0 }
}

/// Returns `true` if and only if `x` is zero, subnormal or normal (not infinite or NaN).
pub fn bid32_is_finite(x: BID32) -> bool {
  unsafe { __bid32_isFinite(x) != 0 }
}

/// Returns infinite value.
pub fn bid32_infinite() -> BID32 {
  unsafe { __bid32_inf() }
}

/// Returns `true` if x is infinite.
pub fn bid32_is_infinite(x: BID32) -> bool {
  unsafe { __bid32_isInf(x) != 0 }
}

pub fn bid32_is_nan(x: BID32) -> bool {
  unsafe { __bid32_isNaN(x) != 0 }
}

pub fn bid32_is_normal(x: BID32) -> bool {
  unsafe { __bid32_isNormal(x) != 0 }
}

/// Returns `true` if and only if x has a negative sign.
pub fn bid32_is_signaling(x: BID32) -> bool {
  unsafe { __bid32_isSignaling(x) != 0 }
}

/// Returns `true` if and only if x has a negative sign.
pub fn bid32_is_signed(x: BID32) -> bool {
  unsafe { __bid32_isSigned(x) != 0 }
}

/// Returns `true` if and only if `x` is subnormal.
pub fn bid32_is_subnormal(x: BID32) -> bool {
  unsafe { __bid32_isSubnormal(x) != 0 }
}

/// Returns `true` if and only if `x` is `+0` or `-0`.
pub fn bid32_is_zero(x: BID32) -> bool {
  unsafe { __bid32_isZero(x) != 0 }
}

/// Returns `x*(10^n)`.
pub fn bid32_ldexp(x: BID32, n: i32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_ldexp(x, n, round, flags) }
}

/// Returns natural logarithm from gamma function.
pub fn bid32_lgamma(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_lgamma(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value of
/// type [i64], rounding according to the provided rounding direction.
pub fn bid32_llrint(x: BID32, round: RndMode, flags: &mut ExcFlags) -> LongLong {
  unsafe { __bid32_llrint(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value of
/// type [i64], using rounding to nearest-away.
pub fn bid32_llround(x: BID32, flags: &mut ExcFlags) -> LongLong {
  unsafe { __bid32_llround(x, flags) }
}

/// Returns natural logarithm of `x`.
pub fn bid32_log(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_log(x, round, flags) }
}

/// Returns base 10 logarithm of `x`.
pub fn bid32_log10(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_log10(x, round, flags) }
}

/// Returns natural logarithm of `1 + x`.
pub fn bid32_log1p(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_log1p(x, round, flags) }
}

/// Returns base 2 logarithm of `x`.
pub fn bid32_log2(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_log2(x, round, flags) }
}

/// Returns the unbiased radix-independent exponent from `x`.
pub fn bid32_logb(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_logb(x, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value,
/// rounding according to the provided rounding direction.
pub fn bid32_lrint(x: BID32, round: RndMode, flags: &mut ExcFlags) -> Long {
  unsafe { __bid32_lrint(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value,
/// using rounding to nearest-away.
pub fn bid32_lround(x: BID32, flags: &mut ExcFlags) -> Long {
  unsafe { __bid32_lround(x, flags) }
}

/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise, it is either x or y, canonicalized.
pub fn bid32_max_num(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_maxnum(x, y, flags) }
}

pub fn bid32_max_num_mag(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_maxnum_mag(x, y, flags) }
}

/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise, it is either x or y, canonicalized.
pub fn bid32_min_num(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_minnum(x, y, flags) }
}

pub fn bid32_min_num_mag(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_minnum_mag(x, y, flags) }
}

/// Splits the number `x` into integral and fractional part.
pub fn bid32_modf(x: BID32, int: &mut BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_modf(x, int, flags) }
}

/// Returns a result of decimal floating-point multiplication.
pub fn bid32_mul(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_mul(x, y, round, flags) }
}

/// Returns `NaN` with payload.
pub fn bid32_nan(s: &str) -> BID32 {
  let cstring = CString::new(s).unwrap();
  unsafe { __bid32_nan(cstring.as_ptr()) }
}

pub fn bid32_nearbyint(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_nearbyint(x, round, flags) }
}

pub fn bid32_nextafter(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_nextafter(x, y, flags) }
}

pub fn bid32_nextdown(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_nextdown(x, flags) }
}

pub fn bid32_nexttoward(x: BID32, y: BID128, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_nexttoward(x, y, flags) }
}

pub fn bid32_nextup(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_nextup(x, flags) }
}

/// Returns the same value as `x` but with reversed sign.
pub fn bid32_negate(x: BID32) -> BID32 {
  unsafe { __bid32_negate(x) }
}

/// Returns decimal floating-point power.
pub fn bid32_pow(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_pow(x, y, round, flags) }
}

/// Returns the quantum of a finite argument as a signed integer value.
pub fn bid32_quantexp(x: BID32) -> Signed {
  unsafe { __bid32_quantexp(x) }
}

/// Returns the quantum of a finite argument as a signed long long integer value.
pub fn bid32_llquantexp(x: BID32) -> LongLong {
  unsafe { __bid32_llquantexp(x) }
}

/// Returns the quantum of a finite argument.
/// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
pub fn bid32_quantum(x: BID32) -> BID32 {
  unsafe { __bid32_quantum(x) }
}

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
pub fn bid32_quantize(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_quantize(x, y, round, flags) }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid32_quiet_equal(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_equal(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid32_quiet_greater(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_greater(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid32_quiet_greater_equal(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_greater_equal(x, y, flags) != 0 }
}

pub fn bid32_quiet_greater_unordered(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_greater_unordered(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid32_quiet_less(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_less(x, y, flags) != 0 }
}

/// Compares 64-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid32_quiet_less_equal(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_less_equal(x, y, flags) != 0 }
}

pub fn bid32_quiet_less_unordered(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_less_unordered(x, y, flags) != 0 }
}

pub fn bid32_quiet_not_equal(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_not_equal(x, y, flags) != 0 }
}

pub fn bid32_quiet_not_greater(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_not_greater(x, y, flags) != 0 }
}

pub fn bid32_quiet_not_less(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_not_less(x, y, flags) != 0 }
}

pub fn bid32_quiet_ordered(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_ordered(x, y, flags) != 0 }
}

pub fn bid32_quiet_unordered(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_quiet_unordered(x, y, flags) != 0 }
}

pub fn bid32_radix(x: BID32) -> Signed {
  unsafe { __bid32_radix(x) }
}

/// Returns decimal floating-point remainder.
pub fn bid32_rem(x: BID32, y: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_rem(x, y, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the current rounding mode; signals inexact exceptions.
pub fn bid32_round_integral_exact(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_round_integral_exact(x, round, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-nearest-away** mode; does not signal inexact exceptions.
pub fn bid32_round_integral_nearest_away(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_round_integral_nearest_away(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-nearest-even** mode; does not signal inexact exceptions.
pub fn bid32_round_integral_nearest_even(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_round_integral_nearest_even(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-down** mode; does not signal inexact exceptions.
pub fn bid32_round_integral_negative(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_round_integral_negative(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-up** mode; does not signal inexact exceptions.
pub fn bid32_round_integral_positive(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_round_integral_positive(x, flags) }
}

/// Rounds 64-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-zero** mode; does not signal inexact exceptions.
pub fn bid32_round_integral_zero(x: BID32, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_round_integral_zero(x, flags) }
}

pub fn bid32_same_quantum(x: BID32, y: BID32) -> bool {
  unsafe { __bid32_sameQuantum(x, y) != 0 }
}

/// Returns `x * 10^n` where `n` is of type [i32].
pub fn bid32_scalbn(x: BID32, n: i32) -> BID32 {
  unsafe { __bid32_scalbn(x, n) }
}

/// Returns `x * 10^n` where `n` is of type [i64].
pub fn bid32_scalbln(x: BID32, n: i64) -> BID32 {
  unsafe { __bid32_scalbln(x, n) }
}

pub fn bid32_signaling_greater(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_greater(x, y, flags) != 0 }
}

pub fn bid32_signaling_greater_equal(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_greater_equal(x, y, flags) != 0 }
}

pub fn bid32_signaling_greater_unordered(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_greater_unordered(x, y, flags) != 0 }
}

pub fn bid32_signaling_less(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_less(x, y, flags) != 0 }
}

pub fn bid32_signaling_less_equal(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_less_equal(x, y, flags) != 0 }
}

pub fn bid32_signaling_less_unordered(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_less_unordered(x, y, flags) != 0 }
}

pub fn bid32_signaling_not_greater(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_not_greater(x, y, flags) != 0 }
}

pub fn bid32_signaling_not_less(x: BID32, y: BID32, flags: &mut ExcFlags) -> bool {
  unsafe { __bid32_signaling_not_less(x, y, flags) != 0 }
}

/// Returns `sin(x)`.
pub fn bid32_sin(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_sin(x, round, flags) }
}

/// Returns `sinh(x)`.
pub fn bid32_sinh(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_sinh(x, round, flags) }
}

/// Returns decimal floating-point square root.
pub fn bid32_sqrt(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_sqrt(x, round, flags) }
}

/// Returns a result of decimal floating-point subtraction.
pub fn bid32_sub(x: BID32, y: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_sub(x, y, round, flags) }
}

/// Returns `tan(x)`.
pub fn bid32_tan(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_tan(x, round, flags) }
}

/// Returns `tanh(x)`.
pub fn bid32_tanh(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_tanh(x, round, flags) }
}

pub fn bid32_tgamma(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_tgamma(x, round, flags) }
}

pub fn bid32_to_bid32(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  unsafe { __bid32_to_bid32(x, round, flags) }
}

pub fn bid32_to_bid128(x: BID32, round: RndMode, flags: &mut ExcFlags) -> BID128 {
  unsafe { __bid32_to_bid128(x, round, flags) }
}

pub fn bid32_to_binary32(x: BID32, round: RndMode, flags: &mut ExcFlags) -> Float {
  unsafe { __bid32_to_binary32(x, round, flags) }
}

pub fn bid32_to_binary64(x: BID32, round: RndMode, flags: &mut ExcFlags) -> Double {
  unsafe { __bid32_to_binary64(x, round, flags) }
}

pub fn bid32_to_int16_ceil(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_ceil(x, flags) }
}

pub fn bid32_to_int16_floor(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_floor(x, flags) }
}

pub fn bid32_to_int16_int(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_int(x, flags) }
}

pub fn bid32_to_int16_rnint(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_rnint(x, flags) }
}

pub fn bid32_to_int16_rninta(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_rninta(x, flags) }
}

pub fn bid32_to_int16_xceil(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_xceil(x, flags) }
}

pub fn bid32_to_int16_xfloor(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_xfloor(x, flags) }
}

pub fn bid32_to_int16_xint(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_xint(x, flags) }
}

pub fn bid32_to_int16_xrnint(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_xrnint(x, flags) }
}

pub fn bid32_to_int16_xrninta(x: BID32, flags: &mut ExcFlags) -> i16 {
  unsafe { __bid32_to_int16_xrninta(x, flags) }
}

pub fn bid32_to_int32_ceil(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_ceil(x, flags) }
}

pub fn bid32_to_int32_floor(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_floor(x, flags) }
}

pub fn bid32_to_int32_int(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_int(x, flags) }
}

pub fn bid32_to_int32_rnint(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_rnint(x, flags) }
}

pub fn bid32_to_int32_rninta(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_rninta(x, flags) }
}

pub fn bid32_to_int32_xceil(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_xceil(x, flags) }
}

pub fn bid32_to_int32_xfloor(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_xfloor(x, flags) }
}

pub fn bid32_to_int32_xint(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_xint(x, flags) }
}

pub fn bid32_to_int32_xrnint(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_xrnint(x, flags) }
}

pub fn bid32_to_int32_xrninta(x: BID32, flags: &mut ExcFlags) -> i32 {
  unsafe { __bid32_to_int32_xrninta(x, flags) }
}

pub fn bid32_to_int64_ceil(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_ceil(x, flags) }
}

pub fn bid32_to_int64_floor(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_floor(x, flags) }
}

pub fn bid32_to_int64_int(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_int(x, flags) }
}

pub fn bid32_to_int64_rnint(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_rnint(x, flags) }
}

pub fn bid32_to_int64_rninta(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_rninta(x, flags) }
}

pub fn bid32_to_int64_xceil(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_xceil(x, flags) }
}

pub fn bid32_to_int64_xfloor(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_xfloor(x, flags) }
}

pub fn bid32_to_int64_xint(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_xint(x, flags) }
}

pub fn bid32_to_int64_xrnint(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_xrnint(x, flags) }
}

pub fn bid32_to_int64_xrninta(x: BID32, flags: &mut ExcFlags) -> i64 {
  unsafe { __bid32_to_int64_xrninta(x, flags) }
}

pub fn bid32_to_int8_ceil(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_ceil(x, flags) }
}

pub fn bid32_to_int8_floor(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_floor(x, flags) }
}

pub fn bid32_to_int8_int(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_int(x, flags) }
}

pub fn bid32_to_int8_rnint(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_rnint(x, flags) }
}

pub fn bid32_to_int8_rninta(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_rninta(x, flags) }
}

pub fn bid32_to_int8_xceil(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_xceil(x, flags) }
}

pub fn bid32_to_int8_xfloor(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_xfloor(x, flags) }
}

pub fn bid32_to_int8_xint(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_xint(x, flags) }
}

pub fn bid32_to_int8_xrnint(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_xrnint(x, flags) }
}

pub fn bid32_to_int8_xrninta(x: BID32, flags: &mut ExcFlags) -> i8 {
  unsafe { __bid32_to_int8_xrninta(x, flags) }
}

pub fn bid32_to_uint16_ceil(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_ceil(x, flags) }
}

pub fn bid32_to_uint16_floor(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_floor(x, flags) }
}

pub fn bid32_to_uint16_int(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_int(x, flags) }
}

pub fn bid32_to_uint16_rnint(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_rnint(x, flags) }
}

pub fn bid32_to_uint16_rninta(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_rninta(x, flags) }
}

pub fn bid32_to_uint16_xceil(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_xceil(x, flags) }
}

pub fn bid32_to_uint16_xfloor(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_xfloor(x, flags) }
}

pub fn bid32_to_uint16_xint(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_xint(x, flags) }
}

pub fn bid32_to_uint16_xrnint(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_xrnint(x, flags) }
}

pub fn bid32_to_uint16_xrninta(x: BID32, flags: &mut ExcFlags) -> u16 {
  unsafe { __bid32_to_uint16_xrninta(x, flags) }
}

pub fn bid32_to_uint32_ceil(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_ceil(x, flags) }
}

pub fn bid32_to_uint32_floor(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_floor(x, flags) }
}

pub fn bid32_to_uint32_int(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_int(x, flags) }
}

pub fn bid32_to_uint32_rnint(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_rnint(x, flags) }
}

pub fn bid32_to_uint32_rninta(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_rninta(x, flags) }
}

pub fn bid32_to_uint32_xceil(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_xceil(x, flags) }
}

pub fn bid32_to_uint32_xfloor(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_xfloor(x, flags) }
}

pub fn bid32_to_uint32_xint(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_xint(x, flags) }
}

pub fn bid32_to_uint32_xrnint(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_xrnint(x, flags) }
}

pub fn bid32_to_uint32_xrninta(x: BID32, flags: &mut ExcFlags) -> u32 {
  unsafe { __bid32_to_uint32_xrninta(x, flags) }
}

pub fn bid32_to_uint64_ceil(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_ceil(x, flags) }
}

pub fn bid32_to_uint64_floor(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_floor(x, flags) }
}

pub fn bid32_to_uint64_int(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_int(x, flags) }
}

pub fn bid32_to_uint64_rnint(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_rnint(x, flags) }
}

pub fn bid32_to_uint64_rninta(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_rninta(x, flags) }
}

pub fn bid32_to_uint64_xceil(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_xceil(x, flags) }
}

pub fn bid32_to_uint64_xfloor(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_xfloor(x, flags) }
}

pub fn bid32_to_uint64_xint(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_xint(x, flags) }
}

pub fn bid32_to_uint64_xrnint(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_xrnint(x, flags) }
}

pub fn bid32_to_uint64_xrninta(x: BID32, flags: &mut ExcFlags) -> u64 {
  unsafe { __bid32_to_uint64_xrninta(x, flags) }
}

pub fn bid32_to_uint8_ceil(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_ceil(x, flags) }
}

pub fn bid32_to_uint8_floor(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_floor(x, flags) }
}

pub fn bid32_to_uint8_int(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_int(x, flags) }
}

pub fn bid32_to_uint8_rnint(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_rnint(x, flags) }
}

pub fn bid32_to_uint8_rninta(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_rninta(x, flags) }
}

pub fn bid32_to_uint8_xceil(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_xceil(x, flags) }
}

pub fn bid32_to_uint8_xfloor(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_xfloor(x, flags) }
}

pub fn bid32_to_uint8_xint(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_xint(x, flags) }
}

pub fn bid32_to_uint8_xrnint(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_xrnint(x, flags) }
}

pub fn bid32_to_uint8_xrninta(x: BID32, flags: &mut ExcFlags) -> u8 {
  unsafe { __bid32_to_uint8_xrninta(x, flags) }
}

/// Converts 32-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
pub fn bid32_to_string(x: BID32, flags: &mut ExcFlags) -> String {
  let mut buf = [0_u8; 1024];
  unsafe {
    __bid32_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
  }
}

pub fn bid32_total_order(x: BID32, y: BID32) -> bool {
  unsafe { __bid32_totalOrder(x, y) != 0 }
}

pub fn bid32_total_order_mag(x: BID32, y: BID32) -> bool {
  unsafe { __bid32_totalOrderMag(x, y) != 0 }
}
