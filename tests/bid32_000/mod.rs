mod bid32_abs;
mod bid32_acos;
mod bid32_acosh;
mod bid32_add;
mod bid32_asin;
mod bid32_asinh;
mod bid32_atan;
mod bid32_atan2;
mod bid32_atanh;
mod bid32_cbrt;
mod bid32_class;
mod bid32_constants;
mod bid32_copy;
mod bid32_copy_sign;
mod bid32_cos;
mod bid32_cosh;
mod bid32_div;
mod bid32_erf;
mod bid32_erfc;
mod bid32_exp;
mod bid32_exp10;
mod bid32_exp2;
mod bid32_expm1;
mod bid32_fdim;
mod bid32_fma;
mod bid32_fmod;
mod bid32_frexp;
mod bid32_from;
mod bid32_from_string;
mod bid32_hypot;
mod bid32_ilogb;
mod bid32_infinite;
mod bid32_is_canonical;
mod bid32_is_finite;
mod bid32_is_nan;
mod bid32_is_normal;
mod bid32_is_signaling;
mod bid32_is_signed;
mod bid32_is_subnormal;
mod bid32_is_zero;
mod bid32_ldexp;
mod bid32_lgamma;
mod bid32_llquantexp;
mod bid32_llrint;
mod bid32_llround;
mod bid32_log;
mod bid32_log10;
mod bid32_log1p;
mod bid32_log2;
mod bid32_logb;
mod bid32_lrint;
mod bid32_lround;
mod bid32_max_num;
mod bid32_max_num_mag;
mod bid32_min_num;
mod bid32_min_num_mag;
mod bid32_miscellaneous;
mod bid32_modf;
mod bid32_mul;
mod bid32_nan;
mod bid32_nearbyint;
mod bid32_negate;
mod bid32_nextafter;
mod bid32_nextdown;
mod bid32_nexttoward;
mod bid32_nextup;
mod bid32_pow;
mod bid32_quantexp;
mod bid32_quantize;
mod bid32_quantum;
mod bid32_quiet_equal;
mod bid32_quiet_greater;
mod bid32_quiet_greater_equal;
mod bid32_quiet_greater_unordered;
mod bid32_quiet_less;
mod bid32_quiet_less_equal;
mod bid32_quiet_less_unordered;
mod bid32_quiet_not_equal;
mod bid32_quiet_not_greater;
mod bid32_quiet_not_less;
mod bid32_quiet_ordered;
mod bid32_quiet_unordered;
mod bid32_radix;
mod bid32_rem;
mod bid32_rounding;
mod bid32_same_quantum;
mod bid32_scalbln;
mod bid32_scalbn;
mod bid32_signaling_greater;
mod bid32_signaling_greater_equal;
mod bid32_signaling_greater_unordered;
mod bid32_signaling_less;
mod bid32_signaling_less_equal;
mod bid32_signaling_less_unordered;
mod bid32_signaling_not_greater;
mod bid32_signaling_not_less;
mod bid32_sin;
mod bid32_sinh;
mod bid32_sqrt;
mod bid32_sub;
mod bid32_tan;
mod bid32_tanh;
mod bid32_tgamma;
mod bid32_to_bid128;
mod bid32_to_bid64;
mod bid32_to_binary32;
mod bid32_to_binary64;
mod bid32_to_int16;
mod bid32_to_int32;
mod bid32_to_int64;
mod bid32_to_int8;
mod bid32_to_string;
mod bid32_to_uint16;
mod bid32_to_uint32;
mod bid32_to_uint64;
mod bid32_to_uint8;
mod bid32_total_order;
mod bid32_total_order_mag;

use super::*;

fn eq(expected: &str, actual: BID32) {
  let mut flags = EXE_CLEAR;
  assert_eq!(expected, bid32_to_string(actual, &mut flags));
  assert_eq!(EXE_CLEAR, flags);
}

fn eqf(expected: u32, actual: u32) {
  assert_eq!(expected, actual);
}

/// Returns a positive subnormal value for testing purposes.
fn bid32_positive_subnormal() -> BID32 {
  BID32::new(0x00000001)
}

/// Returns a negative subnormal value for testing purposes.
fn bid32_negative_subnormal() -> BID32 {
  BID32::new(0x80000001)
}
