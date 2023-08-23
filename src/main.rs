use crate::utils::get_program_name;
use std::env::args;
use std::error::Error;
use std::process::Command;

mod asm_generator;
mod command_line;
mod compiler;
mod error_handeling;
mod lexer;
mod parser;
mod utils;
use command_line::{help_command, CliArgs};
use compiler::compile_to_asm;

// --- Static Compiler Defenition
pub static VERSION: &str = "v0.0.1-Beta";
pub static COPYRIGHT: &str = "Mahan Farzaneh 2023-2024";
pub static DEBUG: bool = true;

/// Compiles the given file into an executable
fn compile_command(arg: &mut CliArgs) {
    compile_to_asm(arg.get());
    compile_to_exc(arg.get());
}

/// Runs External commands for generating the executable
pub fn compile_to_exc(path: String) {
    let program_name = get_program_name(path);
    println!("[info] Assembling for elf64 - generaiting output.o");
    let nasm_output = Command::new("nasm")
        .arg("-felf64")
        .arg("-o")
        .arg(format!("./build/{}.o", program_name))
        .arg(format!("./build/{}.asm", program_name))
        .output()
        .expect("Can not run nasm command! do you have nasm installed?");
    if !nasm_output.status.success() {
        println!("[error] Failed to Assemble: Status code non zero");
        println!("{}", String::from_utf8(nasm_output.stderr).unwrap());
    }
    println!("[info] Linking object file...");
    let linker_output = Command::new("ld")
        .arg("-o")
        .arg(format!("./build/{}", program_name))
        .arg(format!("./build/{}.o", program_name))
        .output()
        .expect("Can not link using ld command!");
    if !linker_output.status.success() {
        println!("[error] Failed to Assemble: Status code non zero");
        println!("{}", String::from_utf8(linker_output.stderr).unwrap());
    }
    println!("[sucsees] Executable File Has been Generated!");
}

/// Run The Program Directly after generating the executable
pub fn run_program() {
    println!("+ Running The Generated Executable");
    let output = Command::new("./build/output")
        .output()
        .expect("Error Executing the program!");
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

/// Executes commands resived by commandline
/// nemet [commands] <options> [path]
/// First level of command line argument parsing
fn commands(arg: &mut CliArgs) {
    match arg.get().as_str() {
        "--help" | "help" | "-h" => {
            help_command();
        }
        "--version" | "-v" => {
            println!("{VERSION}");
        }
        "--compile" | "-c" => {
            arg.next();
            compile_command(arg);
        }
        _ => {
            compile_command(arg);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = CliArgs::new(args().collect());
    commands(&mut args);
    Ok(())
}

#[cfg(test)]
mod functional {
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
}
