use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+2E+0", bid128_from_string("2", RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("-12345E-2", bid128_from_string("-123.45", RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-12345E-2", bid128_from_string("-12345e-2", RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let inputs = ["inf", "Inf", "INF", "+inf", "infinity", "+infinity", "Infinity", "+Infinity"];
  for input in inputs {
    let mut flags = FB_CLEAR;
    eq("+Inf", bid128_from_string(input, RM_NEAREST_EVEN, &mut flags));
    eqf(FB_CLEAR, flags);
  }
}

#[test]
fn _0005() {
  let inputs = ["-inf", "-Inf", "-INF", "-infinity", "-Infinity"];
  for input in inputs {
    let mut flags = FB_CLEAR;
    eq("-Inf", bid128_from_string(input, RM_NEAREST_EVEN, &mut flags));
    eqf(FB_CLEAR, flags);
  }
}

#[test]
fn _0006() {
  let inputs = ["nan", "+nan", "NaN", "+NaN", "qNaN", "+qNaN", "+QNaN"];
  for input in inputs {
    let mut flags = FB_CLEAR;
    eq("+NaN", bid128_from_string(input, RM_NEAREST_EVEN, &mut flags));
    eqf(FB_CLEAR, flags);
  }
}

#[test]
fn _0007() {
  let inputs = ["-nan", "-NaN", "-qnan", "-QNaN"];
  for input in inputs {
    let mut flags = FB_CLEAR;
    eq("-NaN", bid128_from_string(input, RM_NEAREST_EVEN, &mut flags));
    eqf(FB_CLEAR, flags);
  }
}

#[test]
fn _0008() {
  let inputs = ["snan", "+snan", "sNaN", "+sNaN"];
  for input in inputs {
    let mut flags = FB_CLEAR;
    eq("+SNaN", bid128_from_string(input, RM_NEAREST_EVEN, &mut flags));
    eqf(FB_CLEAR, flags);
  }
}

#[test]
fn _0009() {
  let inputs = ["-snan", "-sNaN", "-sNaN"];
  for input in inputs {
    let mut flags = FB_CLEAR;
    eq("-SNaN", bid128_from_string(input, RM_NEAREST_EVEN, &mut flags));
    eqf(FB_CLEAR, flags);
  }
}
