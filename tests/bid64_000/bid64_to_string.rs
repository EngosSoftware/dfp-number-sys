use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!("+2E+0", bid64_to_string(d64("2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!("-12345E-2", bid64_to_string(d64("-123.45"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert_eq!("-12345E-2", bid64_to_string(d64("-12345e-2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let inputs = ["inf", "Inf", "INF", "+inf", "infinity", "+infinity", "Infinity", "+Infinity"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    assert_eq!("+Inf", bid64_to_string(d64(input), &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0005() {
  let inputs = ["-inf", "-Inf", "-INF", "-infinity", "-Infinity"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    assert_eq!("-Inf", bid64_to_string(d64(input), &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0006() {
  let inputs = ["nan", "+nan", "NaN", "+NaN", "qNaN", "+qNaN", "+QNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    assert_eq!("+NaN", bid64_to_string(d64(input), &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0007() {
  let inputs = ["-nan", "-NaN", "-qnan", "-QNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    assert_eq!("-NaN", bid64_to_string(d64(input), &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0008() {
  let inputs = ["snan", "+snan", "sNaN", "+sNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    assert_eq!("+SNaN", bid64_to_string(d64(input), &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}

#[test]
fn _0009() {
  let inputs = ["-snan", "-sNaN", "-sNaN"];
  for input in inputs {
    let mut flags = EXE_CLEAR;
    assert_eq!("-SNaN", bid64_to_string(d64(input), &mut flags));
    eqf(EXE_CLEAR, flags);
  }
}
