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
mod bid32_is_nan;
mod bid32_is_normal;
mod bid32_miscellaneous;
mod bid32_to_string;

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
