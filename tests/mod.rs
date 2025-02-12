//! # Common definitions for all tests

mod bid128_000;
mod bid32_000;
mod bid64_000;

macro_rules! flags {
  () => {
    &mut EXE_CLEAR.clone()
  };
}

use flags;

use dfp_number_sys::bid128_000::*;
use dfp_number_sys::bid32_000::*;
use dfp_number_sys::bid64_000::*;
use dfp_number_sys::*;

fn d128(s: &str) -> BID128 {
  let mut flags = EXE_CLEAR;
  let x = bid128_from_string(s, RND_NEAREST_EVEN, &mut flags);
  assert_eq!(EXE_CLEAR, flags);
  x
}

fn d64(s: &str) -> BID64 {
  let mut flags = EXE_CLEAR;
  let x = bid64_from_string(s, RND_NEAREST_EVEN, &mut flags);
  assert_eq!(EXE_CLEAR, flags);
  x
}

fn d32(s: &str) -> BID32 {
  let mut flags = EXE_CLEAR;
  let x = bid32_from_string(s, RND_NEAREST_EVEN, &mut flags);
  assert_eq!(EXE_CLEAR, flags);
  x
}
