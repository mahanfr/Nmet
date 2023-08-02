use crate::parser::program;
use crate::compiler::IRGenerator;
use std::env::args;
use std::error::Error;
use std::fs;
use std::process::{Command, exit};

mod lexer;
mod parser;
mod compiler;
use lexer::Lexer;

// --- Static Compiler Defenition
static VERSION: &str = "v0.0.1-Beta";
static COPYRIGHT: &str = "Mahan Farzaneh 2023-2024";
static DEBUG: bool = true;

fn padding_right(str: &str) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    for _ in 0..(20 - str.len()) {
        text.push(' ');
    }
    text
}

fn help_command() {
    println!("Nemet {VERSION}");
    println!("Nemet Programmin Language Copyright: {COPYRIGHT}");
    println!("Project distributed Under MIT License");
    if DEBUG {
        println!("--- DEBUG MODE ---");
    }
    println!("\nnemet [Command] <path> (Options)");
    println!("Commands:");
    println!("\t{} Show help", padding_right("help"));
    println!("Options:");
    println!("\t{} Show help", padding_right("--help"));
    println!("\t{} Show Version", padding_right("--version"));
}

fn compile_command(arg: &mut CliArgs) {
    let source = fs::read_to_string(arg.get()).expect("Can not Read the file");
    let mut lexer = Lexer::new(String::new(), source);
    let mut ir_gen = IRGenerator::new();
    ir_gen.compile(program(&mut lexer)).expect("Can not Compile Program");
    compile_to_exc();
}

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
    // println!("+ Running The Generated Executable");
    // let output = Command::new("./build/output")
    //     .output()
    //     .expect("Error Executing the program!");
    // println!("{}",String::from_utf8(output.stdout)?);
}

// nemet [commands] <options> [path] 

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

struct CliArgs {
    args: Vec<String>,
    index: usize,
}

impl CliArgs {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            index: 1,
        }
    }
   
    pub fn get(&self) -> String {
        self.args[self.index].clone()
    }

    pub fn next(&mut self) {
        if self.index < self.args.len() {
            self.index += 1;
        } else {
             help_command();
             exit(-1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = CliArgs::new(args().collect());
    commands(&mut args);
    Ok(())
}
