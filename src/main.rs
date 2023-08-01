use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process::{exit, Command};

mod lexer;
mod parser;
use lexer::Lexer;
use parser::{
    Assgin, Block, ElseBlock, Expr, IFStmt, ProgramFile, ProgramItem, StaticVariable, Stmt,
    VariableDeclare, WhileStmt,
};

use crate::parser::program;

// --- Static Compiler Defenition
static VERSION: &'static str = "v0.0.1-Beta";
static COPYRIGHT: &'static str = "Mahan Farzaneh 2023-2024";
static DEBUG: bool = true;

// struct AsmInBlock {
//
// }
//
// enum AsmOp {
//     Block(AsmInBlock),
//     Jump(String),
//     JumpNotEq(String),
//     JumpEq(String),
// }
//
// struct AsmVariable {
//     // TODO: type
//     ident: String,
//     offset: usize,
//     size: usize,
// }
//
// struct FuncAsmBlock {
//     tag: String,
//     ops: Vec<AsmOp>,
//     scope_mem_size: usize,
//     scope_variables: Vec<AsmVariable>,
//     stack_count: usize,
//     // TODO: frame size
// }

macro_rules! asm {
    ($($arg:tt)+) => (
        format!("    {}\n",format_args!($($arg)+))
    );
}

fn padding_right(str: &str) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    for _ in 0..(20 - str.len()) {
        text.push(' ');
    }
    text
}

fn help_command() -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

fn compile_command(path: String) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path.clone()).expect("Can not Read the file");
    let mut lexer = Lexer::new(String::new(), source.to_string());
    let mut ir_gen = IRGenerator::new();
    ir_gen.compile(program(&mut lexer))?;
    compile_to_exc()?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct VariableMap {
    _ident: String,
    offset: usize,
    size: usize,
    is_mut: bool,
}

pub struct IRGenerator {
    blocks_buf: Vec<String>,
    static_var_buf: Vec<String>,
    scoped_blocks: Vec<usize>,
    block_id: usize,
    variables_map: HashMap<String, VariableMap>,
    mem_offset: usize,
}

impl IRGenerator {
    // TODO: handle Error for Parsing
    pub fn new() -> Self {
        Self {
            static_var_buf: Vec::new(),
            blocks_buf: Vec::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            variables_map: HashMap::new(),
            mem_offset: 0,
        }
    }

    fn frame_size(&self) -> usize {
        return 2 << self.mem_offset.ilog2() as usize;
    }

    pub fn find_variable(&self, ident: String) -> Option<VariableMap> {
        for block_id in &self.scoped_blocks {
            let map_ident = format!("{ident}%{}", block_id);
            let map = self.variables_map.get(&map_ident);
            if map.is_some() {
                return Some(map.unwrap().clone());
            }
        }
        return None;
    }

    pub fn insert_variable(&mut self, var: &VariableDeclare) {
        let ident = format!("{}%{}", var.ident, self.block_id);
        let var_map = VariableMap {
            _ident: var.ident.clone(),
            offset: self.mem_offset,
            // TODO: Change size
            size: 8,
            is_mut: var.mutable,
        };
        self.mem_offset += 8;
        if var.init_value.is_some() {
            let init_value = var.init_value.clone().unwrap();
            // this pushes result in stack
            self.compile_expr(&init_value);
            let mem_acss = format!("qword [rbp-{}]", var_map.offset + var_map.size);
            self.blocks_buf.push(asm!("pop rax"));
            self.blocks_buf.push(asm!("mov {mem_acss},rax"));
        }
        self.variables_map.insert(ident, var_map);
    }

