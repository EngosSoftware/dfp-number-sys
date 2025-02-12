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

use crate::{ExcFlags, RndMode, BID32};
use libc::{c_char, c_uint};
use std::ffi::{CStr, CString};

extern "C" {
  fn __bid32_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID32;
  fn __bid32_to_string(s: *mut c_char, x: BID32, flags: *mut c_uint);
}

/// Converts a number represented as string format (decimal character sequence)
/// to 32-bit decimal floating-point format (binary encoding).
pub fn bid32_from_string(s: &str, round: RndMode, flags: &mut ExcFlags) -> BID32 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid32_from_string(c_s.as_ptr(), round, flags) }
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
