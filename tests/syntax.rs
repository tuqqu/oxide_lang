use oxide::run_file_with_streams;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

const SAMPLE_PATH: &str = "./tests/scripts";
const OUTPUT_PATH: &str = "./tests/output";

#[test]
fn test_operator() {
    test_script("operator");
}

#[test]
fn test_if() {
    test_script("if");
}

#[test]
fn test_match() {
    test_script("match");
}

#[test]
fn test_variable() {
    test_script("variable");
}

#[test]
fn test_constant() {
    test_script("constant");
}

#[test]
fn test_types() {
    test_script("types");
}

#[test]
fn test_while() {
    test_script("while");
}

#[test]
fn test_for() {
    test_script("for");
}

#[test]
fn test_loop() {
    test_script("loop");
}

#[test]
fn test_logic_expr() {
    test_script("logic_expr");
}

#[test]
fn test_scope() {
    test_script("scope");
}

#[test]
fn test_stdlib() {
    test_script("stdlib");
}

#[test]
fn test_comment() {
    test_script("comment");
}

#[test]
fn test_fn() {
    test_script("fn");
}

#[test]
fn test_recursive_fn() {
    test_script("recursive_fn");
}

#[test]
fn test_closure() {
    test_script("closure");
}

#[test]
fn test_lambda() {
    test_script("lambda");
}

#[test]
fn test_struct_properties() {
    test_script("struct_properties");
}

fn test_script(script: &str) {
    let sample_file: String = format!("{}/{}.ox", SAMPLE_PATH, script);
    let output_file: String = format!("{}/{}.output", OUTPUT_PATH, script);

    let stdout = Rc::new(RefCell::new(Vec::<u8>::new()));
    let stderr = Rc::new(RefCell::new(Vec::<u8>::new()));

    let vecout = Rc::clone(&stdout);
    let vecerr = Rc::clone(&stderr);

    run_file_with_streams(sample_file, Some(stdout), Some(vecerr), None);

    let expected = &*vecout.borrow();
    let expected = String::from_utf8_lossy(expected);

    let actual = fs::read_to_string(&output_file).expect("Error while reading file.");

    assert_eq!(expected, actual);
}