    // TODO: Handle Compilation Error
    pub fn compile(&mut self, program: ProgramFile) -> Result<(), Box<dyn Error>> {
        for item in program.items {
            match item {
                ProgramItem::StaticVar(s) => {
                    self.compile_static_var(s);
                }
                ProgramItem::Func(f) => {
                    if f.ident == "main" {
                        self.blocks_buf.push("_start:\n".to_string());
                    } else {
                        todo!();
                    }
                    // set rbp to stack pointer for this block
                    let index_1 = self.blocks_buf.len();
                    self.blocks_buf.push(String::new());
                    let index_2 = self.blocks_buf.len();
                    self.blocks_buf.push(String::new());
                    let index_3 = self.blocks_buf.len();
                    self.blocks_buf.push(String::new());

                    self.compile_block(&f.block);
                    // revert rbp
                    if self.variables_map.len() > 0 {
                        self.blocks_buf[index_1] = asm!("push rbp");
                        self.blocks_buf[index_2] = asm!("mov rbp, rsp");
                        self.blocks_buf[index_3] = asm!("sub rsp, {}", self.frame_size());
                        self.blocks_buf.push(asm!("pop rbp"));
                    }
                    // Call Exit Syscall
                    if f.ident == "main" {
                        self.blocks_buf.push(asm!("mov rax, 60"));
                        self.blocks_buf.push(asm!("mov rdi, 0"));
                        self.blocks_buf.push(asm!("syscall"));
                    }
                }
            }
        }
        assert!(
            self.scoped_blocks.len() == 0,
            "Somting went wrong: Scope has not been cleared"
        );

        //println!("{:?}",self.scoped_blocks);
        self.write_to_file()?;
        Ok(())
    }

    fn compile_static_var(&mut self, stat_v: StaticVariable) {
        if self.static_var_buf.len() == 0 {
            self.static_var_buf.push("section .data\n".to_string());
        }
        let value = match stat_v.value {
            Expr::Int(x) => x.to_string(),
            _ => {
                todo!()
            }
        };
        self.static_var_buf
            .push(format!("{} db {}\n", stat_v.ident, value));
    }

    fn compile_block(&mut self, block: &Block) {
        self.block_id += 1;
        self.scoped_blocks.push(self.block_id);
        for stmt in &block.stmts {
            self.compile_stmt(&stmt);
        }
        self.scoped_blocks.pop().unwrap();
    }

    fn compile_if_stmt(&mut self, ifs: &IFStmt, exit_tag: usize) {
        self.compile_expr(&ifs.condition);
        let next_tag = match ifs.else_block.as_ref() {
            ElseBlock::None => exit_tag,
            _ => self.blocks_buf.len(),
        };
        self.blocks_buf.push(asm!("jne .L{}", next_tag));
        self.compile_block(&ifs.then_block);
        match ifs.else_block.as_ref() {
            ElseBlock::None => {
                self.blocks_buf.push(asm!(".L{}:", next_tag));
            }
            ElseBlock::Else(b) => {
                self.blocks_buf.push(asm!("jmp .L{}", exit_tag));
                self.blocks_buf.push(asm!(".L{}:", next_tag));
                self.compile_block(&b);
                self.blocks_buf.push(asm!(".L{}:", exit_tag));
            }
            ElseBlock::Elif(iff) => {
                self.blocks_buf.push(asm!("jmp .L{}", exit_tag));
                self.blocks_buf.push(asm!(".L{}:", next_tag));
                self.compile_if_stmt(iff, exit_tag);
            }
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VariableDecl(v) => {
                self.insert_variable(v);
            }
            Stmt::Print(e) => {
                self.compile_expr(&e);
                self.blocks_buf.push(asm!("pop rdi"));
                self.blocks_buf.push(asm!("call print"));
            }
            Stmt::If(ifs) => {
                let exit_tag = self.blocks_buf.len();
                self.compile_if_stmt(ifs, exit_tag);
            }
            Stmt::Assgin(a) => {
                self.compile_assgin(a);
            }
            Stmt::While(w) => {
                self.compile_while(w);
            }
            _ => {
                todo!();
            }
        }
    }

    fn compile_while(&mut self, w_stmt: &WhileStmt) {
        let cond_tag = self.blocks_buf.len();
        self.blocks_buf.push(asm!("jmp .L{}",cond_tag));
        let block_tag = cond_tag + 1;
        self.blocks_buf.push(asm!(".L{}:",block_tag));
        self.compile_block(&w_stmt.block);
        self.blocks_buf.push(asm!(".L{}:",cond_tag));
        self.compile_expr(&w_stmt.condition);
        // TODO: Change This
        self.blocks_buf.push(asm!("jne .L{}",block_tag));
    }

