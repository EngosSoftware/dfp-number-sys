//! # Common definitions for all tests

mod bid128_000;

macro_rules! flags {
  () => {
    &mut FB_CLEAR.clone()
  };
}

use flags;
