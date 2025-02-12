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

use crate::{ExcFlags, RndMode, BID64};
use libc::{c_char, c_uint};
use std::ffi::{CStr, CString};

extern "C" {
  fn __bid64_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID64;
  fn __bid64_to_string(s: *mut c_char, x: BID64, flags: *mut c_uint);
}

/// Converts a number represented as string format (decimal character sequence)
/// to 64-bit decimal floating-point format (binary encoding).
pub fn bid64_from_string(s: &str, round: RndMode, flags: &mut ExcFlags) -> BID64 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid64_from_string(c_s.as_ptr(), round, flags) }
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
