use crate::compiler::IRGenerator;
use crate::parser::program;
use std::env::args;
use std::error::Error;
use std::fs;
use std::process::Command;

mod command_line;
mod compiler;
mod lexer;
mod parser;
use command_line::{help_command, CliArgs};
use lexer::Lexer;

// --- Static Compiler Defenition
pub static VERSION: &str = "v0.0.1-Beta";
pub static COPYRIGHT: &str = "Mahan Farzaneh 2023-2024";
pub static DEBUG: bool = true;

/// Compiles the given file into an executable
fn compile_command(arg: &mut CliArgs) {
    let source = fs::read_to_string(arg.get()).expect("Can not Read the file");
    let mut lexer = Lexer::new(String::new(), source);
    let mut ir_gen = IRGenerator::new();
    ir_gen
        .compile(program(&mut lexer))
        .expect("Can not Compile Program");
    compile_to_exc();
}

/// Runs External commands for generating the executable
pub fn compile_to_exc() {
    println!("[info] Assembling for elf64 - generaiting output.o");
    let nasm_output = Command::new("nasm")
        .arg("-felf64")
        .arg("-o")
        .arg("./build/output.o")
        .arg("./build/output.asm")
        .output()
        .expect("Can not run nasm command! do you have nasm installed?");
    if !nasm_output.status.success() {
        println!("[error] Failed to Assemble: Status code non zero");
        println!("{}", String::from_utf8(nasm_output.stderr).unwrap());
    }
    println!("[info] Linking object file...");
    let linker_output = Command::new("ld")
        .arg("-o")
        .arg("./build/output")
        .arg("./build/output.o")
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
