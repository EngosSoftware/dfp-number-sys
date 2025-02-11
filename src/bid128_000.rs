//! # `bid128_000` bindings
//!
//! ```text
//! bid128 _ 0 0 0
//! ──┬───   ┬ ┬ ┬
//!   │      │ │ │
//!   │      │ │ └── status flags passed as a separate argument
//!   │      │ └── rounding mode passed as a separate argument
//!   │      └── result returned by value
//!   └─────── 128-bit decimal in BID format
//! ```

use crate::{Class, BID128, FB_CLEAR};
use libc::{c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_ulonglong};
use std::ffi::{CStr, CString};

/// Value `Inf` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_INF;
/// assert_eq!("+Inf", bid128_quiet_to_string(x));
/// ```
pub const BID128_INF: BID128 = BID128 { w: [0x0, 0x7800000000000000] };

/// Value `-Inf` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_MINUS_INF;
/// assert_eq!("-Inf", bid128_quiet_to_string(x));
/// ```
pub const BID128_MINUS_INF: BID128 = BID128 { w: [0x0, 0xF800000000000000] };

/// Value `0` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_ZERO;
/// assert_eq!("+0E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_ZERO: BID128 = BID128 { w: [0x0, 0x3040000000000000] };

/// Value `-0` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_MINUS_ZERO;
/// assert_eq!("-0E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_MINUS_ZERO: BID128 = BID128 { w: [0x0, 0xB040000000000000] };

/// Value `1` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_ONE;
/// assert_eq!("+1E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_ONE: BID128 = BID128 { w: [0x1, 0x3040000000000000] };

/// Value `-1` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_MINUS_ONE;
/// assert_eq!("-1E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_MINUS_ONE: BID128 = BID128 { w: [0x1, 0xB040000000000000] };

/// Value `2` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_TWO;
/// assert_eq!("+2E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_TWO: BID128 = BID128 { w: [0x2, 0x3040000000000000] };

/// Value `-2` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_MINUS_TWO;
/// assert_eq!("-2E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_MINUS_TWO: BID128 = BID128 { w: [0x2, 0xB040000000000000] };

/// Value `0.1` represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_ONE_TENTH;
/// assert_eq!("+1E-1", bid128_quiet_to_string(x));
/// ```
pub const BID128_ONE_TENTH: BID128 = BID128 { w: [0x1, 0x303E000000000000] };

/// Value **`1000000000`** represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_BILLION;
/// assert_eq!("+1000000000E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_BILLION: BID128 = BID128 {
    w: [0x3B9ACA00, 0x3040000000000000],
};

/// Value **`-1000000000`** represented as a 128-bit decimal floating-point.
///
/// # Example
///
/// ```
/// use dfp_number_sys::bid128_000::*;
///
/// let x = BID128_MINUS_BILLION;
/// assert_eq!("-1000000000E+0", bid128_quiet_to_string(x));
/// ```
pub const BID128_MINUS_BILLION: BID128 = BID128 {
    w: [0x3B9ACA00, 0xB040000000000000],
};

