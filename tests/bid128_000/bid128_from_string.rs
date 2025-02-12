use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid128_from_string("2", RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("-12345E-2", bid128_from_string("-123.45", RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-12345E-2", bid128_from_string("-12345e-2", RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let inputs = ["inf", "Inf", "INF", "+inf", "infinity", "+infinity", "Infinity", "+Infinity"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    eq("+Inf", bid128_from_string(input, RND_NEAREST_EVEN, &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0005() {
  let inputs = ["-inf", "-Inf", "-INF", "-infinity", "-Infinity"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    eq("-Inf", bid128_from_string(input, RND_NEAREST_EVEN, &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0006() {
  let inputs = ["nan", "+nan", "NaN", "+NaN", "qNaN", "+qNaN", "+QNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    eq("+NaN", bid128_from_string(input, RND_NEAREST_EVEN, &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0007() {
  let inputs = ["-nan", "-NaN", "-qnan", "-QNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    eq("-NaN", bid128_from_string(input, RND_NEAREST_EVEN, &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0008() {
  let inputs = ["snan", "+snan", "sNaN", "+sNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    eq("+SNaN", bid128_from_string(input, RND_NEAREST_EVEN, &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0009() {
  let inputs = ["-snan", "-sNaN", "-sNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    eq("-SNaN", bid128_from_string(input, RND_NEAREST_EVEN, &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}
