# Rust bindings for Intel® Decimal Floating-Point Math Library v2.2

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
[eula-url]: IntelRDFPMathLib20U2/eula.txt
[build-badge-linux]: https://github.com/wisbery/dfp-number-sys/actions/workflows/build-linux-ix86-64.yml/badge.svg
[build-badge-windows]: https://github.com/wisbery/dfp-number-sys/actions/workflows/build-windows-ix86-64.yml/badge.svg
[build-badge-macos]: https://github.com/wisbery/dfp-number-sys/actions/workflows/build-macos-ix86-64.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/wisbery/dfp-number-sys/actions/workflows/build-macos-arm64.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[coc-url]: CODE_OF_CONDUCT.md

## Implemented bindings

| Rust binding name                  | Library function name                 |
|------------------------------------|---------------------------------------|
| bid128_abs                         | __bid128_abs                          |                        
| bid128_add                         | __bid128_add                          |                        
| bid128_copy                        | __bid128_copy                         |                                                                      
| bid128_div                         | __bid128_div                          |                                                                         
| bid128_exp                         | __bid128_exp                          |                                                                         
| bid128_frexp                       | __bid128_frexp                        |                                                                   
| bid128_from_int32                  | __bid128_from_int32                   |                                                    
| bid128_from_int64                  | __bid128_from_int64                   |                                                    
| bid128_from_string                 | __bid128_from_string                  |                                                 
| bid128_from_uint32                 | __bid128_from_uint32                  |                                                 
| bid128_from_uint64                 | __bid128_from_uint64                  |                                                 
| bid128_ilogb                       | __bid128_ilogb                        |                                                                   
| bid128_isFinite                    | __bid128_isFinite                     |                                                          
| bid128_inf                         | __bid128_inf                          |                                                                         
| bid128_isInf                       | __bid128_isInf                        |                                                                   
| bid128_isSigned                    | __bid128_isSigned                     |                                                          
| bid128_isZero                      | __bid128_isZero                       |                                                                
| bid128_log                         | __bid128_log                          |                                                                         
| bid128_maxnum                      | __bid128_maxnum                       |                                                                
| bid128_minnum                      | __bid128_minnum                       |                                                                
| bid128_negate                      | __bid128_negate                       |                                                                
| bid128_mul                         | __bid128_mul                          |                                                                         
| bid128_pow                         | __bid128_pow                          |                                                                         
| bid128_quantexp                    | __bid128_quantexp                     |                                                          
| bid128_quantum                     | __bid128_quantum                      |                                                             
| bid128_quantize                    | __bid128_quantize                     |                                                          
| bid128_quiet_equal                 | __bid128_quiet_equal                  |                                                 
| bid128_quiet_greater               | __bid128_quiet_greater                |                                           
| bid128_quiet_greater_equal         | __bid128_quiet_greater_equal          |                         
| bid128_quiet_less                  | __bid128_quiet_less                   |                                                    
| bid128_quiet_less_equal            | __bid128_quiet_less_equal             |                                  
| bid128_rem                         | __bid128_rem                          |                                                                         
| bid128_round_integral_exact        | __bid128_round_integral_exact         |                      
| bid128_round_integral_nearest_away | __bid128_round_integral_nearest_away  | 
| bid128_round_integral_nearest_even | __bid128_round_integral_nearest_even  | 
| bid128_round_integral_negative     | __bid128_round_integral_negative      |     
| bid128_round_integral_positive     | __bid128_round_integral_positive      |     
| bid128_round_integral_zero         | __bid128_round_integral_zero          |         
| bid128_scalbn                      | __bid128_scalbn                       |                                    
| bid128_scalbln                     | __bid128_scalbln                      |                                 
| bid128_sqrt                        | __bid128_sqrt                         |                                          
| bid128_sub                         | __bid128_sub                          |                                             
| bid128_to_int32_int                | __bid128_to_int32_int                 |                  
| bid128_to_uint32_int               | __bid128_to_uint32_int                |               
| bid128_to_int64_int                | __bid128_to_int64_int                 |                  
| bid128_to_uint64_int               | __bid128_to_uint64_int                |               
| bid128_to_string                   | __bid128_to_string                    |                   

## Bindings to be implemented

