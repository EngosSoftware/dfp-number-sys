use std::fmt::Write;
use std::fs;

const OUTPUT_FILE: &str = "README.txt";
const BID128_FUNCTIONS_FILE: &str = "./tests/generator/bid128-functions.txt";
const BID64_FUNCTIONS_FILE: &str = "./tests/generator/bid64-functions.txt";
const BID32_FUNCTIONS_FILE: &str = "./tests/generator/bid32-functions.txt";
const BID128_DOC_LINK: &str = "https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid128_000/fn.#FUNCTION#.html";
const BID64_DOC_LINK: &str = "https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid64_000/fn.#FUNCTION#.html";
const BID32_DOC_LINK: &str = "https://docs.rs/dfp-number-sys/latest/dfp_number_sys/bid32_000/fn.#FUNCTION#.html";

#[test]
fn generate_readme_content() {
  let mut buffer = String::new();
  generator(&mut buffer, BID128_FUNCTIONS_FILE, "128-bit bindings", BID128_DOC_LINK);
  generator(&mut buffer, BID64_FUNCTIONS_FILE, "64-bit bindings", BID64_DOC_LINK);
  generator(&mut buffer, BID32_FUNCTIONS_FILE, "32-bit bindings", BID32_DOC_LINK);
  fs::write(OUTPUT_FILE, buffer).expect("failed to write output file");
}

fn generator(w: &mut impl Write, file_name: &str, title: &str, doc_link: &str) {
  // Load function names from file.
  let function_names = fs::read_to_string(file_name)
    .expect("failed to load input file")
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| line.trim().to_string())
    .collect::<Vec<String>>();

  // Sort function names alphabetically.
  let mut sorted_function_names = function_names.clone();
  sorted_function_names.sort();

  // Check if the order is the same in the input file.
  let zipped = function_names.iter().zip(sorted_function_names.iter());
  for (name, sorted_name) in zipped {
    assert_eq!(name, sorted_name);
  }
  assert_eq!(function_names, sorted_function_names);

  // Calculate the maximum function name length.
  let max_width = sorted_function_names.iter().map(|name| name.len()).max().unwrap();

  // Generate Markdown table with function names.
  let _ = writeln!(w, "| {}{} |", title, " ".repeat(max_width - title.len() + 2));
  let _ = writeln!(w, "|{}|", "-".repeat(max_width + 4));
  for function_name in &sorted_function_names {
    let fill = " ".repeat(max_width - function_name.len());
    let _ = writeln!(w, "| [{}]{} |", function_name, fill);
  }

  let _ = writeln!(w);

  // Generate documentation links.
  for function_name in &sorted_function_names {
    let _ = writeln!(w, "[{}]: {}", function_name, doc_link.replace("#FUNCTION#", function_name));
  }

  let _ = writeln!(w);
}
