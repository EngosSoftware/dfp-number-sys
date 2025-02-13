# Rust bindings for Intel® Decimal Floating-Point Math Library v2.3

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
![build MacOs arm64][build-badge-macos-arm64]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![EULA licensed][eula-badge]][eula-url]
[![Contributor Covenant][coc-badge]][coc-url]

[crates-badge]: https://img.shields.io/crates/v/dfp-number-sys.svg
[crates-url]: https://crates.io/crates/dfp-number-sys
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: LICENSE
[eula-badge]: https://img.shields.io/badge/License-EULA-blue.svg
[eula-url]: IntelRDFPMathLib20U3/eula.txt
[build-badge-linux]: https://github.com/EngosSoftware/dfp-number-sys/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/dfp-number-sys/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/dfp-number-sys/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/dfp-number-sys/actions/workflows/build-macos-m1.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[coc-url]: CODE_OF_CONDUCT.md
[repository-url]: https://github.com/EngosSoftware/dfp-number-sys

| 128-bit bindings                     |
|--------------------------------------|
| [bid128_abs]                         |
| [bid128_acos]                        |
| [bid128_acosh]                       |
| [bid128_add]                         |
| [bid128_add_dd]                      |
| [bid128_add_dq]                      |
| [bid128_add_qd]                      |
| [bid128_asin]                        |
| [bid128_asinh]                       |
| [bid128_atan]                        |
| [bid128_atan2]                       |
| [bid128_atanh]                       |
| [bid128_cbrt]                        |
| [bid128_class]                       |
| [bid128_copy]                        |
| [bid128_copy_sign]                   |
| [bid128_cos]                         |
| [bid128_cosh]                        |
| [bid128_div]                         |
| [bid128_div_dd]                      |
| [bid128_div_dq]                      |
| [bid128_div_qd]                      |
| [bid128_erf]                         |
| [bid128_erfc]                        |
| [bid128_exp]                         |
| [bid128_exp10]                       |
| [bid128_exp2]                        |
| [bid128_expm1]                       |
| [bid128_fdim]                        |
| [bid128_fma]                         |
| [bid128_fma_ddd]                     |
| [bid128_fma_ddq]                     |
| [bid128_fma_dqd]                     |
| [bid128_fma_dqq]                     |
| [bid128_fma_qdd]                     |
| [bid128_fma_qdq]                     |
| [bid128_fma_qqd]                     |
| [bid128_fmod]                        |
| [bid128_frexp]                       |
| [bid128_from_int32]                  |
| [bid128_from_int64]                  |
| [bid128_from_string]                 |
| [bid128_from_uint32]                 |
| [bid128_from_uint64]                 |
| [bid128_hypot]                       |
| [bid128_ilogb]                       |
| [bid128_infinite]                    |
| [bid128_is_canonical]                |
| [bid128_is_finite]                   |
| [bid128_is_infinite]                 |
| [bid128_is_nan]                      |
| [bid128_is_normal]                   |
| [bid128_is_signaling]                |
| [bid128_is_signed]                   |
| [bid128_is_subnormal]                |
| [bid128_is_zero]                     |
| [bid128_ldexp]                       |
| [bid128_lgamma]                      |
| [bid128_llquantexp]                  |
| [bid128_llrint]                      |
| [bid128_llround]                     |
| [bid128_log]                         |
| [bid128_log10]                       |
| [bid128_log1p]                       |
| [bid128_log2]                        |
| [bid128_logb]                        |
| [bid128_lrint]                       |
| [bid128_lround]                      |
| [bid128_max_num]                     |
| [bid128_max_num_mag]                 |
| [bid128_min_num]                     |
| [bid128_min_num_mag]                 |
| [bid128_modf]                        |
| [bid128_mul]                         |
| [bid128_mul_dd]                      |
| [bid128_mul_dq]                      |
| [bid128_mul_qd]                      |
| [bid128_nan]                         |
| [bid128_nearbyint]                   |
| [bid128_negate]                      |
| [bid128_nextafter]                   |
| [bid128_nextdown]                    |
| [bid128_nexttoward]                  |
| [bid128_nextup]                      |
| [bid128_pow]                         |
| [bid128_quantexp]                    |
| [bid128_quantize]                    |
| [bid128_quantum]                     |
| [bid128_quiet_equal]                 |
| [bid128_quiet_greater]               |
| [bid128_quiet_greater_equal]         |
| [bid128_quiet_greater_unordered]     |
| [bid128_quiet_less]                  |
| [bid128_quiet_less_equal]            |
| [bid128_quiet_less_unordered]        |
| [bid128_quiet_not_equal]             |
| [bid128_quiet_not_greater]           |
| [bid128_quiet_not_less]              |
| [bid128_quiet_ordered]               |
| [bid128_quiet_to_string]             |
| [bid128_quiet_unordered]             |
| [bid128_radix]                       |
| [bid128_rem]                         |
| [bid128_round_integral_exact]        |
| [bid128_round_integral_nearest_away] |
| [bid128_round_integral_nearest_even] |
| [bid128_round_integral_negative]     |
| [bid128_round_integral_positive]     |
| [bid128_round_integral_zero]         |
| [bid128_same_quantum]                |
| [bid128_scalbln]                     |
| [bid128_scalbn]                      |
| [bid128_signaling_greater]           |
| [bid128_signaling_greater_equal]     |
| [bid128_signaling_greater_unordered] |
| [bid128_signaling_less]              |
| [bid128_signaling_less_equal]        |
| [bid128_signaling_less_unordered]    |
| [bid128_signaling_not_greater]       |
| [bid128_signaling_not_less]          |
| [bid128_sin]                         |
| [bid128_sinh]                        |
| [bid128_sqrt]                        |
| [bid128_sub]                         |
| [bid128_sub_dd]                      |
| [bid128_sub_dq]                      |
| [bid128_sub_qd]                      |
| [bid128_tan]                         |
| [bid128_tanh]                        |
| [bid128_tgamma]                      |
| [bid128_to_bid32]                    |
| [bid128_to_bid64]                    |
| [bid128_to_binary32]                 |
| [bid128_to_binary64]                 |
| [bid128_to_int16_ceil]               |
| [bid128_to_int16_floor]              |
| [bid128_to_int16_int]                |
| [bid128_to_int16_rnint]              |
| [bid128_to_int16_rninta]             |
| [bid128_to_int16_xceil]              |
| [bid128_to_int16_xfloor]             |
| [bid128_to_int16_xint]               |
| [bid128_to_int16_xrnint]             |
| [bid128_to_int16_xrninta]            |
| [bid128_to_int32_ceil]               |
| [bid128_to_int32_floor]              |
| [bid128_to_int32_int]                |
| [bid128_to_int32_rnint]              |
| [bid128_to_int32_rninta]             |
| [bid128_to_int32_xceil]              |
| [bid128_to_int32_xfloor]             |
| [bid128_to_int32_xint]               |
| [bid128_to_int32_xrnint]             |
| [bid128_to_int32_xrninta]            |
| [bid128_to_int64_ceil]               |
| [bid128_to_int64_floor]              |
| [bid128_to_int64_int]                |
| [bid128_to_int64_rnint]              |
| [bid128_to_int64_rninta]             |
| [bid128_to_int64_xceil]              |
| [bid128_to_int64_xfloor]             |
| [bid128_to_int64_xint]               |
| [bid128_to_int64_xrnint]             |
| [bid128_to_int64_xrninta]            |
| [bid128_to_int8_ceil]                |
| [bid128_to_int8_floor]               |
| [bid128_to_int8_int]                 |
| [bid128_to_int8_rnint]               |
| [bid128_to_int8_rninta]              |
| [bid128_to_int8_xceil]               |
| [bid128_to_int8_xfloor]              |
| [bid128_to_int8_xint]                |
| [bid128_to_int8_xrnint]              |
| [bid128_to_int8_xrninta]             |
| [bid128_to_string]                   |
| [bid128_to_uint16_ceil]              |
| [bid128_to_uint16_floor]             |
| [bid128_to_uint16_int]               |
| [bid128_to_uint16_rnint]             |
| [bid128_to_uint16_rninta]            |
| [bid128_to_uint16_xceil]             |
| [bid128_to_uint16_xfloor]            |
| [bid128_to_uint16_xint]              |
| [bid128_to_uint16_xrnint]            |
| [bid128_to_uint16_xrninta]           |
| [bid128_to_uint32_ceil]              |
| [bid128_to_uint32_floor]             |
| [bid128_to_uint32_int]               |
| [bid128_to_uint32_rnint]             |
| [bid128_to_uint32_rninta]            |
| [bid128_to_uint32_xceil]             |
| [bid128_to_uint32_xfloor]            |
| [bid128_to_uint32_xint]              |
| [bid128_to_uint32_xrnint]            |
| [bid128_to_uint32_xrninta]           |
| [bid128_to_uint64_ceil]              |
| [bid128_to_uint64_floor]             |
| [bid128_to_uint64_int]               |
| [bid128_to_uint64_rnint]             |
| [bid128_to_uint64_rninta]            |
| [bid128_to_uint64_xceil]             |
| [bid128_to_uint64_xfloor]            |
| [bid128_to_uint64_xint]              |
| [bid128_to_uint64_xrnint]            |
| [bid128_to_uint64_xrninta]           |
| [bid128_to_uint8_ceil]               |
| [bid128_to_uint8_floor]              |
| [bid128_to_uint8_int]                |
| [bid128_to_uint8_rnint]              |
| [bid128_to_uint8_rninta]             |
| [bid128_to_uint8_xceil]              |
| [bid128_to_uint8_xfloor]             |
| [bid128_to_uint8_xint]               |
| [bid128_to_uint8_xrnint]             |
| [bid128_to_uint8_xrninta]            |
| [bid128_total_order]                 |
| [bid128_total_order_mag]             |

