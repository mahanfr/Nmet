use crate::{compile_to_exc, compiler::compile_to_asm, utils::get_program_name};
use std::{fs::remove_file, process::Command};

fn generate_asm(path: impl ToString) {
    compile_to_asm(path.to_string());
    compile_to_exc(path.to_string());
    let program_name = get_program_name(path);
    remove_file(format!("./build/{}.o", program_name)).unwrap_or_else(|_| ());
    remove_file(format!("./build/{}.asm", program_name)).unwrap_or_else(|_| ());
}

#[test]
fn binary_expr_test() {
    generate_asm("./tests/binary_expr.nmt");
    let output = Command::new("./build/binary_expr")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/binary_expr").unwrap_or_else(|_| ());
}

#[test]
fn cont_break_test() {
    generate_asm("./tests/cont_break.nmt");
    let output = Command::new("./build/cont_break")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "5\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/cont_break").unwrap_or_else(|_| ());
}

#[test]
fn compare_expr_test() {
    generate_asm("./tests/compare_expr.nmt");
    let output = Command::new("./build/compare_expr")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "1\n1\n1\n1\n0\n1\n1\n0\n1\n0\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/compare_expr").unwrap_or_else(|_| ());
}

#[test]
fn string_expr_test() {
    generate_asm("./tests/string_expr.nmt");
    let output = Command::new("./build/string_expr")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "Hello\nWorld\t\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/string_expr").unwrap_or_else(|_| ());
}

#[test]
fn structs_test() {
    generate_asm("./tests/structs.nmt");
    let output = Command::new("./build/structs")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "65\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/structs").unwrap_or_else(|_| ());
}

#[test]
fn loops_test() {
    generate_asm("./tests/loops.nmt");
    let output = Command::new("./build/loops")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "32\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/loops").unwrap_or_else(|_| ());
}

#[test]
fn conditions_test() {
    generate_asm("./tests/conditions.nmt");
    let output = Command::new("./build/conditions")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "420\n69\n85\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/conditions").unwrap_or_else(|_| ());
}

#[test]
fn functions_test() {
    generate_asm("./tests/functions.nmt");
    let output = Command::new("./build/functions")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "1\n2\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/functions").unwrap_or_else(|_| ());
}

#[test]
fn assgin_test() {
    generate_asm("./tests/assgin.nmt");
    let output = Command::new("./build/assgin")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "20\n22\n12\n24\n2\n0\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/assgin").unwrap_or_else(|_| ());
}

#[test]
fn arrays_test() {
    generate_asm("./tests/arrays.nmt");
    let output = Command::new("./build/arrays")
        .output()
        .expect("Error Executing the program!");
    assert!(output.status.success());
    let expectation = "0\n1\n2\n";
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        expectation.to_string()
    );
    remove_file("./build/arrays").unwrap_or_else(|_| ());
}
