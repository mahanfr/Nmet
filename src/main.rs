use crate::utils::get_program_name;
use std::error::Error;
use std::process::Command;
use std::{env::args, process::exit};

mod codegen;
mod command_line;
mod compiler;
mod elf;
mod error_handeling;
mod lexer;
mod macros;
mod output_generator;
mod parser;
#[cfg(test)]
mod tests;
mod utils;
use command_line::{help_command, CliArgs};
use compiler::{compile_to_asm, compile_to_elf};

// --- Static Compiler Defenition
pub static VERSION: &str = "v0.0.1-Beta";
pub static COPYRIGHT: &str = "Mahan Farzaneh 2023-2024";
pub static DEBUG: bool = true;

/// Compiles the given file into an executable
fn compile_command(arg: &mut CliArgs) {
    if arg.get().starts_with('-') {
        match arg.get().as_str() {
            // "--llvm" => {
            //     arg.next();
            //     compile_to_llvm(arg.get());
            //     return;
            // }
            "-elf" => {
                arg.next();
                compile_to_elf(arg.get());
                exit(0);
            }
            _ => {
                eprintln!("Unknown option for compile command!");
                exit(-1);
            }
        }
    }
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
