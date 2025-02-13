mod bid128_0000;
mod bid128_abs;
mod bid128_acos;
mod bid128_acosh;
mod bid128_add;
mod bid128_asin;
mod bid128_asinh;
mod bid128_atan;
mod bid128_atan2;
mod bid128_atanh;
mod bid128_cbrt;
mod bid128_class;
mod bid128_constants;
mod bid128_copy;
mod bid128_copy_sign;
mod bid128_cos;
mod bid128_cosh;
mod bid128_div;
mod bid128_erf;
mod bid128_erfc;
mod bid128_exp;
mod bid128_exp10;
mod bid128_exp2;
mod bid128_expm1;
mod bid128_fdim;
mod bid128_fma;
mod bid128_fmod;
mod bid128_frexp;
mod bid128_from;
mod bid128_from_string;
mod bid128_hypot;
mod bid128_ilogb;
mod bid128_is_canonical;
mod bid128_is_finite;
mod bid128_is_nan;
mod bid128_is_normal;
mod bid128_is_signaling;
mod bid128_is_signed;
mod bid128_is_subnormal;
mod bid128_is_zero;
mod bid128_ldexp;
mod bid128_lgamma;
mod bid128_llquantexp;
mod bid128_llrint;
mod bid128_llround;
mod bid128_log;
mod bid128_log10;
mod bid128_log1p;
mod bid128_log2;
mod bid128_logb;
mod bid128_lrint;
mod bid128_lround;
mod bid128_max_num;
mod bid128_max_num_mag;
mod bid128_min_num;
mod bid128_min_num_mag;
mod bid128_miscellaneous;
mod bid128_modf;
mod bid128_mul;
mod bid128_nan;
mod bid128_nearbyint;
mod bid128_negate;
mod bid128_nextafter;
mod bid128_nextdown;
mod bid128_nexttoward;
mod bid128_nextup;
mod bid128_pow;
mod bid128_quantexp;
mod bid128_quantize;
mod bid128_quantum;
mod bid128_quiet_equal;
mod bid128_quiet_greater;
mod bid128_quiet_greater_equal;
mod bid128_quiet_greater_unordered;
mod bid128_quiet_less;
mod bid128_quiet_less_equal;
mod bid128_quiet_less_unordered;
mod bid128_quiet_not_equal;
mod bid128_quiet_not_greater;
mod bid128_quiet_not_less;
mod bid128_quiet_ordered;
mod bid128_quiet_unordered;
mod bid128_radix;
mod bid128_rem;
mod bid128_same_quantum;
mod bid128_signaling_greater;
mod bid128_signaling_greater_equal;
mod bid128_signaling_greater_unordered;
mod bid128_signaling_less;
mod bid128_signaling_less_equal;
mod bid128_signaling_less_unordered;
mod bid128_signaling_not_greater;
mod bid128_signaling_not_less;
mod bid128_sin;
mod bid128_sinh;
mod bid128_sub;
mod bid128_tan;
mod bid128_tanh;
mod bid128_tgamma;
mod bid128_to_bid32;
mod bid128_to_bid64;
mod bid128_to_binary32;
mod bid128_to_binary64;
mod bid128_to_int16;
mod bid128_to_int32;
mod bid128_to_int64;
mod bid128_to_int8;
mod bid128_to_string;
mod bid128_to_uint16;
mod bid128_to_uint32;
mod bid128_to_uint64;
mod bid128_to_uint8;
mod bid128_total_order;
mod bid128_total_order_mag;

use super::*;

fn eq(expected: &str, actual: BID128) {
  let mut flags = EXE_CLEAR;
  assert_eq!(expected, bid128_to_string(actual, &mut flags));
  assert_eq!(EXE_CLEAR, flags);
}

fn eqf(expected: u32, actual: u32) {
  assert_eq!(expected, actual);
}

/// Returns a positive subnormal value for testing purposes.
fn bid128_positive_subnormal() -> BID128 {
  BID128::new(0x07a63158fbd6b32f, 0x0002000000000000)
}

/// Returns a negative subnormal value for testing purposes.
fn bid128_negative_subnormal() -> BID128 {
  BID128::new(0x07a63158fbd6b32f, 0x8002000000000000)
}