- `__bid128_acos`
- `__bid128_acosh`
- `__bid128_asin`
- `__bid128_asinh`
- `__bid128_atan`
- `__bid128_atan2`
- `__bid128_atanh`
- `__bid128_cbrt`
- `__bid128_class`
- `__bid128_copySign`
- `__bid128_cos`
- `__bid128_cosh`
- `__bid128_erf`
- `__bid128_erfc`
- `__bid128_exp10`
- `__bid128_exp2`
- `__bid128_expm1`
- `__bid128_fdim`
- `__bid128_fma`
- `__bid128_fmod`
- `__bid128_hypot`
- `__bid128_isCanonical`
- `__bid128_isNaN`
- `__bid128_isNormal`
- `__bid128_isSignaling`
- `__bid128_isSigned`
- `__bid128_isSubnormal`
- `__bid128_ldexp`
- `__bid128_lgamma`
- `__bid128_llquantexp`
- `__bid128_llrint`
- `__bid128_llround`
- `__bid128_log10`
- `__bid128_log1p`
- `__bid128_log2`
- `__bid128_logb`
- `__bid128_lrint`
- `__bid128_lround`
- `__bid128_maxnum_mag`
- `__bid128_minnum_mag`
- `__bid128_modf`
- `__bid128_nan`
- `__bid128_nearbyint`
- `__bid128_nextafter`
- `__bid128_nextdown`
- `__bid128_nexttoward`
- `__bid128_nextup`
- `__bid128_quiet_greater_unordered`
- `__bid128_quiet_less_unordered`
- `__bid128_quiet_not_equal`
- `__bid128_quiet_not_greater`
- `__bid128_quiet_not_less`
- `__bid128_quiet_ordered`
- `__bid128_quiet_unordered`
- `__bid128_radix`
- `__bid128_sameQuantum`
- `__bid128_signaling_greater`
- `__bid128_signaling_greater_equal`
- `__bid128_signaling_greater_unordered`
- `__bid128_signaling_less`
- `__bid128_signaling_less_equal`
- `__bid128_signaling_less_unordered`
- `__bid128_signaling_not_greater`
- `__bid128_signaling_not_less`
- `__bid128_sin`
- `__bid128_sinh`
- `__bid128_tan`
- `__bid128_tanh`
- `__bid128_tgamma`
- `__bid128_to_bid32`
- `__bid128_to_bid64`
- `__bid128_to_binary128`
- `__bid128_to_binary32`
- `__bid128_to_binary64`
- `__bid128_to_binary80`
- `__bid128_to_int16_ceil`
- `__bid128_to_int16_floor`
- `__bid128_to_int16_int`
- `__bid128_to_int16_rnint`
- `__bid128_to_int16_rninta`
- `__bid128_to_int16_xceil`
- `__bid128_to_int16_xfloor`
- `__bid128_to_int16_xint`
- `__bid128_to_int16_xrnint`
- `__bid128_to_int16_xrninta`
- `__bid128_to_int32_ceil`
- `__bid128_to_int32_floor`
- `__bid128_to_int32_rnint`
- `__bid128_to_int32_rninta`
- `__bid128_to_int32_xceil`
- `__bid128_to_int32_xfloor`
- `__bid128_to_int32_xint`
- `__bid128_to_int32_xrnint`
- `__bid128_to_int32_xrninta`
- `__bid128_to_int64_ceil`
- `__bid128_to_int64_floor`
- `__bid128_to_int64_rnint`
- `__bid128_to_int64_rninta`
- `__bid128_to_int64_xceil`
- `__bid128_to_int64_xfloor`
- `__bid128_to_int64_xint`
- `__bid128_to_int64_xrnint`
- `__bid128_to_int64_xrninta`
- `__bid128_to_int8_ceil`
- `__bid128_to_int8_floor`
- `__bid128_to_int8_int`
- `__bid128_to_int8_rnint`
- `__bid128_to_int8_rninta`
- `__bid128_to_int8_xceil`
- `__bid128_to_int8_xfloor`
- `__bid128_to_int8_xint`
- `__bid128_to_int8_xrnint`
- `__bid128_to_int8_xrninta`
- `__bid128_to_uint16_ceil`
- `__bid128_to_uint16_floor`
- `__bid128_to_uint16_int`
- `__bid128_to_uint16_rnint`
- `__bid128_to_uint16_rninta`
- `__bid128_to_uint16_xceil`
- `__bid128_to_uint16_xfloor`
- `__bid128_to_uint16_xint`
- `__bid128_to_uint16_xrnint`
- `__bid128_to_uint16_xrninta`
- `__bid128_to_uint32_ceil`
- `__bid128_to_uint32_floor`
- `__bid128_to_uint32_rnint`
- `__bid128_to_uint32_rninta`
- `__bid128_to_uint32_xceil`
- `__bid128_to_uint32_xfloor`
- `__bid128_to_uint32_xint`
- `__bid128_to_uint32_xrnint`
- `__bid128_to_uint32_xrninta`
- `__bid128_to_uint64_ceil`
- `__bid128_to_uint64_floor`
- `__bid128_to_uint64_rnint`
- `__bid128_to_uint64_rninta`
- `__bid128_to_uint64_xceil`
- `__bid128_to_uint64_xfloor`
- `__bid128_to_uint64_xint`
- `__bid128_to_uint64_xrnint`
- `__bid128_to_uint64_xrninta`
- `__bid128_to_uint8_ceil`
- `__bid128_to_uint8_floor`
- `__bid128_to_uint8_int`
- `__bid128_to_uint8_rnint`
- `__bid128_to_uint8_rninta`
- `__bid128_to_uint8_xceil`
- `__bid128_to_uint8_xfloor`
- `__bid128_to_uint8_xint`
- `__bid128_to_uint8_xrnint`
- `__bid128_to_uint8_xrninta`
- `__bid128_totalOrder`
- `__bid128_totalOrderMag`
- `__bid128d_sqrt`
- `__bid128dd_add`
- `__bid128dd_div`
- `__bid128dd_mul`
- `__bid128dd_sub`
- `__bid128ddd_fma`
- `__bid128ddq_fma`
- `__bid128dq_add`
- `__bid128dq_div`
- `__bid128dq_mul`
- `__bid128dq_sub`
- `__bid128dqd_fma`
- `__bid128dqq_fma`
- `__bid128qd_add`
- `__bid128qd_div`
- `__bid128qd_mul`
- `__bid128qd_sub`
- `__bid128qdd_fma`
- `__bid128qdq_fma`
- `__bid128qqd_fma`

## License

### License for Intel® Decimal Floating-Point Math Library v2.2

- [EULA](IntelRDFPMathLib20U2/eula.txt)

### License for Rust bindings

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE](LICENSE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
