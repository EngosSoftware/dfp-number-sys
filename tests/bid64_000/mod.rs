mod bid64_abs;
mod bid64_acos;
mod bid64_acosh;
mod bid64_add;
mod bid64_asin;
mod bid64_asinh;
mod bid64_atan;
mod bid64_atan2;
mod bid64_atanh;
mod bid64_cbrt;
mod bid64_class;
mod bid64_constants;
mod bid64_copy;
mod bid64_copy_sign;
mod bid64_cos;
mod bid64_cosh;
mod bid64_div;
mod bid64_erf;
mod bid64_erfc;
mod bid64_exp;
mod bid64_exp10;
mod bid64_exp2;
mod bid64_expm1;
mod bid64_fdim;
mod bid64_fma;
mod bid64_fmod;
mod bid64_frexp;
mod bid64_from_string;
mod bid64_hypot;
mod bid64_is_canonical;
mod bid64_is_finite;
mod bid64_is_nan;
mod bid64_is_normal;
mod bid64_is_signaling;
mod bid64_is_signed;
mod bid64_is_subnormal;
mod bid64_is_zero;
mod bid64_ldexp;
mod bid64_lgamma;
mod bid64_llquantexp;
mod bid64_llrint;
mod bid64_llround;
mod bid64_log;
mod bid64_log10;
mod bid64_log1p;
mod bid64_log2;
mod bid64_logb;
mod bid64_lrint;
mod bid64_lround;
mod bid64_max_num;
mod bid64_max_num_mag;
mod bid64_min_num;
mod bid64_min_num_mag;
mod bid64_miscellaneous;
mod bid64_modf;
mod bid64_mul;
mod bid64_nan;
mod bid64_nearbyint;
mod bid64_nextafter;
mod bid64_nextdown;
mod bid64_nexttoward;
mod bid64_nextup;
mod bid64_quantexp;
mod bid64_quantum;
mod bid64_quiet_equal;
mod bid64_quiet_greater;
mod bid64_quiet_greater_equal;
mod bid64_quiet_greater_unordered;
mod bid64_quiet_less;
mod bid64_quiet_less_equal;
mod bid64_quiet_less_unordered;
mod bid64_quiet_not_equal;
mod bid64_quiet_not_greater;
mod bid64_quiet_not_less;
mod bid64_quiet_ordered;
mod bid64_quiet_unordered;
mod bid64_radix;
mod bid64_same_quantum;
mod bid64_signaling_greater;
mod bid64_signaling_greater_equal;
mod bid64_signaling_greater_unordered;
mod bid64_signaling_less;
mod bid64_signaling_less_equal;
mod bid64_signaling_less_unordered;
mod bid64_signaling_not_greater;
mod bid64_signaling_not_less;
mod bid64_sin;
mod bid64_sinh;
mod bid64_sub;
mod bid64_tan;
mod bid64_tanh;
mod bid64_tgamma;
mod bid64_to_bid128;
mod bid64_to_bid32;
mod bid64_to_binary32;
mod bid64_to_binary64;
mod bid64_to_int16;
mod bid64_to_int32;
mod bid64_to_int64;
mod bid64_to_int8;
mod bid64_to_string;
mod bid64_to_uint16;
mod bid64_to_uint32;
mod bid64_to_uint64;
mod bid64_to_uint8;
mod bid64_total_order;
mod bid64_total_order_mag;

use super::*;

fn eq(expected: &str, actual: BID64) {
  let mut flags = EXE_CLEAR;
  assert_eq!(expected, bid64_to_string(actual, &mut flags));
  assert_eq!(EXE_CLEAR, flags);
}

fn eqf(expected: u32, actual: u32) {
  assert_eq!(expected, actual);
}

/// Returns a positive subnormal value for testing purposes.
fn bid64_positive_subnormal() -> BID64 {
  BID64::new(0x0000000000000001)
}

/// Returns a negative subnormal value for testing purposes.
fn bid64_negative_subnormal() -> BID64 {
  BID64::new(0x8000000000000001)
}
