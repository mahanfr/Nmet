use std::io::{BufWriter, Write};
use std::process::Command;
use std::error::Error;
use std::fs::{self, File};
use std::env::args;

mod lexer;
mod parser;
use lexer::Lexer;
use parser::{ProgramFile, ProgramItem, StaticVariable, Expr, Stmt};

use crate::parser::program;

// --- Static Compiler Defenition
static VERSION : &'static str = "v0.0.1-Beta";
static COPYRIGHT : &'static str = "Mahan Farzaneh 2023-2024";
static DEBUG : bool = true;

macro_rules! asm {
    ($($arg:tt)+) => (
        format!("    {}\n",format_args!($($arg)+))
    );
}

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
    let mut lexer = Lexer::new(String::new(),source.to_string());
    let mut ir_gen = IRGenerator::new();
    ir_gen.compile(program(&mut lexer))?;
    compile_to_exc()?;
    Ok(())
}

pub struct IRGenerator {
    blocks_buf: Vec<String>,
    static_var_buf: Vec<String>, 
}

impl IRGenerator {
    // TODO: handle Error for Parsing
    pub fn new() -> Self {
        Self {
            static_var_buf: Vec::new(),
            blocks_buf: Vec::new(),
        }
    }

    // TODO: Handle Compilation Error
    pub fn compile(&mut self, program: ProgramFile) -> Result<(),Box<dyn Error>> {
        for item in program.items {
            match item {
                ProgramItem::StaticVar(s) => {
                    self.compile_static_var(s);
                },
                ProgramItem::Func(f) => {
                    if f.ident == "main" {
                        self.blocks_buf.push("_start:\n".to_string());
                    } else {
                        todo!();
                    }
                    for stmt in f.block.stmts {
                        self.compile_stmt(&stmt);
                    }

                    if f.ident == "main" {
                        self.blocks_buf.push(asm!("mov rax, 60"));
                        self.blocks_buf.push(asm!("mov rdi, 0"));
                        self.blocks_buf.push(asm!("syscall"));
                    }
                },
            }
        }
        self.write_to_file()?;
        Ok(()) 
    }

