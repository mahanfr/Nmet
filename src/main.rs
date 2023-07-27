use std::io::{BufWriter, Write};
use std::process::Command;
use std::{error::Error, process::exit};
use std::fs::{self, File};
use std::env::args;

mod lexer;
mod ast;
mod parser;
use lexer::{Lexer, TokenType};
use parser::{ProgramFile, ProgramItem};

use crate::parser::program;

// --- Static Compiler Defenition
static VERSION : &'static str = "v0.0.1-Beta";
static COPYRIGHT : &'static str = "Mahan Farzaneh 2023-2024";
static DEBUG : bool = true;

fn padding_right(str : &str) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    for _ in 0..(20-str.len()) {
       text.push(' '); 
    }
    text
}

fn help_command() -> Result<(),Box<dyn Error>> {
    println!("Nemet {VERSION}");
    println!("Nemet Programmin Language Copyright: {COPYRIGHT}");
    println!("Project distributed Under MIT License");
    if DEBUG {
        println!("--- DEBUG MODE ---");
    }
    println!("\nnemet [Command] <path> (Options)");
    println!("Commands:");
    println!("\t{} Show help",padding_right("help"));
    println!("Options:");
    println!("\t{} Show help",padding_right("--help"));
    println!("\t{} Show Version",padding_right("--version"));
    Ok(())
}

fn compile_command(path: String) -> Result<(),Box<dyn Error>> {
    let source = fs::read_to_string(path.clone())
        .expect("Can not Read the file");
    let mut lexer = Lexer::new(path.clone(),source);
    let mut token = lexer.next_token();
    while !token.is_none() {
        println!("{:?}",token.unwrap());
        token = lexer.next_token();
    }
    Ok(())
}

pub fn compile(program: ProgramFile) -> Result<(),Box<dyn Error>> {
    fs::create_dir_all("./build")?;
    let stream = File::create("./build/output.asm")?;
    let mut file = BufWriter::new(stream);
    file.write(b";; This File is Automatically Created Useing Nemet Parser\n")?;
    file.write(b";; Under MIT License Copyright MahanFarzaneh 2023-2024\n\n")?;
    file.write(b"section .text\n")?;
    file.write(b"global _start\n")?;

    for item in program.items {
        match item {
            ProgramItem::Func(f) => {
                if f.ident == "main" {
                   file.write(b"_start:\n")?;
                   file.write(b"    mov rax, 60\n")?;
                   file.write(b"    mov rdi, 0\n")?;
                   file.write(b"    syscall\n")?;
                }
            },
            ProgramItem::StaticVar(s) => {
                todo!();
            }
        }
    }
    file.flush()?;

    Command::new("nasm")
        .arg("-felf64")
        .arg("-o")
        .arg("./build/output.o")
        .arg("./build/output.asm")
        .output()?;
    Command::new("ld")
        .arg("-o")
        .arg("./build/output")
        .arg("./build/output.o")
        .output()?;

    Ok(())
}


fn main() -> Result<(),Box<dyn Error>> {
    let source = 
    r#"
        fun main() {
        }
    "#;
    let mut lexer = Lexer::new(String::new(),source.to_string());
    // TODO: Move to top root of parser
    // lexer.next_token();
    // println!("{:?}",expr(&mut lexer));
    compile(program(&mut lexer))?;
    return Ok(());
    let mut arg = args().into_iter();
    arg.next();
    loop {
        let Some(command) = arg.next() else {
            break;
        };
        match command.as_str() {
            "help" => {
                help_command()?;
                return Ok(());
            },
            "--help" => {
                help_command()?;
                return Ok(());
            },
            "--version" => {
                println!("{VERSION}");
                return Ok(());
            },
            _ => {
                compile_command(command.clone())?;
                return Ok(());
            },
        }
    }
    Ok(())
}
