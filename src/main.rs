/**********************************************************************************************
*
*   Nmet main entry point
*
*   This file provides and entry point to the compiler all the cli arguments and shell
*   funtionalites are handeled frem this file.
*   All modules used in the code base must be defined here.
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use std::env::Args;
use std::error::Error;
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::Mutex;
use std::{env::args, process::exit};

mod codegen;
mod compiler;
mod error_handeling;
mod lexer;
mod linker;
mod macros;
mod optim;
mod parser;
mod terms;
#[cfg(test)]
mod tests;
mod utils;
use codegen::text::x86_64_nasm_generator;
use compiler::{compile, CompilerContext};
use linker::parse_elf_objfile;
use utils::get_output_path_from_input;

use crate::compiler::impl_bifs;
use crate::utils::padding_right;

// --- Static Compiler Defenition
pub static VERSION: &str = "v0.0.1-Beta";

/// Terget name for assembling using Nasm
fn assembler_target() -> &'static str {
    if cfg!(windows) {
        "win64"
    } else {
        "elf64"
    }
}

// nmet [options] (input_file)
// -nasm -no-link -no-assemble -keep-asm -keep-obj
// -o outputpath
// -l<mod_name>
// -L<mod_path>
// -T <Target>
#[derive(Debug, Default)]
pub struct CompilerOptions {
    pub output_path: Option<PathBuf>,
    pub use_nasm: bool,
    pub no_linking: bool,
    pub no_assembling: bool,
    pub keep_asm: bool,
    pub keep_obj: bool,
    pub linker_flags: Vec<String>,
    pub use_libc: bool,
    pub create_bin: bool,
    pub simulate: bool,
    pub target_platform: u8,
}

fn copywrite() {
    println!("-------------------------------------------------------------");
    println!("| Nmet v0.7.1                                                |");
    println!("| Nmet Programmin Language Copyright: Mahanfaraneh 2023-2024 |");
    println!("| Project distributed Under MIT License                      |");
    println!("-------------------------------------------------------------");
}

pub fn help_command(program_name: &str) {
    println!("{program_name} [options] (input_file)");
    println!("Options:");
    println!(
        "  {} Specify output path (default: \"./build/input_file\")",
        padding_right("-o <output_path>")
    );
    println!(
        "  {} Simulate (Interpet) the program for debugging and running on unsupported OS",
        padding_right("-s | --simulate")
    );
    println!(
        "  {} dump instructions in a binary file",
        padding_right("-b | --bin")
    );
    println!(
        "  {} use Nasm Assembler to assemble generated code",
        padding_right("--nasm")
    );
    println!(
        "  {} Do not link the generated object file",
        padding_right("--no-link")
    );
    println!(
        "  {} Only Generates an asm file",
        padding_right("--no-assemble")
    );
    println!(
        "  {} Do not remove the generated asm file",
        padding_right("--keep-asm")
    );
    println!(
        "  {} Do not remove the generated object file",
        padding_right("--keep-obj")
    );
    println!("  {} Use C library Dynamicaly", padding_right("--use-libc"));
    println!(
        "  {} Search for library LIBNAME",
        padding_right("-l<LIBNAME>")
    );
    println!(
        "  {} add directory to library search path",
        padding_right("-L<DIR>")
    );
    println!("  {} Show help", padding_right("-h, --help"));
    println!("  {} Show Version", padding_right("-v, --version"));
}

/// Runs External commands for generating the object files
pub fn assemble_with_nasm(path: PathBuf) {
    log_info!(
        "Assembling for {} - generaiting {}",
        assembler_target(),
        path.with_extension("o").to_string_lossy()
    );
    let nasm_output = Command::new("nasm")
        .arg(format!("-f{}", assembler_target()))
        .arg("-o")
        .arg(path.with_extension("o"))
        .arg(path.with_extension("asm"))
        .output()
        .expect("Can not run nasm command! do you have nasm installed?");
    if !nasm_output.status.success() {
        log_error!("Failed to Assemble: Status code non zero");
        eprintln!("{}", String::from_utf8(nasm_output.stderr).unwrap());
    }
    log_success!("Object file generated using Nasm!");
}

/// Runs External commands for generating the executable
pub fn link_to_exc(path: PathBuf, co: &CompilerOptions) {
    log_info!(
        "Linking object file - generating {}",
        path.with_extension("").to_string_lossy()
    );
    let linker_output = Command::new("ld")
        .arg("-o")
        .arg(path.with_extension(""))
        .arg(path.with_extension("o"))
        .args(["-dynamic-linker", "/usr/lib64/ld-linux-x86-64.so.2"])
        .args(&co.linker_flags)
        .output()
        .expect("Can not link using ld command!");
    if !linker_output.status.success() {
        log_error!("Failed to Link Exectable: Status code non zero");
        eprintln!("{}", String::from_utf8(linker_output.stderr).unwrap());
    } else {
        log_success!("Executable file have been Generated!");
    }
}

pub fn setup_compiler(input: String, co: &CompilerOptions) {
    let out_path = match co.output_path.clone() {
        None => get_output_path_from_input(input.clone().into()),
        Some(pt) => pt,
    };
    let mut compiler_context = CompilerContext::new(input.clone());

    compile(&mut compiler_context, input.clone());
    impl_bifs(&mut compiler_context);
    let prefix = out_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    if co.simulate {
        log_info!("Simulation is not supported yet!");
    }
    if co.use_nasm {
        log_info!("Generating asm text file...");
        x86_64_nasm_generator(out_path.as_path(), &compiler_context).unwrap();
        log_success!("Nasm Text file Generated!");
        if co.no_assembling {
            return;
        }
        assemble_with_nasm(out_path.clone());
        if co.no_linking {
            return;
        }
        link_to_exc(out_path.clone(), co);
    } else {
        if co.create_bin {
            log_info!("Generating binary file...");
            crate::codegen::elf::generate_bin(out_path.as_path(), &mut compiler_context);
            log_success!("Instructions Binary file Generated!");
        }
        log_info!("Generating elf object file...");
        crate::codegen::elf::generate_elf(out_path.as_path(), &mut compiler_context);
        log_success!("Elf object file Generated!");
        if co.no_linking {
            return;
        }
        link_to_exc(out_path.clone(), co);
    }
    if !co.keep_asm && remove_file(out_path.with_extension("asm")).is_ok() {
        log_info!("Removing asm files")
    }
    if !co.keep_obj && remove_file(out_path.with_extension("o")).is_ok() {
        log_info!("Removing object files")
    }
}

fn collect_compiler_options(args: &mut Args) -> (String, CompilerOptions) {
    let mut co = CompilerOptions::default();
    if cfg!(windows) {
        co.target_platform = 1;
    }
    let compiler_path = args.next().unwrap();
    let mut input_path = String::new();
    loop {
        let Some(arg) = args.next() else {
            break;
        };
        if arg.starts_with("-l") || arg.starts_with("-L") {
            co.linker_flags.push(arg.clone());
            continue;
        }
        match arg.as_str() {
            "-h" | "--help" => {
                copywrite();
                help_command(&compiler_path);
                exit(0);
            }
            "-v" | "--version" => {
                copywrite();
                exit(0);
            }
            "--no-link" => co.no_linking = true,
            "--no-assemble" => co.no_assembling = true,
            "--nasm" => co.use_nasm = true,
            "--keep-asm" => co.keep_asm = true,
            "--keep-obj" => co.keep_obj = true,
            "--use-libc" => co.use_libc = true,
            "-b" | "--bin" => co.create_bin = true,
            "-s" | "--simulate" => co.simulate = true,
            "-T" => {
                let Some(target) = args.next() else {
                    log_error!("No target specified!");
                    help_command(&compiler_path);
                    exit(-1);
                };
                co.target_platform = target_string_to_number(&target);
            }
            "-o" => {
                let Some(path) = args.next() else {
                    log_error!("No output path after -o option!");
                    help_command(&compiler_path);
                    exit(-1);
                };
                co.output_path = Some(PathBuf::from_str(&path).unwrap());
            }
            _ => {
                if arg.starts_with('-') {
                    log_error!("Invalid compiler option ({})!", arg);
                    help_command(&compiler_path);
                    exit(-1);
                } else {
                    input_path = arg;
                }
            }
        }
    }
    if input_path.is_empty() {
        log_error!("No file has been provided!");
        help_command(&compiler_path);
        exit(-1);
    }
    (input_path, co)
}

static TARGET_PLATFORM: Mutex<u8> = Mutex::new(0);

pub fn target_string_to_number(target: &str) -> u8 {
    match target {
        "LINUX" | "linux" => 0,
        "WINDOWS" | "WIN" | "windows" | "win" => 1,
        _ => u8::MAX,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    parse_elf_objfile("./tests/libadd.a".to_string());
    return Ok(());

    let mut args = args();
    let (ipath, co) = collect_compiler_options(&mut args);
    *TARGET_PLATFORM.lock().unwrap() = co.target_platform;
    setup_compiler(ipath, &co);
    Ok(())
}