extern "C" {
    fn __bid128_abs(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_acos(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_acosh(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_add(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_asin(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_asinh(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_atan(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_atan2(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_atanh(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_cbrt(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_copy(x: BID128) -> BID128;
    fn __bid128_copySign(x: BID128, y: BID128) -> BID128;
    fn __bid128_cos(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_cosh(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_class(x: BID128) -> c_int;
    fn __bid128_div(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_exp(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_exp10(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_exp2(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_expm1(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_erf(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_erfc(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_fdim(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_fma(x: BID128, y: BID128, z: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_fmod(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_frexp(x: BID128, exp: *mut c_int) -> BID128;
    fn __bid128_from_int32(x: c_int) -> BID128;
    fn __bid128_from_int64(x: c_longlong) -> BID128;
    fn __bid128_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_from_uint32(x: c_uint) -> BID128;
    fn __bid128_from_uint64(x: c_ulonglong) -> BID128;
    fn __bid128_hypot(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_ilogb(x: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_inf() -> BID128;
    fn __bid128_isCanonical(x: BID128) -> c_int;
    fn __bid128_isFinite(x: BID128) -> c_int;
    fn __bid128_isInf(x: BID128) -> c_int;
    fn __bid128_isNaN(x: BID128) -> c_int;
    fn __bid128_isNormal(x: BID128) -> c_int;
    fn __bid128_isSignaling(x: BID128) -> c_int;
    fn __bid128_isSigned(x: BID128) -> c_int;
    fn __bid128_isSubnormal(x: BID128) -> c_int;
    fn __bid128_isZero(x: BID128) -> c_int;
    fn __bid128_ldexp(x: BID128, n: c_int, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_lgamma(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_llrint(x: BID128, round: c_uint, flags: *mut c_uint) -> c_longlong;
    fn __bid128_llround(x: BID128, flags: *mut c_uint) -> c_longlong;
    fn __bid128_llquantexp(x: BID128) -> c_longlong;
    fn __bid128_log(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_log10(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_log1p(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_log2(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_logb(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_lrint(x: BID128, round: c_uint, flags: *mut c_uint) -> c_long;
    fn __bid128_lround(x: BID128, flags: *mut c_uint) -> c_long;
    fn __bid128_maxnum(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_maxnum_mag(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_minnum(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_minnum_mag(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_modf(x: BID128, int: *mut BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_mul(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_nan(s: *const c_char) -> BID128;
    fn __bid128_nearbyint(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_nextafter(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_nextdown(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_nexttoward(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_nextup(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_negate(x: BID128) -> BID128;
    fn __bid128_pow(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_quantexp(x: BID128) -> c_int;
    fn __bid128_quantum(x: BID128) -> BID128;
    fn __bid128_quantize(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_quiet_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_greater(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_greater_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_greater_unordered(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_less(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_less_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_less_unordered(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_not_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_not_greater(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_not_less(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_ordered(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_quiet_unordered(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_rem(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_round_integral_exact(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_round_integral_nearest_away(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_round_integral_nearest_even(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_round_integral_negative(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_round_integral_positive(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_round_integral_zero(x: BID128, flags: *mut c_uint) -> BID128;
    fn __bid128_scalbn(x: BID128, n: c_int) -> BID128;
    fn __bid128_scalbln(x: BID128, n: c_longlong) -> BID128;
    fn __bid128_sin(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_sinh(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_sqrt(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_sub(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_tan(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_tanh(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
    fn __bid128_to_int32_int(x: BID128, flags: *mut c_uint) -> c_int;
    fn __bid128_to_uint32_int(x: BID128, flags: *mut c_uint) -> c_uint;
    fn __bid128_to_int64_int(x: BID128, flags: *mut c_uint) -> c_longlong;
    fn __bid128_to_uint64_int(x: BID128, flags: *mut c_uint) -> c_ulonglong;
    fn __bid128_to_string(s: *mut c_char, x: BID128, flags: *mut c_uint);
}

pub type ExcFlags = c_uint;
pub type RndMode = c_uint;
pub type Signed = c_int;
pub type Unsigned = c_uint;
pub type Long = c_long;
pub type UnsignedLong = c_ulong;
pub type LongLong = c_longlong;
pub type UnsignedLongLong = c_ulonglong;

/// Returns the absolute value of 128-bit decimal floating-point number.
///
/// # Examples
///
/// ```
/// use dfp_number_sys::bid128_000::*;
/// use dfp_number_sys::FB_CLEAR;
///
/// let x = bid128_from_int32(-2);
/// let mut flags = FB_CLEAR;
/// let y = bid128_abs(x, &mut flags);
/// assert_eq!("+2E+0", bid128_quiet_to_string(y));
/// assert_eq!(flags, FB_CLEAR);
/// ```
pub fn bid128_abs(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_abs(x, flags) }
}

/// Returns `arcsin(x)`.
pub fn bid128_acos(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_acos(x, round, flags) }
}

/// Returns `arsinh(x)`.
pub fn bid128_acosh(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_acosh(x, round, flags) }
}

/// Returns a result of decimal floating-point addition.
pub fn bid128_add(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_add(x, y, round, flags) }
}

/// Returns `arcsin(x)`.
pub fn bid128_asin(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_asin(x, round, flags) }
}

/// Returns `arsinh(x)`.
pub fn bid128_asinh(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_asinh(x, round, flags) }
}

/// Returns `arctan(x)`.
pub fn bid128_atan(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_atan(x, round, flags) }
}

/// Returns `arctan2(x, y)`.
pub fn bid128_atan2(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_atan2(x, y, round, flags) }
}

/// Returns `artanh(x)`.
pub fn bid128_atanh(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_atanh(x, round, flags) }
}

/// Returns the cube root of the argument `x`.
pub fn bid128_cbrt(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_cbrt(x, round, flags) }
}

/// Copies a 128-bit decimal floating-point operand to a destination
/// in the same format, with no change.
pub fn bid128_copy(x: BID128) -> BID128 {
    unsafe { __bid128_copy(x) }
}

/// Copies argument `x` to destination in the same format as `x`, but with the sign of `y`.
pub fn bid128_copy_sign(x: BID128, y: BID128) -> BID128 {
    unsafe { __bid128_copySign(x, y) }
}

/// Returns `cos(x)`.
pub fn bid128_cos(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_cos(x, round, flags) }
}

/// Returns `cosh(x)`.
pub fn bid128_cosh(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_cosh(x, round, flags) }
}

/// Returns the class of the specified argument `x`.
pub fn bid128_class(x: BID128) -> Class {
    unsafe { __bid128_class(x) as u32 }.into()
}

/// Returns a result of decimal floating-point division.
pub fn bid128_div(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_div(x, y, round, flags) }
}

/// Returns the result of Gaussian error function for specified `x`.
pub fn bid128_erf(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_erf(x, round, flags) }
}

/// Returns the result of the complementary Gaussian error function for specified `x`: `erfc(x) = 1 - erf(x)`.
pub fn bid128_erfc(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_erfc(x, round, flags) }
}

/// Returns the value of `e` raised to the `x`th power.
pub fn bid128_exp(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_exp(x, round, flags) }
}

/// Returns the `10^x`.
pub fn bid128_exp10(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_exp10(x, round, flags) }
}

/// Returns the `2^x`.
pub fn bid128_exp2(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_exp2(x, round, flags) }
}

/// Returns the `e^x - 1`.
pub fn bid128_expm1(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_expm1(x, round, flags) }
}

/// Returns positive difference between `x` and `y`.
/// Result is x - y if x > y, and +0 is x <= y.
pub fn bid128_fdim(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_fdim(x, y, round, flags) }
}

/// Returns `(x * y) + z` rounded as one ternary operation.
pub fn bid128_fma(x: BID128, y: BID128, z: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_fma(x, y, z, round, flags) }
}

/// Returns the remainder of the division `x/y`.
pub fn bid128_fmod(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_fmod(x, y, flags) }
}

/// If x is not a floating-point number, the results are unspecified (this
/// implementation returns x and *exp = 0). Otherwise, the frexp function
/// returns the value res, such that res has a magnitude in the interval
/// [1/10, 1) or zero, and x = res*2^*exp. If x is zero, both parts of the
/// result are zero. `frexp` does not raise any exceptions.
pub fn bid128_frexp(x: BID128, exp: &mut i32) -> BID128 {
    unsafe { __bid128_frexp(x, exp) }
}

/// Converts 32-bit signed integer to 128-bit decimal floating-point number.
pub fn bid128_from_int32(x: i32) -> BID128 {
    unsafe { __bid128_from_int32(x) }
}

/// Converts 64-bit signed integer to 128-bit decimal floating-point number.
pub fn bid128_from_int64(x: i64) -> BID128 {
    unsafe { __bid128_from_int64(x) }
}

/// Converts a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
pub fn bid128_from_string(s: &str, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    let c_s = CString::new(s).unwrap();
    unsafe { __bid128_from_string(c_s.as_ptr(), round, flags) }
}

/// Converts 32-bit unsigned integer to 128-bit decimal floating-point number.
pub fn bid128_from_uint32(x: u32) -> BID128 {
    unsafe { __bid128_from_uint32(x) }
}

/// Converts 64-bit unsigned integer to 128-bit decimal floating-point number.
pub fn bid128_from_uint64(x: u64) -> BID128 {
    unsafe { __bid128_from_uint64(x) }
}

/// Returns the square root of the squares of two arguments.
pub fn bid128_hypot(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_hypot(x, y, round, flags) }
}

/// Returns the exponent e of x, a signed integral value, determined as though x
/// were represented with infinite range and minimum exponent.
pub fn bid128_ilogb(x: BID128, flags: &mut ExcFlags) -> Signed {
    unsafe { __bid128_ilogb(x, flags) }
}

/// Returns `true` if and only if `x` is canonical number, infinity or NaN.
pub fn bid128_is_canonical(x: BID128) -> bool {
    unsafe { __bid128_isCanonical(x) != 0 }
}

/// Returns `true` if and only if `x` is zero, subnormal or normal (not infinite or NaN).
pub fn bid128_is_finite(x: BID128) -> bool {
    unsafe { __bid128_isFinite(x) != 0 }
}

/// Returns x with infinite value.
pub fn bid128_infinite() -> BID128 {
    unsafe { __bid128_inf() }
}

/// Returns `true` if x is infinite.
pub fn bid128_is_infinite(x: BID128) -> bool {
    unsafe { __bid128_isInf(x) != 0 }
}

/// Returns `true` if `x` is a `NaN`.
pub fn bid128_is_nan(x: BID128) -> bool {
    unsafe { __bid128_isNaN(x) != 0 }
}

/// Returns `true` if and only if `x` is normal (not zero, subnormal, infinite, or NaN).
pub fn bid128_is_normal(x: BID128) -> bool {
    unsafe { __bid128_isNormal(x) != 0 }
}

/// Returns `true` if and only if x has a negative sign.
pub fn bid128_is_signaling(x: BID128) -> bool {
    unsafe { __bid128_isSignaling(x) != 0 }
}

/// Returns `true` if and only if x has a negative sign.
pub fn bid128_is_signed(x: BID128) -> bool {
    unsafe { __bid128_isSigned(x) != 0 }
}

/// Returns `true` if and only if `x` is subnormal.
pub fn bid128_is_subnormal(x: BID128) -> bool {
    unsafe { __bid128_isSubnormal(x) != 0 }
}

/// Returns `true` if and only if `x` is `+0` or `-0`.
pub fn bid128_is_zero(x: BID128) -> bool {
    unsafe { __bid128_isZero(x) != 0 }
}

/// Returns `x*(10^n)`.
pub fn bid128_ldexp(x: BID128, n: i32, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_ldexp(x, n, round, flags) }
}

/// Returns natural logarithm from gamma function.
pub fn bid128_lgamma(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_lgamma(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value of
/// type [i64], rounding according to the provided rounding direction.
pub fn bid128_llrint(x: BID128, round: RndMode, flags: &mut ExcFlags) -> LongLong {
    unsafe { __bid128_llrint(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value of
/// type [i64], using rounding to nearest-away.
pub fn bid128_llround(x: BID128, flags: &mut ExcFlags) -> LongLong {
    unsafe { __bid128_llround(x, flags) }
}

/// Returns natural logarithm of `x`.
pub fn bid128_log(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_log(x, round, flags) }
}

/// Returns base 10 logarithm of `x`.
pub fn bid128_log10(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_log10(x, round, flags) }
}

/// Returns natural logarithm of `1 + x`.
pub fn bid128_log1p(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_log1p(x, round, flags) }
}

/// Returns base 2 logarithm of `x`.
pub fn bid128_log2(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_log2(x, round, flags) }
}

/// Returns the unbiased radix-independent exponent from `x`.
pub fn bid128_logb(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_logb(x, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value,
/// rounding according to the provided rounding direction.
#[cfg(target_pointer_width = "64")]
pub fn bid128_lrint(x: BID128, round: RndMode, flags: &mut ExcFlags) -> Long {
    unsafe { __bid128_lrint(x, round, flags) }
}

/// Returns its argument `x` rounded to the nearest integer value,
/// using rounding to nearest-away.
pub fn bid128_lround(x: BID128, flags: &mut ExcFlags) -> Long {
    unsafe { __bid128_lround(x, flags) }
}

/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise, it is either x or y, canonicalized.
pub fn bid128_max_num(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_maxnum(x, y, flags) }
}

pub fn bid128_max_num_mag(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_maxnum_mag(x, y, flags) }
}

/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise, it is either x or y, canonicalized.
pub fn bid128_min_num(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_minnum(x, y, flags) }
}

pub fn bid128_min_num_mag(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_minnum_mag(x, y, flags) }
}

/// Splits the number `x` into integral and fractional part.
pub fn bid128_modf(x: BID128, int: &mut BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_modf(x, int, flags) }
}

/// Returns a result of decimal floating-point multiplication.
pub fn bid128_mul(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_mul(x, y, round, flags) }
}

/// Returns `NaN` with payload.
pub fn bid128_nan(s: &str) -> BID128 {
    let cstring = CString::new(s).unwrap();
    unsafe { __bid128_nan(cstring.as_ptr()) }
}

pub fn bid128_nearbyint(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_nearbyint(x, round, flags) }
}

pub fn bid128_nextafter(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_nextafter(x, y, flags) }
}

pub fn bid128_nextdown(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_nextdown(x, flags) }
}

pub fn bid128_nexttoward(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_nexttoward(x, y, flags) }
}

pub fn bid128_nextup(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_nextup(x, flags) }
}

/// Returns the same value as `x` but with reversed sign.
pub fn bid128_negate(x: BID128) -> BID128 {
    unsafe { __bid128_negate(x) }
}

/// Returns decimal floating-point power.
pub fn bid128_pow(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_pow(x, y, round, flags) }
}

/// Returns the quantum of a finite argument as a signed integer value.
pub fn bid128_quantexp(x: BID128) -> Signed {
    unsafe { __bid128_quantexp(x) }
}

/// Returns the quantum of a finite argument as a signed long long integer value.
pub fn bid128_llquantexp(x: BID128) -> LongLong {
    unsafe { __bid128_llquantexp(x) }
}

/// Returns the quantum of a finite argument.
/// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
pub fn bid128_quantum(x: BID128) -> BID128 {
    unsafe { __bid128_quantum(x) }
}

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
pub fn bid128_quantize(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_quantize(x, y, round, flags) }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_equal(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_equal(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_greater(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_greater(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_greater_equal(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_greater_equal(x, y, flags) != 0 }
}

pub fn bid128_quiet_greater_unordered(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_greater_unordered(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_less(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_less(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_less_equal(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_less_equal(x, y, flags) != 0 }
}

pub fn bid128_quiet_less_unordered(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_less_unordered(x, y, flags) != 0 }
}

pub fn bid128_quiet_not_equal(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_not_equal(x, y, flags) != 0 }
}

pub fn bid128_quiet_not_greater(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_not_greater(x, y, flags) != 0 }
}

pub fn bid128_quiet_not_less(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_not_less(x, y, flags) != 0 }
}

pub fn bid128_quiet_ordered(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_ordered(x, y, flags) != 0 }
}

pub fn bid128_quiet_unordered(x: BID128, y: BID128, flags: &mut ExcFlags) -> bool {
    unsafe { __bid128_quiet_unordered(x, y, flags) != 0 }
}

/// Converts 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence); exceptions are not signaled.
pub fn bid128_quiet_to_string(x: BID128) -> String {
    let mut buf = [0_u8; 1024];
    let mut flags = FB_CLEAR;
    unsafe {
        __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, &mut flags);
        CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
    }
}

/// Returns decimal floating-point remainder.
pub fn bid128_rem(x: BID128, y: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_rem(x, y, flags) }
}

/// Rounds 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the current [rounding mode](crate::RoundingModes); signals inexact exceptions.
pub fn bid128_round_integral_exact(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_round_integral_exact(x, round, flags) }
}

/// Rounds 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-nearest-away** mode; does not signal inexact exceptions.
pub fn bid128_round_integral_nearest_away(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_round_integral_nearest_away(x, flags) }
}

/// Rounds 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-nearest-even** mode; does not signal inexact exceptions.
pub fn bid128_round_integral_nearest_even(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_round_integral_nearest_even(x, flags) }
}

/// Rounds 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-down** mode; does not signal inexact exceptions.
pub fn bid128_round_integral_negative(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_round_integral_negative(x, flags) }
}

/// Rounds 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-up** mode; does not signal inexact exceptions.
pub fn bid128_round_integral_positive(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_round_integral_positive(x, flags) }
}

/// Rounds 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the **rounding-to-zero** mode; does not signal inexact exceptions.
pub fn bid128_round_integral_zero(x: BID128, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_round_integral_zero(x, flags) }
}

/// Returns `x * 10^n` where `n` is of type [i32].
pub fn bid128_scalbn(x: BID128, n: i32) -> BID128 {
    unsafe { __bid128_scalbn(x, n) }
}

/// Returns `x * 10^n` where `n` is of type [i64].
pub fn bid128_scalbln(x: BID128, n: i64) -> BID128 {
    unsafe { __bid128_scalbln(x, n) }
}

/// Returns `sin(x)`.
pub fn bid128_sin(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_sin(x, round, flags) }
}

/// Returns `sinh(x)`.
pub fn bid128_sinh(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_sinh(x, round, flags) }
}

/// Returns decimal floating-point square root.
pub fn bid128_sqrt(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_sqrt(x, round, flags) }
}

/// Returns a result of decimal floating-point subtraction.
pub fn bid128_sub(x: BID128, y: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_sub(x, y, round, flags) }
}

/// Returns `tan(x)`.
pub fn bid128_tan(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_tan(x, round, flags) }
}

/// Returns `tanh(x)`.
pub fn bid128_tanh(x: BID128, round: RndMode, flags: &mut ExcFlags) -> BID128 {
    unsafe { __bid128_tanh(x, round, flags) }
}

/// Converts 128-bit decimal floating-point value to 32-bit signed integer
/// with rounding-to-zero mode; inexact exceptions are not signaled.
pub fn bid128_to_int32_int(x: BID128, flags: &mut ExcFlags) -> Signed {
    unsafe { __bid128_to_int32_int(x, flags) }
}

/// Converts 128-bit decimal floating-point value to 32-bit unsigned integer
/// with rounding-to-zero mode; inexact exceptions are not signaled.
pub fn bid128_to_uint32_int(x: BID128, flags: &mut ExcFlags) -> Unsigned {
    unsafe { __bid128_to_uint32_int(x, flags) }
}

/// Converts 128-bit decimal floating-point value to 64-bit signed integer
/// with rounding-to-zero mode; inexact exceptions are not signaled.
pub fn bid128_to_int64_int(x: BID128, flags: &mut ExcFlags) -> LongLong {
    unsafe { __bid128_to_int64_int(x, flags) }
}

/// Converts 128-bit decimal floating-point value to 64-bit unsigned integer
/// with rounding-to-zero mode; inexact exceptions are not signaled.
pub fn bid128_to_uint64_int(x: BID128, flags: &mut ExcFlags) -> UnsignedLongLong {
    unsafe { __bid128_to_uint64_int(x, flags) }
}

/// Converts 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
pub fn bid128_to_string(x: BID128, flags: &mut ExcFlags) -> String {
    let mut buf = [0_u8; 1024];
    unsafe {
        __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
        CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
    }
}