    fn compile_assgin(&mut self, assign: &Assgin) {
        match &assign.left {
            Expr::Variable(v) => {
                let v_map = self.find_variable(v.clone()).unwrap_or_else(|| {
                    eprintln!("Error: Could not find variable {} in this scope", v.clone());
                    exit(1);
                });
                if !v_map.is_mut {
                    eprintln!("Error: Variable is not mutable. Did you forgot to define it with '=' insted of ':=' ?");
                    exit(1);
                }
                match assign.op {
                    parser::AssginOp::Eq => {
                        self.compile_expr(&assign.right);
                        let mem_acss = format!("qword [rbp-{}]", v_map.offset + v_map.size);
                        self.blocks_buf.push(asm!("pop rax"));
                        self.blocks_buf.push(asm!("mov {mem_acss},rax"));
                    }
                }
            }
            Expr::ArrayIndex(_) => {
                todo!();
            }
            _ => {
                eprintln!("Error: Expected a Variable type expression found Value");
                exit(1);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        // left = compile expr
        // right = compile expr
        // +
        match expr {
            Expr::Variable(v) => {
                let v_map = self.find_variable(v.clone()).unwrap_or_else(|| {
                    eprintln!("Error: Trying to access an Undifined variable ({v})");
                    exit(1);
                });
                let mem_acss = format!("qword [rbp-{}]", v_map.offset + v_map.size);
                self.blocks_buf.push(asm!("mov rax,{mem_acss}"));
                self.blocks_buf.push(asm!("push rax"));
            }
            Expr::Int(x) => {
                // push x
                self.blocks_buf.push(asm!("push {}", x));
            }
            Expr::Compare(c) => {
                // TODO: Convert exprs to 0 or 1 and push into stack
                self.compile_expr(c.left.as_ref());
                self.compile_expr(c.right.as_ref());
                self.blocks_buf.push(asm!("pop rax"));
                self.blocks_buf.push(asm!("pop rbx"));
                self.blocks_buf.push(asm!("cmp rax, rbx"));
            }
            Expr::Binary(b) => {
                self.compile_expr(b.left.as_ref());
                self.compile_expr(b.right.as_ref());
                self.blocks_buf.push(asm!("pop rax"));
                self.blocks_buf.push(asm!("pop rbx"));
                match b.op {
                    parser::Op::Plus => {
                        self.blocks_buf.push(asm!("add rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    }
                    parser::Op::Sub => {
                        self.blocks_buf.push(asm!("sub rbx, rax"));
                        self.blocks_buf.push(asm!("push rbx"));
                    }
                    parser::Op::Multi => {
                        self.blocks_buf.push(asm!("imul rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    }
                    parser::Op::Devide => {
                        todo!();
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Unary(_) => {
                todo!();
            }
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

pub fn compile_to_exc() -> Result<(), Box<dyn Error>> {
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
        println!("{}", String::from_utf8(nasm_output.stderr)?);
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
        println!("{}", String::from_utf8(linker_output.stderr)?);
    }
    println!("[sucsees] Executable File Has been Generated!");
    // println!("+ Running The Generated Executable");
    // let output = Command::new("./build/output")
    //     .output()
    //     .expect("Error Executing the program!");
    // println!("{}",String::from_utf8(output.stdout)?);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut arg = args();
    arg.next();
    loop {
        if let Some(command) = arg.next() {
            match command.as_str() {
                "help" => {
                    help_command()?;
                    return Ok(());
                }
                "--help" => {
                    help_command()?;
                    return Ok(());
                }
                "--version" => {
                    println!("{VERSION}");
                    return Ok(());
                }
                _ => {
                    compile_command(command.clone())?;
                    return Ok(());
                }
            }
        }else {
            break;
        }
    }
    Ok(())
}