    fn compile_static_var(&mut self,stat_v: StaticVariable) {
        if self.static_var_buf.len() == 0 {
            self.static_var_buf.push("section .data\n".to_string());
        }
        let value = match stat_v.value{
            Expr::Int(x) => x.to_string(),
            _ => {
                todo!()
            }
        };
        self.static_var_buf.push(format!("{} db {}\n",stat_v.ident,value));
    }


    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(e) => {
                self.comile_expr(&e);
                self.blocks_buf.push(asm!("pop rdi"));
                self.blocks_buf.push(asm!("call print"));
            },
            _ => {
                todo!();
            }
        }
    }

    fn comile_expr(&mut self, expr: &Expr) {
        // left = compile expr
        // right = compile expr
        // +
        match expr {
            Expr::Int(x) => {
                // push x
                self.blocks_buf.push(asm!("push {}",x));
            },
            Expr::Binary(b) => {
                self.comile_expr(b.left.as_ref());
                self.comile_expr(b.right.as_ref());
                self.blocks_buf.push(asm!("pop rax"));
                self.blocks_buf.push(asm!("pop rbx"));
                match b.op {
                    parser::Op::Plus => {
                        self.blocks_buf.push(asm!("add rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    },
                    parser::Op::Sub => {
                        self.blocks_buf.push(asm!("sub rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    },
                    parser::Op::Multi => {
                        self.blocks_buf.push(asm!("imul rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    },
                    parser::Op::Devide => {
                        todo!();
                    },
                    _ => unreachable!(),
                }
            },
            Expr::Unary(_) => {
                todo!();
            },
            _ => {
                todo!();
            }
        }
    }

    // TODO: Error Handleing Error Type FILE
    fn write_to_file(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all("./build").unwrap();
        let stream = File::create("./build/output.asm").unwrap();
        let mut file = BufWriter::new(stream);
        println!("[info] Generating asm files...");
        file.write(b";; This File is Automatically Created Useing Nemet Parser\n")?;
        file.write(b";; Under MIT License Copyright MahanFarzaneh 2023-2024\n\n")?;
        for line in &self.static_var_buf {
             file.write(line.as_bytes())?;
        }
        file.write(b"\n")?;
        // TODO: Add this to the section related
        file.write(b"section .text\n")?;
        file.write(b"global _start\n")?;

        file.write(b"print:\n")?;
        file.write(b"    push    rbp\n")?;
        file.write(b"    mov     rbp, rsp\n")?;
        file.write(b"    sub     rsp, 64\n")?;
        file.write(b"    mov     qword [rbp-56], rdi\n")?;
        file.write(b"    mov     qword [rbp-8], 1\n")?;
        file.write(b"    mov     eax, 32\n")?;
        file.write(b"    sub     rax, qword [rbp-8]\n")?;
        file.write(b"    mov     BYTE [rbp-48+rax], 10\n")?;
        file.write(b".L3:\n")?;
        file.write(b"    mov     rcx, qword [rbp-56]\n")?;
        file.write(b"    mov     rdx, -3689348814741910323\n")?;
        file.write(b"    mov     rax, rcx\n")?;
        file.write(b"    mul     rdx\n")?;
        file.write(b"    shr     rdx, 3\n")?;
        file.write(b"    mov     rax, rdx\n")?;
        file.write(b"    sal     rax, 2\n")?;
        file.write(b"    add     rax, rdx\n")?;
        file.write(b"    add     rax, rax\n")?;
        file.write(b"    sub     rcx, rax\n")?;
        file.write(b"    mov     rdx, rcx\n")?;
        file.write(b"    mov     eax, edx\n")?;
        file.write(b"    lea     edx, [rax+48]\n")?;
        file.write(b"    mov     eax, 31\n")?;
        file.write(b"    sub     rax, qword [rbp-8]\n")?;
        file.write(b"    mov     byte [rbp-48+rax], dl\n")?;
        file.write(b"    add     qword [rbp-8], 1\n")?;
        file.write(b"    mov     rax, qword [rbp-56]\n")?;
        file.write(b"    mov     rdx, -3689348814741910323\n")?;
        file.write(b"    mul     rdx\n")?;
        file.write(b"    mov     rax, rdx\n")?;
        file.write(b"    shr     rax, 3\n")?;
        file.write(b"    mov     qword [rbp-56], rax\n")?;
        file.write(b"    cmp     qword [rbp-56], 0\n")?;
        file.write(b"    jne     .L3\n")?;
        file.write(b"    mov     eax, 32\n")?;
        file.write(b"    sub     rax, qword [rbp-8]\n")?;
        file.write(b"    lea     rdx, [rbp-48]\n")?;
        file.write(b"    add     rax, rdx\n")?;
        file.write(b"    mov     rsi, rax\n")?;
        file.write(b"    mov     rbx, qword [rbp-8]\n")?;
        file.write(b"    mov     rdx, rbx\n")?;
        file.write(b"    mov     rdi, 1\n")?;
        file.write(b"    mov     rax, 1\n")?;
        file.write(b"    syscall\n")?;
        file.write(b"    leave\n")?;
        file.write(b"    ret\n")?;

        for instruct in &self.blocks_buf {
            file.write(instruct.as_bytes())?;
        }

        file.flush().unwrap();
        Ok(())
    }

}

pub fn compile_to_exc() -> Result<(),Box<dyn Error>> {
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
        println!("{}",String::from_utf8(nasm_output.stderr)?);
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
        println!("{}",String::from_utf8(linker_output.stderr)?);
    }
    println!("[sucsees] Executable File Has been Generated!");
    // println!("+ Running The Generated Executable");
    // let output = Command::new("./build/output")
    //     .output()
    //     .expect("Error Executing the program!");
    // println!("{}",String::from_utf8(output.stdout)?);
    Ok(())
}

fn main() -> Result<(),Box<dyn Error>> {
    let mut arg = args();
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