[bid128_abs]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_abs.html
[bid128_acos]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_acos.html
[bid128_acosh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_acosh.html
[bid128_add]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_add.html
[bid128_add_dd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_add_dd.html
[bid128_add_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_add_dq.html
[bid128_add_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_add_qd.html
[bid128_asin]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_asin.html
[bid128_asinh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_asinh.html
[bid128_atan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_atan.html
[bid128_atan2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_atan2.html
[bid128_atanh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_atanh.html
[bid128_cbrt]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_cbrt.html
[bid128_class]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_class.html
[bid128_copy]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_copy.html
[bid128_copy_sign]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_copy_sign.html
[bid128_cos]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_cos.html
[bid128_cosh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_cosh.html
[bid128_div]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_div.html
[bid128_div_dd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_div_dd.html
[bid128_div_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_div_dq.html
[bid128_div_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_div_qd.html
[bid128_erf]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_erf.html
[bid128_erfc]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_erfc.html
[bid128_exp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_exp.html
[bid128_exp10]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_exp10.html
[bid128_exp2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_exp2.html
[bid128_expm1]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_expm1.html
[bid128_fdim]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fdim.html
[bid128_fma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma.html
[bid128_fma_ddd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_ddd.html
[bid128_fma_ddq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_ddq.html
[bid128_fma_dqd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_dqd.html
[bid128_fma_dqq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_dqq.html
[bid128_fma_qdd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_qdd.html
[bid128_fma_qdq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_qdq.html
[bid128_fma_qqd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fma_qqd.html
[bid128_fmod]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_fmod.html
[bid128_frexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_frexp.html
[bid128_from_int32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_from_int32.html
[bid128_from_int64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_from_int64.html
[bid128_from_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_from_string.html
[bid128_from_uint32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_from_uint32.html
[bid128_from_uint64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_from_uint64.html
[bid128_hypot]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_hypot.html
[bid128_ilogb]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_ilogb.html
[bid128_infinite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_infinite.html
[bid128_is_canonical]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_canonical.html
[bid128_is_finite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_finite.html
[bid128_is_infinite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_infinite.html
[bid128_is_nan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_nan.html
[bid128_is_normal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_normal.html
[bid128_is_signaling]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_signaling.html
[bid128_is_signed]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_signed.html
[bid128_is_subnormal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_subnormal.html
[bid128_is_zero]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_is_zero.html
[bid128_ldexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_ldexp.html
[bid128_lgamma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_lgamma.html
[bid128_llquantexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_llquantexp.html
[bid128_llrint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_llrint.html
[bid128_llround]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_llround.html
[bid128_log]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_log.html
[bid128_log10]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_log10.html
[bid128_log1p]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_log1p.html
[bid128_log2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_log2.html
[bid128_logb]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_logb.html
[bid128_lrint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_lrint.html
[bid128_lround]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_lround.html
[bid128_max_num]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_max_num.html
[bid128_max_num_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_max_num_mag.html
[bid128_min_num]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_min_num.html
[bid128_min_num_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_min_num_mag.html
[bid128_modf]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_modf.html
[bid128_mul]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_mul.html
[bid128_mul_dd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_mul_dd.html
[bid128_mul_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_mul_dq.html
[bid128_mul_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_mul_qd.html
[bid128_nan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_nan.html
[bid128_nearbyint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_nearbyint.html
[bid128_negate]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_negate.html
[bid128_nextafter]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_nextafter.html
[bid128_nextdown]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_nextdown.html
[bid128_nexttoward]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_nexttoward.html
[bid128_nextup]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_nextup.html
[bid128_pow]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_pow.html
[bid128_quantexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quantexp.html
[bid128_quantize]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quantize.html
[bid128_quantum]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quantum.html
[bid128_quiet_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_equal.html
[bid128_quiet_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_greater.html
[bid128_quiet_greater_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_greater_equal.html
[bid128_quiet_greater_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_greater_unordered.html
[bid128_quiet_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_less.html
[bid128_quiet_less_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_less_equal.html
[bid128_quiet_less_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_less_unordered.html
[bid128_quiet_not_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_not_equal.html
[bid128_quiet_not_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_not_greater.html
[bid128_quiet_not_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_not_less.html
[bid128_quiet_ordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_ordered.html
[bid128_quiet_to_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_to_string.html
[bid128_quiet_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_quiet_unordered.html
[bid128_radix]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_radix.html
[bid128_rem]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_rem.html
[bid128_round_integral_exact]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_round_integral_exact.html
[bid128_round_integral_nearest_away]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_round_integral_nearest_away.html
[bid128_round_integral_nearest_even]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_round_integral_nearest_even.html
[bid128_round_integral_negative]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_round_integral_negative.html
[bid128_round_integral_positive]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_round_integral_positive.html
[bid128_round_integral_zero]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_round_integral_zero.html
[bid128_same_quantum]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_same_quantum.html
[bid128_scalbln]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_scalbln.html
[bid128_scalbn]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_scalbn.html
[bid128_signaling_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_greater.html
[bid128_signaling_greater_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_greater_equal.html
[bid128_signaling_greater_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_greater_unordered.html
[bid128_signaling_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_less.html
[bid128_signaling_less_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_less_equal.html
[bid128_signaling_less_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_less_unordered.html
[bid128_signaling_not_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_not_greater.html
[bid128_signaling_not_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_signaling_not_less.html
[bid128_sin]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sin.html
[bid128_sinh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sinh.html
[bid128_sqrt]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sqrt.html
[bid128_sub]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sub.html
[bid128_sub_dd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sub_dd.html
[bid128_sub_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sub_dq.html
[bid128_sub_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_sub_qd.html
[bid128_tan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_tan.html
[bid128_tanh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_tanh.html
[bid128_tgamma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_tgamma.html
[bid128_to_bid32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_bid32.html
[bid128_to_bid64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_bid64.html
[bid128_to_binary32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_binary32.html
[bid128_to_binary64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_binary64.html
[bid128_to_int16_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_ceil.html
[bid128_to_int16_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_floor.html
[bid128_to_int16_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_int.html
[bid128_to_int16_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_rnint.html
[bid128_to_int16_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_rninta.html
[bid128_to_int16_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_xceil.html
[bid128_to_int16_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_xfloor.html
[bid128_to_int16_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_xint.html
[bid128_to_int16_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_xrnint.html
[bid128_to_int16_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int16_xrninta.html
[bid128_to_int32_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_ceil.html
[bid128_to_int32_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_floor.html
[bid128_to_int32_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_int.html
[bid128_to_int32_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_rnint.html
[bid128_to_int32_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_rninta.html
[bid128_to_int32_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_xceil.html
[bid128_to_int32_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_xfloor.html
[bid128_to_int32_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_xint.html
[bid128_to_int32_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_xrnint.html
[bid128_to_int32_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int32_xrninta.html
[bid128_to_int64_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_ceil.html
[bid128_to_int64_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_floor.html
[bid128_to_int64_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_int.html
[bid128_to_int64_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_rnint.html
[bid128_to_int64_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_rninta.html
[bid128_to_int64_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_xceil.html
[bid128_to_int64_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_xfloor.html
[bid128_to_int64_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_xint.html
[bid128_to_int64_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_xrnint.html
[bid128_to_int64_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int64_xrninta.html
[bid128_to_int8_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_ceil.html
[bid128_to_int8_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_floor.html
[bid128_to_int8_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_int.html
[bid128_to_int8_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_rnint.html
[bid128_to_int8_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_rninta.html
[bid128_to_int8_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_xceil.html
[bid128_to_int8_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_xfloor.html
[bid128_to_int8_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_xint.html
[bid128_to_int8_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_xrnint.html
[bid128_to_int8_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_int8_xrninta.html
[bid128_to_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_string.html
[bid128_to_uint16_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_ceil.html
[bid128_to_uint16_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_floor.html
[bid128_to_uint16_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_int.html
[bid128_to_uint16_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_rnint.html
[bid128_to_uint16_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_rninta.html
[bid128_to_uint16_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_xceil.html
[bid128_to_uint16_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_xfloor.html
[bid128_to_uint16_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_xint.html
[bid128_to_uint16_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_xrnint.html
[bid128_to_uint16_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint16_xrninta.html
[bid128_to_uint32_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_ceil.html
[bid128_to_uint32_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_floor.html
[bid128_to_uint32_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_int.html
[bid128_to_uint32_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_rnint.html
[bid128_to_uint32_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_rninta.html
[bid128_to_uint32_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_xceil.html
[bid128_to_uint32_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_xfloor.html
[bid128_to_uint32_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_xint.html
[bid128_to_uint32_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_xrnint.html
[bid128_to_uint32_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint32_xrninta.html
[bid128_to_uint64_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_ceil.html
[bid128_to_uint64_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_floor.html
[bid128_to_uint64_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_int.html
[bid128_to_uint64_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_rnint.html
[bid128_to_uint64_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_rninta.html
[bid128_to_uint64_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_xceil.html
[bid128_to_uint64_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_xfloor.html
[bid128_to_uint64_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_xint.html
[bid128_to_uint64_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_xrnint.html
[bid128_to_uint64_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint64_xrninta.html
[bid128_to_uint8_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_ceil.html
[bid128_to_uint8_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_floor.html
[bid128_to_uint8_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_int.html
[bid128_to_uint8_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_rnint.html
[bid128_to_uint8_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_rninta.html
[bid128_to_uint8_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_xceil.html
[bid128_to_uint8_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_xfloor.html
[bid128_to_uint8_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_xint.html
[bid128_to_uint8_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_xrnint.html
[bid128_to_uint8_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_to_uint8_xrninta.html
[bid128_total_order]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_total_order.html
[bid128_total_order_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.bid128_total_order_mag.html

| 64-bit bindings                     |
|-------------------------------------|
| [bid64_abs]                         |
| [bid64_acos]                        |
| [bid64_acosh]                       |
| [bid64_add]                         |
| [bid64_add_dq]                      |
| [bid64_add_qd]                      |
| [bid64_add_qq]                      |
| [bid64_asin]                        |
| [bid64_asinh]                       |
| [bid64_atan]                        |
| [bid64_atan2]                       |
| [bid64_atanh]                       |
| [bid64_cbrt]                        |
| [bid64_class]                       |
| [bid64_copy]                        |
| [bid64_copy_sign]                   |
| [bid64_cos]                         |
| [bid64_cosh]                        |
| [bid64_div]                         |
| [bid64_div_dq]                      |
| [bid64_div_qd]                      |
| [bid64_div_qq]                      |
| [bid64_erf]                         |
| [bid64_erfc]                        |
| [bid64_exp]                         |
| [bid64_exp10]                       |
| [bid64_exp2]                        |
| [bid64_expm1]                       |
| [bid64_fdim]                        |
| [bid64_fma]                         |
| [bid64_fma_ddq]                     |
| [bid64_fma_dqd]                     |
| [bid64_fma_dqq]                     |
| [bid64_fma_qdd]                     |
| [bid64_fma_qdq]                     |
| [bid64_fma_qqd]                     |
| [bid64_fma_qqq]                     |
| [bid64_fmod]                        |
| [bid64_frexp]                       |
| [bid64_from_int32]                  |
| [bid64_from_int64]                  |
| [bid64_from_string]                 |
| [bid64_from_uint32]                 |
| [bid64_from_uint64]                 |
| [bid64_hypot]                       |
| [bid64_ilogb]                       |
| [bid64_infinite]                    |
| [bid64_is_canonical]                |
| [bid64_is_finite]                   |
| [bid64_is_infinite]                 |
| [bid64_is_nan]                      |
| [bid64_is_normal]                   |
| [bid64_is_signaling]                |
| [bid64_is_signed]                   |
| [bid64_is_subnormal]                |
| [bid64_is_zero]                     |
| [bid64_ldexp]                       |
| [bid64_lgamma]                      |
| [bid64_llquantexp]                  |
| [bid64_llrint]                      |
| [bid64_llround]                     |
| [bid64_log]                         |
| [bid64_log10]                       |
| [bid64_log1p]                       |
| [bid64_log2]                        |
| [bid64_logb]                        |
| [bid64_lrint]                       |
| [bid64_lround]                      |
| [bid64_max_num]                     |
| [bid64_max_num_mag]                 |
| [bid64_min_num]                     |
| [bid64_min_num_mag]                 |
| [bid64_modf]                        |
| [bid64_mul]                         |
| [bid64_mul_dq]                      |
| [bid64_mul_qd]                      |
| [bid64_mul_qq]                      |
| [bid64_nan]                         |
| [bid64_nearbyint]                   |
| [bid64_negate]                      |
| [bid64_nextafter]                   |
| [bid64_nextdown]                    |
| [bid64_nexttoward]                  |
| [bid64_nextup]                      |
| [bid64_pow]                         |
| [bid64_quantexp]                    |
| [bid64_quantize]                    |
| [bid64_quantum]                     |
| [bid64_quiet_equal]                 |
| [bid64_quiet_greater]               |
| [bid64_quiet_greater_equal]         |
| [bid64_quiet_greater_unordered]     |
| [bid64_quiet_less]                  |
| [bid64_quiet_less_equal]            |
| [bid64_quiet_less_unordered]        |
| [bid64_quiet_not_equal]             |
| [bid64_quiet_not_greater]           |
| [bid64_quiet_not_less]              |
| [bid64_quiet_ordered]               |
| [bid64_quiet_to_string]             |
| [bid64_quiet_unordered]             |
| [bid64_radix]                       |
| [bid64_rem]                         |
| [bid64_round_integral_exact]        |
| [bid64_round_integral_nearest_away] |
| [bid64_round_integral_nearest_even] |
| [bid64_round_integral_negative]     |
| [bid64_round_integral_positive]     |
| [bid64_round_integral_zero]         |
| [bid64_same_quantum]                |
| [bid64_scalbln]                     |
| [bid64_scalbn]                      |
| [bid64_signaling_greater]           |
| [bid64_signaling_greater_equal]     |
| [bid64_signaling_greater_unordered] |
| [bid64_signaling_less]              |
| [bid64_signaling_less_equal]        |
| [bid64_signaling_less_unordered]    |
| [bid64_signaling_not_greater]       |
| [bid64_signaling_not_less]          |
| [bid64_sin]                         |
| [bid64_sinh]                        |
| [bid64_sqrt]                        |
| [bid64_sub]                         |
| [bid64_sub_dq]                      |
| [bid64_sub_qd]                      |
| [bid64_sub_qq]                      |
| [bid64_tan]                         |
| [bid64_tanh]                        |
| [bid64_tgamma]                      |
| [bid64_to_bid128]                   |
| [bid64_to_bid32]                    |
| [bid64_to_binary32]                 |
| [bid64_to_binary64]                 |
| [bid64_to_int16_ceil]               |
| [bid64_to_int16_floor]              |
| [bid64_to_int16_int]                |
| [bid64_to_int16_rnint]              |
| [bid64_to_int16_rninta]             |
| [bid64_to_int16_xceil]              |
| [bid64_to_int16_xfloor]             |
| [bid64_to_int16_xint]               |
| [bid64_to_int16_xrnint]             |
| [bid64_to_int16_xrninta]            |
| [bid64_to_int32_ceil]               |
| [bid64_to_int32_floor]              |
| [bid64_to_int32_int]                |
| [bid64_to_int32_rnint]              |
| [bid64_to_int32_rninta]             |
| [bid64_to_int32_xceil]              |
| [bid64_to_int32_xfloor]             |
| [bid64_to_int32_xint]               |
| [bid64_to_int32_xrnint]             |
| [bid64_to_int32_xrninta]            |
| [bid64_to_int64_ceil]               |
| [bid64_to_int64_floor]              |
| [bid64_to_int64_int]                |
| [bid64_to_int64_rnint]              |
| [bid64_to_int64_rninta]             |
| [bid64_to_int64_xceil]              |
| [bid64_to_int64_xfloor]             |
| [bid64_to_int64_xint]               |
| [bid64_to_int64_xrnint]             |
| [bid64_to_int64_xrninta]            |
| [bid64_to_int8_ceil]                |
| [bid64_to_int8_floor]               |
| [bid64_to_int8_int]                 |
| [bid64_to_int8_rnint]               |
| [bid64_to_int8_rninta]              |
| [bid64_to_int8_xceil]               |
| [bid64_to_int8_xfloor]              |
| [bid64_to_int8_xint]                |
| [bid64_to_int8_xrnint]              |
| [bid64_to_int8_xrninta]             |
| [bid64_to_string]                   |
| [bid64_to_uint16_ceil]              |
| [bid64_to_uint16_floor]             |
| [bid64_to_uint16_int]               |
| [bid64_to_uint16_rnint]             |
| [bid64_to_uint16_rninta]            |
| [bid64_to_uint16_xceil]             |
| [bid64_to_uint16_xfloor]            |
| [bid64_to_uint16_xint]              |
| [bid64_to_uint16_xrnint]            |
| [bid64_to_uint16_xrninta]           |
| [bid64_to_uint32_ceil]              |
| [bid64_to_uint32_floor]             |
| [bid64_to_uint32_int]               |
| [bid64_to_uint32_rnint]             |
| [bid64_to_uint32_rninta]            |
| [bid64_to_uint32_xceil]             |
| [bid64_to_uint32_xfloor]            |
| [bid64_to_uint32_xint]              |
| [bid64_to_uint32_xrnint]            |
| [bid64_to_uint32_xrninta]           |
| [bid64_to_uint64_ceil]              |
| [bid64_to_uint64_floor]             |
| [bid64_to_uint64_int]               |
| [bid64_to_uint64_rnint]             |
| [bid64_to_uint64_rninta]            |
| [bid64_to_uint64_xceil]             |
| [bid64_to_uint64_xfloor]            |
| [bid64_to_uint64_xint]              |
| [bid64_to_uint64_xrnint]            |
| [bid64_to_uint64_xrninta]           |
| [bid64_to_uint8_ceil]               |
| [bid64_to_uint8_floor]              |
| [bid64_to_uint8_int]                |
| [bid64_to_uint8_rnint]              |
| [bid64_to_uint8_rninta]             |
| [bid64_to_uint8_xceil]              |
| [bid64_to_uint8_xfloor]             |
| [bid64_to_uint8_xint]               |
| [bid64_to_uint8_xrnint]             |
| [bid64_to_uint8_xrninta]            |
| [bid64_total_order]                 |
| [bid64_total_order_mag]             |

[bid64_abs]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_abs.html
[bid64_acos]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_acos.html
[bid64_acosh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_acosh.html
[bid64_add]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_add.html
[bid64_add_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_add_dq.html
[bid64_add_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_add_qd.html
[bid64_add_qq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_add_qq.html
[bid64_asin]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_asin.html
[bid64_asinh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_asinh.html
[bid64_atan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_atan.html
[bid64_atan2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_atan2.html
[bid64_atanh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_atanh.html
[bid64_cbrt]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_cbrt.html
[bid64_class]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_class.html
[bid64_copy]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_copy.html
[bid64_copy_sign]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_copy_sign.html
[bid64_cos]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_cos.html
[bid64_cosh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_cosh.html
[bid64_div]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_div.html
[bid64_div_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_div_dq.html
[bid64_div_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_div_qd.html
[bid64_div_qq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_div_qq.html
[bid64_erf]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_erf.html
[bid64_erfc]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_erfc.html
[bid64_exp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_exp.html
[bid64_exp10]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_exp10.html
[bid64_exp2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_exp2.html
[bid64_expm1]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_expm1.html
[bid64_fdim]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fdim.html
[bid64_fma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma.html
[bid64_fma_ddq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_ddq.html
[bid64_fma_dqd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_dqd.html
[bid64_fma_dqq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_dqq.html
[bid64_fma_qdd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_qdd.html
[bid64_fma_qdq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_qdq.html
[bid64_fma_qqd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_qqd.html
[bid64_fma_qqq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fma_qqq.html
[bid64_fmod]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_fmod.html
[bid64_frexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_frexp.html
[bid64_from_int32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_from_int32.html
[bid64_from_int64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_from_int64.html
[bid64_from_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_from_string.html
[bid64_from_uint32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_from_uint32.html
[bid64_from_uint64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_from_uint64.html
[bid64_hypot]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_hypot.html
[bid64_ilogb]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_ilogb.html
[bid64_infinite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_infinite.html
[bid64_is_canonical]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_canonical.html
[bid64_is_finite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_finite.html
[bid64_is_infinite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_infinite.html
[bid64_is_nan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_nan.html
[bid64_is_normal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_normal.html
[bid64_is_signaling]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_signaling.html
[bid64_is_signed]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_signed.html
[bid64_is_subnormal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_subnormal.html
[bid64_is_zero]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_is_zero.html
[bid64_ldexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_ldexp.html
[bid64_lgamma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_lgamma.html
[bid64_llquantexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_llquantexp.html
[bid64_llrint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_llrint.html
[bid64_llround]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_llround.html
[bid64_log]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_log.html
[bid64_log10]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_log10.html
[bid64_log1p]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_log1p.html
[bid64_log2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_log2.html
[bid64_logb]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_logb.html
[bid64_lrint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_lrint.html
[bid64_lround]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_lround.html
[bid64_max_num]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_max_num.html
[bid64_max_num_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_max_num_mag.html
[bid64_min_num]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_min_num.html
[bid64_min_num_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_min_num_mag.html
[bid64_modf]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_modf.html
[bid64_mul]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_mul.html
[bid64_mul_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_mul_dq.html
[bid64_mul_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_mul_qd.html
[bid64_mul_qq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_mul_qq.html
[bid64_nan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_nan.html
[bid64_nearbyint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_nearbyint.html
[bid64_negate]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_negate.html
[bid64_nextafter]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_nextafter.html
[bid64_nextdown]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_nextdown.html
[bid64_nexttoward]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_nexttoward.html
[bid64_nextup]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_nextup.html
[bid64_pow]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_pow.html
[bid64_quantexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quantexp.html
[bid64_quantize]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quantize.html
[bid64_quantum]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quantum.html
[bid64_quiet_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_equal.html
[bid64_quiet_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_greater.html
[bid64_quiet_greater_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_greater_equal.html
[bid64_quiet_greater_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_greater_unordered.html
[bid64_quiet_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_less.html
[bid64_quiet_less_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_less_equal.html
[bid64_quiet_less_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_less_unordered.html
[bid64_quiet_not_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_not_equal.html
[bid64_quiet_not_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_not_greater.html
[bid64_quiet_not_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_not_less.html
[bid64_quiet_ordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_ordered.html
[bid64_quiet_to_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_to_string.html
[bid64_quiet_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_quiet_unordered.html
[bid64_radix]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_radix.html
[bid64_rem]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_rem.html
[bid64_round_integral_exact]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_round_integral_exact.html
[bid64_round_integral_nearest_away]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_round_integral_nearest_away.html
[bid64_round_integral_nearest_even]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_round_integral_nearest_even.html
[bid64_round_integral_negative]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_round_integral_negative.html
[bid64_round_integral_positive]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_round_integral_positive.html
[bid64_round_integral_zero]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_round_integral_zero.html
[bid64_same_quantum]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_same_quantum.html
[bid64_scalbln]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_scalbln.html
[bid64_scalbn]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_scalbn.html
[bid64_signaling_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_greater.html
[bid64_signaling_greater_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_greater_equal.html
[bid64_signaling_greater_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_greater_unordered.html
[bid64_signaling_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_less.html
[bid64_signaling_less_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_less_equal.html
[bid64_signaling_less_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_less_unordered.html
[bid64_signaling_not_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_not_greater.html
[bid64_signaling_not_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_signaling_not_less.html
[bid64_sin]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sin.html
[bid64_sinh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sinh.html
[bid64_sqrt]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sqrt.html
[bid64_sub]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sub.html
[bid64_sub_dq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sub_dq.html
[bid64_sub_qd]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sub_qd.html
[bid64_sub_qq]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_sub_qq.html
[bid64_tan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_tan.html
[bid64_tanh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_tanh.html
[bid64_tgamma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_tgamma.html
[bid64_to_bid128]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_bid128.html
[bid64_to_bid32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_bid32.html
[bid64_to_binary32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_binary32.html
[bid64_to_binary64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_binary64.html
[bid64_to_int16_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_ceil.html
[bid64_to_int16_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_floor.html
[bid64_to_int16_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_int.html
[bid64_to_int16_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_rnint.html
[bid64_to_int16_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_rninta.html
[bid64_to_int16_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_xceil.html
[bid64_to_int16_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_xfloor.html
[bid64_to_int16_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_xint.html
[bid64_to_int16_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_xrnint.html
[bid64_to_int16_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int16_xrninta.html
[bid64_to_int32_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_ceil.html
[bid64_to_int32_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_floor.html
[bid64_to_int32_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_int.html
[bid64_to_int32_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_rnint.html
[bid64_to_int32_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_rninta.html
[bid64_to_int32_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_xceil.html
[bid64_to_int32_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_xfloor.html
[bid64_to_int32_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_xint.html
[bid64_to_int32_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_xrnint.html
[bid64_to_int32_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int32_xrninta.html
[bid64_to_int64_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_ceil.html
[bid64_to_int64_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_floor.html
[bid64_to_int64_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_int.html
[bid64_to_int64_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_rnint.html
[bid64_to_int64_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_rninta.html
[bid64_to_int64_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_xceil.html
[bid64_to_int64_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_xfloor.html
[bid64_to_int64_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_xint.html
[bid64_to_int64_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_xrnint.html
[bid64_to_int64_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int64_xrninta.html
[bid64_to_int8_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_ceil.html
[bid64_to_int8_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_floor.html
[bid64_to_int8_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_int.html
[bid64_to_int8_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_rnint.html
[bid64_to_int8_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_rninta.html
[bid64_to_int8_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_xceil.html
[bid64_to_int8_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_xfloor.html
[bid64_to_int8_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_xint.html
[bid64_to_int8_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_xrnint.html
[bid64_to_int8_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_int8_xrninta.html
[bid64_to_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_string.html
[bid64_to_uint16_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_ceil.html
[bid64_to_uint16_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_floor.html
[bid64_to_uint16_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_int.html
[bid64_to_uint16_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_rnint.html
[bid64_to_uint16_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_rninta.html
[bid64_to_uint16_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_xceil.html
[bid64_to_uint16_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_xfloor.html
[bid64_to_uint16_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_xint.html
[bid64_to_uint16_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_xrnint.html
[bid64_to_uint16_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint16_xrninta.html
[bid64_to_uint32_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_ceil.html
[bid64_to_uint32_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_floor.html
[bid64_to_uint32_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_int.html
[bid64_to_uint32_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_rnint.html
[bid64_to_uint32_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_rninta.html
[bid64_to_uint32_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_xceil.html
[bid64_to_uint32_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_xfloor.html
[bid64_to_uint32_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_xint.html
[bid64_to_uint32_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_xrnint.html
[bid64_to_uint32_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint32_xrninta.html
[bid64_to_uint64_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_ceil.html
[bid64_to_uint64_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_floor.html
[bid64_to_uint64_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_int.html
[bid64_to_uint64_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_rnint.html
[bid64_to_uint64_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_rninta.html
[bid64_to_uint64_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_xceil.html
[bid64_to_uint64_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_xfloor.html
[bid64_to_uint64_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_xint.html
[bid64_to_uint64_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_xrnint.html
[bid64_to_uint64_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint64_xrninta.html
[bid64_to_uint8_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_ceil.html
[bid64_to_uint8_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_floor.html
[bid64_to_uint8_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_int.html
[bid64_to_uint8_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_rnint.html
[bid64_to_uint8_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_rninta.html
[bid64_to_uint8_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_xceil.html
[bid64_to_uint8_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_xfloor.html
[bid64_to_uint8_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_xint.html
[bid64_to_uint8_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_xrnint.html
[bid64_to_uint8_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_to_uint8_xrninta.html
[bid64_total_order]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_total_order.html
[bid64_total_order_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.bid64_total_order_mag.html

| 32-bit bindings                     |
|-------------------------------------|
| [bid32_abs]                         |
| [bid32_acos]                        |
| [bid32_acosh]                       |
| [bid32_add]                         |
| [bid32_asin]                        |
| [bid32_asinh]                       |
| [bid32_atan]                        |
| [bid32_atan2]                       |
| [bid32_atanh]                       |
| [bid32_cbrt]                        |
| [bid32_class]                       |
| [bid32_copy]                        |
| [bid32_copy_sign]                   |
| [bid32_cos]                         |
| [bid32_cosh]                        |
| [bid32_div]                         |
| [bid32_erf]                         |
| [bid32_erfc]                        |
| [bid32_exp]                         |
| [bid32_exp10]                       |
| [bid32_exp2]                        |
| [bid32_expm1]                       |
| [bid32_fdim]                        |
| [bid32_fma]                         |
| [bid32_fmod]                        |
| [bid32_frexp]                       |
| [bid32_from_int32]                  |
| [bid32_from_int64]                  |
| [bid32_from_string]                 |
| [bid32_from_uint32]                 |
| [bid32_from_uint64]                 |
| [bid32_hypot]                       |
| [bid32_ilogb]                       |
| [bid32_infinite]                    |
| [bid32_is_canonical]                |
| [bid32_is_finite]                   |
| [bid32_is_infinite]                 |
| [bid32_is_nan]                      |
| [bid32_is_normal]                   |
| [bid32_is_signaling]                |
| [bid32_is_signed]                   |
| [bid32_is_subnormal]                |
| [bid32_is_zero]                     |
| [bid32_ldexp]                       |
| [bid32_lgamma]                      |
| [bid32_llquantexp]                  |
| [bid32_llrint]                      |
| [bid32_llround]                     |
| [bid32_log]                         |
| [bid32_log10]                       |
| [bid32_log1p]                       |
| [bid32_log2]                        |
| [bid32_logb]                        |
| [bid32_lrint]                       |
| [bid32_lround]                      |
| [bid32_max_num]                     |
| [bid32_max_num_mag]                 |
| [bid32_min_num]                     |
| [bid32_min_num_mag]                 |
| [bid32_modf]                        |
| [bid32_mul]                         |
| [bid32_nan]                         |
| [bid32_nearbyint]                   |
| [bid32_negate]                      |
| [bid32_nextafter]                   |
| [bid32_nextdown]                    |
| [bid32_nexttoward]                  |
| [bid32_nextup]                      |
| [bid32_pow]                         |
| [bid32_quantexp]                    |
| [bid32_quantize]                    |
| [bid32_quantum]                     |
| [bid32_quiet_equal]                 |
| [bid32_quiet_greater]               |
| [bid32_quiet_greater_equal]         |
| [bid32_quiet_greater_unordered]     |
| [bid32_quiet_less]                  |
| [bid32_quiet_less_equal]            |
| [bid32_quiet_less_unordered]        |
| [bid32_quiet_not_equal]             |
| [bid32_quiet_not_greater]           |
| [bid32_quiet_not_less]              |
| [bid32_quiet_ordered]               |
| [bid32_quiet_to_string]             |
| [bid32_quiet_unordered]             |
| [bid32_radix]                       |
| [bid32_rem]                         |
| [bid32_round_integral_exact]        |
| [bid32_round_integral_nearest_away] |
| [bid32_round_integral_nearest_even] |
| [bid32_round_integral_negative]     |
| [bid32_round_integral_positive]     |
| [bid32_round_integral_zero]         |
| [bid32_same_quantum]                |
| [bid32_scalbln]                     |
| [bid32_scalbn]                      |
| [bid32_signaling_greater]           |
| [bid32_signaling_greater_equal]     |
| [bid32_signaling_greater_unordered] |
| [bid32_signaling_less]              |
| [bid32_signaling_less_equal]        |
| [bid32_signaling_less_unordered]    |
| [bid32_signaling_not_greater]       |
| [bid32_signaling_not_less]          |
| [bid32_sin]                         |
| [bid32_sinh]                        |
| [bid32_sqrt]                        |
| [bid32_sub]                         |
| [bid32_tan]                         |
| [bid32_tanh]                        |
| [bid32_tgamma]                      |
| [bid32_to_bid128]                   |
| [bid32_to_bid64]                    |
| [bid32_to_binary32]                 |
| [bid32_to_binary64]                 |
| [bid32_to_int16_ceil]               |
| [bid32_to_int16_floor]              |
| [bid32_to_int16_int]                |
| [bid32_to_int16_rnint]              |
| [bid32_to_int16_rninta]             |
| [bid32_to_int16_xceil]              |
| [bid32_to_int16_xfloor]             |
| [bid32_to_int16_xint]               |
| [bid32_to_int16_xrnint]             |
| [bid32_to_int16_xrninta]            |
| [bid32_to_int32_ceil]               |
| [bid32_to_int32_floor]              |
| [bid32_to_int32_int]                |
| [bid32_to_int32_rnint]              |
| [bid32_to_int32_rninta]             |
| [bid32_to_int32_xceil]              |
| [bid32_to_int32_xfloor]             |
| [bid32_to_int32_xint]               |
| [bid32_to_int32_xrnint]             |
| [bid32_to_int32_xrninta]            |
| [bid32_to_int64_ceil]               |
| [bid32_to_int64_floor]              |
| [bid32_to_int64_int]                |
| [bid32_to_int64_rnint]              |
| [bid32_to_int64_rninta]             |
| [bid32_to_int64_xceil]              |
| [bid32_to_int64_xfloor]             |
| [bid32_to_int64_xint]               |
| [bid32_to_int64_xrnint]             |
| [bid32_to_int64_xrninta]            |
| [bid32_to_int8_ceil]                |
| [bid32_to_int8_floor]               |
| [bid32_to_int8_int]                 |
| [bid32_to_int8_rnint]               |
| [bid32_to_int8_rninta]              |
| [bid32_to_int8_xceil]               |
| [bid32_to_int8_xfloor]              |
| [bid32_to_int8_xint]                |
| [bid32_to_int8_xrnint]              |
| [bid32_to_int8_xrninta]             |
| [bid32_to_string]                   |
| [bid32_to_uint16_ceil]              |
| [bid32_to_uint16_floor]             |
| [bid32_to_uint16_int]               |
| [bid32_to_uint16_rnint]             |
| [bid32_to_uint16_rninta]            |
| [bid32_to_uint16_xceil]             |
| [bid32_to_uint16_xfloor]            |
| [bid32_to_uint16_xint]              |
| [bid32_to_uint16_xrnint]            |
| [bid32_to_uint16_xrninta]           |
| [bid32_to_uint32_ceil]              |
| [bid32_to_uint32_floor]             |
| [bid32_to_uint32_int]               |
| [bid32_to_uint32_rnint]             |
| [bid32_to_uint32_rninta]            |
| [bid32_to_uint32_xceil]             |
| [bid32_to_uint32_xfloor]            |
| [bid32_to_uint32_xint]              |
| [bid32_to_uint32_xrnint]            |
| [bid32_to_uint32_xrninta]           |
| [bid32_to_uint64_ceil]              |
| [bid32_to_uint64_floor]             |
| [bid32_to_uint64_int]               |
| [bid32_to_uint64_rnint]             |
| [bid32_to_uint64_rninta]            |
| [bid32_to_uint64_xceil]             |
| [bid32_to_uint64_xfloor]            |
| [bid32_to_uint64_xint]              |
| [bid32_to_uint64_xrnint]            |
| [bid32_to_uint64_xrninta]           |
| [bid32_to_uint8_ceil]               |
| [bid32_to_uint8_floor]              |
| [bid32_to_uint8_int]                |
| [bid32_to_uint8_rnint]              |
| [bid32_to_uint8_rninta]             |
| [bid32_to_uint8_xceil]              |
| [bid32_to_uint8_xfloor]             |
| [bid32_to_uint8_xint]               |
| [bid32_to_uint8_xrnint]             |
| [bid32_to_uint8_xrninta]            |
| [bid32_total_order]                 |
| [bid32_total_order_mag]             |

[bid32_abs]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_abs.html
[bid32_acos]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_acos.html
[bid32_acosh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_acosh.html
[bid32_add]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_add.html
[bid32_asin]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_asin.html
[bid32_asinh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_asinh.html
[bid32_atan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_atan.html
[bid32_atan2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_atan2.html
[bid32_atanh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_atanh.html
[bid32_cbrt]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_cbrt.html
[bid32_class]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_class.html
[bid32_copy]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_copy.html
[bid32_copy_sign]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_copy_sign.html
[bid32_cos]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_cos.html
[bid32_cosh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_cosh.html
[bid32_div]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_div.html
[bid32_erf]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_erf.html
[bid32_erfc]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_erfc.html
[bid32_exp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_exp.html
[bid32_exp10]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_exp10.html
[bid32_exp2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_exp2.html
[bid32_expm1]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_expm1.html
[bid32_fdim]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_fdim.html
[bid32_fma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_fma.html
[bid32_fmod]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_fmod.html
[bid32_frexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_frexp.html
[bid32_from_int32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_from_int32.html
[bid32_from_int64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_from_int64.html
[bid32_from_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_from_string.html
[bid32_from_uint32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_from_uint32.html
[bid32_from_uint64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_from_uint64.html
[bid32_hypot]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_hypot.html
[bid32_ilogb]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_ilogb.html
[bid32_infinite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_infinite.html
[bid32_is_canonical]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_canonical.html
[bid32_is_finite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_finite.html
[bid32_is_infinite]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_infinite.html
[bid32_is_nan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_nan.html
[bid32_is_normal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_normal.html
[bid32_is_signaling]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_signaling.html
[bid32_is_signed]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_signed.html
[bid32_is_subnormal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_subnormal.html
[bid32_is_zero]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_is_zero.html
[bid32_ldexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_ldexp.html
[bid32_lgamma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_lgamma.html
[bid32_llquantexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_llquantexp.html
[bid32_llrint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_llrint.html
[bid32_llround]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_llround.html
[bid32_log]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_log.html
[bid32_log10]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_log10.html
[bid32_log1p]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_log1p.html
[bid32_log2]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_log2.html
[bid32_logb]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_logb.html
[bid32_lrint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_lrint.html
[bid32_lround]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_lround.html
[bid32_max_num]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_max_num.html
[bid32_max_num_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_max_num_mag.html
[bid32_min_num]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_min_num.html
[bid32_min_num_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_min_num_mag.html
[bid32_modf]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_modf.html
[bid32_mul]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_mul.html
[bid32_nan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_nan.html
[bid32_nearbyint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_nearbyint.html
[bid32_negate]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_negate.html
[bid32_nextafter]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_nextafter.html
[bid32_nextdown]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_nextdown.html
[bid32_nexttoward]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_nexttoward.html
[bid32_nextup]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_nextup.html
[bid32_pow]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_pow.html
[bid32_quantexp]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quantexp.html
[bid32_quantize]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quantize.html
[bid32_quantum]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quantum.html
[bid32_quiet_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_equal.html
[bid32_quiet_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_greater.html
[bid32_quiet_greater_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_greater_equal.html
[bid32_quiet_greater_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_greater_unordered.html
[bid32_quiet_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_less.html
[bid32_quiet_less_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_less_equal.html
[bid32_quiet_less_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_less_unordered.html
[bid32_quiet_not_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_not_equal.html
[bid32_quiet_not_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_not_greater.html
[bid32_quiet_not_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_not_less.html
[bid32_quiet_ordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_ordered.html
[bid32_quiet_to_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_to_string.html
[bid32_quiet_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_quiet_unordered.html
[bid32_radix]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_radix.html
[bid32_rem]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_rem.html
[bid32_round_integral_exact]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_round_integral_exact.html
[bid32_round_integral_nearest_away]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_round_integral_nearest_away.html
[bid32_round_integral_nearest_even]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_round_integral_nearest_even.html
[bid32_round_integral_negative]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_round_integral_negative.html
[bid32_round_integral_positive]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_round_integral_positive.html
[bid32_round_integral_zero]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_round_integral_zero.html
[bid32_same_quantum]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_same_quantum.html
[bid32_scalbln]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_scalbln.html
[bid32_scalbn]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_scalbn.html
[bid32_signaling_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_greater.html
[bid32_signaling_greater_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_greater_equal.html
[bid32_signaling_greater_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_greater_unordered.html
[bid32_signaling_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_less.html
[bid32_signaling_less_equal]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_less_equal.html
[bid32_signaling_less_unordered]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_less_unordered.html
[bid32_signaling_not_greater]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_not_greater.html
[bid32_signaling_not_less]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_signaling_not_less.html
[bid32_sin]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_sin.html
[bid32_sinh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_sinh.html
[bid32_sqrt]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_sqrt.html
[bid32_sub]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_sub.html
[bid32_tan]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_tan.html
[bid32_tanh]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_tanh.html
[bid32_tgamma]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_tgamma.html
[bid32_to_bid128]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_bid128.html
[bid32_to_bid64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_bid64.html
[bid32_to_binary32]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_binary32.html
[bid32_to_binary64]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_binary64.html
[bid32_to_int16_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_ceil.html
[bid32_to_int16_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_floor.html
[bid32_to_int16_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_int.html
[bid32_to_int16_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_rnint.html
[bid32_to_int16_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_rninta.html
[bid32_to_int16_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_xceil.html
[bid32_to_int16_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_xfloor.html
[bid32_to_int16_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_xint.html
[bid32_to_int16_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_xrnint.html
[bid32_to_int16_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int16_xrninta.html
[bid32_to_int32_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_ceil.html
[bid32_to_int32_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_floor.html
[bid32_to_int32_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_int.html
[bid32_to_int32_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_rnint.html
[bid32_to_int32_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_rninta.html
[bid32_to_int32_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_xceil.html
[bid32_to_int32_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_xfloor.html
[bid32_to_int32_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_xint.html
[bid32_to_int32_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_xrnint.html
[bid32_to_int32_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int32_xrninta.html
[bid32_to_int64_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_ceil.html
[bid32_to_int64_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_floor.html
[bid32_to_int64_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_int.html
[bid32_to_int64_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_rnint.html
[bid32_to_int64_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_rninta.html
[bid32_to_int64_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_xceil.html
[bid32_to_int64_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_xfloor.html
[bid32_to_int64_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_xint.html
[bid32_to_int64_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_xrnint.html
[bid32_to_int64_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int64_xrninta.html
[bid32_to_int8_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_ceil.html
[bid32_to_int8_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_floor.html
[bid32_to_int8_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_int.html
[bid32_to_int8_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_rnint.html
[bid32_to_int8_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_rninta.html
[bid32_to_int8_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_xceil.html
[bid32_to_int8_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_xfloor.html
[bid32_to_int8_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_xint.html
[bid32_to_int8_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_xrnint.html
[bid32_to_int8_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_int8_xrninta.html
[bid32_to_string]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_string.html
[bid32_to_uint16_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_ceil.html
[bid32_to_uint16_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_floor.html
[bid32_to_uint16_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_int.html
[bid32_to_uint16_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_rnint.html
[bid32_to_uint16_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_rninta.html
[bid32_to_uint16_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_xceil.html
[bid32_to_uint16_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_xfloor.html
[bid32_to_uint16_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_xint.html
[bid32_to_uint16_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_xrnint.html
[bid32_to_uint16_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint16_xrninta.html
[bid32_to_uint32_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_ceil.html
[bid32_to_uint32_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_floor.html
[bid32_to_uint32_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_int.html
[bid32_to_uint32_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_rnint.html
[bid32_to_uint32_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_rninta.html
[bid32_to_uint32_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_xceil.html
[bid32_to_uint32_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_xfloor.html
[bid32_to_uint32_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_xint.html
[bid32_to_uint32_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_xrnint.html
[bid32_to_uint32_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint32_xrninta.html
[bid32_to_uint64_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_ceil.html
[bid32_to_uint64_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_floor.html
[bid32_to_uint64_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_int.html
[bid32_to_uint64_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_rnint.html
[bid32_to_uint64_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_rninta.html
[bid32_to_uint64_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_xceil.html
[bid32_to_uint64_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_xfloor.html
[bid32_to_uint64_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_xint.html
[bid32_to_uint64_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_xrnint.html
[bid32_to_uint64_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint64_xrninta.html
[bid32_to_uint8_ceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_ceil.html
[bid32_to_uint8_floor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_floor.html
[bid32_to_uint8_int]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_int.html
[bid32_to_uint8_rnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_rnint.html
[bid32_to_uint8_rninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_rninta.html
[bid32_to_uint8_xceil]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_xceil.html
[bid32_to_uint8_xfloor]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_xfloor.html
[bid32_to_uint8_xint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_xint.html
[bid32_to_uint8_xrnint]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_xrnint.html
[bid32_to_uint8_xrninta]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_to_uint8_xrninta.html
[bid32_total_order]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_total_order.html
[bid32_total_order_mag]: https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.bid32_total_order_mag.html

## License

### License for Intel® Decimal Floating-Point Math Library v2.3

- [EULA](IntelRDFPMathLib20U3/eula.txt)

### License for Rust bindings

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE](LICENSE))
  at your option.

## Contribution

Any contributions to [**dfp-number-sys**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
