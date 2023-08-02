use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process::exit;

use crate::parser::block::Block;
use crate::parser::expr::{Expr, CompareOp, Op};
use crate::parser::program::{ProgramFile, ProgramItem, StaticVariable};
use crate::parser::stmt::{VariableDeclare, IFStmt, ElseBlock, Stmt, WhileStmt, Assgin, AssginOp};

macro_rules! asm {
    ($($arg:tt)+) => (
        format!("    {}\n",format_args!($($arg)+))
    );
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
        2 << self.mem_offset.ilog2() as usize
    }

    pub fn find_variable(&self, ident: String) -> Option<VariableMap> {
        for block_id in &self.scoped_blocks {
            let map_ident = format!("{ident}%{}", block_id);
            let map = self.variables_map.get(&map_ident);
            if let Some(map) = map {
                return Some(map.clone())
            }
        }
        None
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
                    if !self.variables_map.is_empty() {
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
            self.scoped_blocks.is_empty(),
            "Somting went wrong: Scope has not been cleared"
        );

        //println!("{:?}",self.scoped_blocks);
        self.write_to_file()?;
        Ok(())
    }

    fn compile_static_var(&mut self, stat_v: StaticVariable) {
        if !self.static_var_buf.is_empty() {
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
            self.compile_stmt(stmt);
        }
        self.scoped_blocks.pop().unwrap();
    }

    fn compile_if_stmt(&mut self, ifs: &IFStmt, exit_tag: usize) {
        self.compile_expr(&ifs.condition);
        let next_tag = match ifs.else_block.as_ref() {
            ElseBlock::None => exit_tag,
            _ => self.blocks_buf.len(),
        };
        self.blocks_buf.push(asm!("pop rax"));
        self.blocks_buf.push(asm!("test rax, rax"));
        self.blocks_buf.push(asm!("jz .L{}",next_tag));
        
        self.compile_block(&ifs.then_block);
        match ifs.else_block.as_ref() {
            ElseBlock::None => {
                self.blocks_buf.push(asm!(".L{}:", next_tag));
            }
            ElseBlock::Else(b) => {
                self.blocks_buf.push(asm!("jmp .L{}", exit_tag));
                self.blocks_buf.push(asm!(".L{}:", next_tag));
                self.compile_block(b);
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
                self.compile_expr(e);
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
        // Jump after a compare
        self.compile_expr(&w_stmt.condition);
        self.blocks_buf.push(asm!("pop rax"));
        self.blocks_buf.push(asm!("test rax, rax"));
        self.blocks_buf.push(asm!("jnz .L{}",block_tag));
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
                    AssginOp::Eq => {
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
                self.blocks_buf.push(asm!("mov rcx, 0"));
                self.blocks_buf.push(asm!("mov rdx, 1"));
                self.blocks_buf.push(asm!("pop rbx"));
                self.blocks_buf.push(asm!("pop rax"));
                self.blocks_buf.push(asm!("cmp rax, rbx"));
                match c.op {
                    CompareOp::Eq => {
                        self.blocks_buf.push(asm!("cmove rcx, rdx"));
                    }
                    CompareOp::NotEq => {
                        self.blocks_buf.push(asm!("cmovne rcx, rdx"));
                    }
                    CompareOp::Bigger => {
                        self.blocks_buf.push(asm!("cmovg rcx, rdx"));
                    }
                    CompareOp::Smaller => {
                        self.blocks_buf.push(asm!("cmovl rcx, rdx"));
                    }
                    CompareOp::BiggerEq => {
                        self.blocks_buf.push(asm!("cmovge rcx, rdx"));
                    }
                    CompareOp::SmallerEq => {
                        self.blocks_buf.push(asm!("cmovle rcx, rdx"));
                    }
                }
                self.blocks_buf.push(asm!("push rcx"));
            }
            Expr::Binary(b) => {
                self.compile_expr(b.left.as_ref());
                self.compile_expr(b.right.as_ref());
                self.blocks_buf.push(asm!("pop rax"));
                self.blocks_buf.push(asm!("pop rbx"));
                match b.op {
                    Op::Plus => {
                        self.blocks_buf.push(asm!("add rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    }
                    Op::Sub => {
                        self.blocks_buf.push(asm!("sub rbx, rax"));
                        self.blocks_buf.push(asm!("push rbx"));
                    }
                    Op::Multi => {
                        self.blocks_buf.push(asm!("imul rax, rbx"));
                        self.blocks_buf.push(asm!("push rax"));
                    }
                    Op::Devide => {
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
        file.write_all(b";; This File is Automatically Created Useing Nemet Parser\n")?;
        file.write_all(b";; Under MIT License Copyright MahanFarzaneh 2023-2024\n\n")?;
        for line in &self.static_var_buf {
            file.write_all(line.as_bytes())?;
        }
        file.write_all(b"\n")?;
        // TODO: Add this to the section related
        file.write_all(b"section .text\n")?;
        file.write_all(b"global _start\n")?;

        file.write_all(b"print:\n")?;
        file.write_all(b"    push    rbp\n")?;
        file.write_all(b"    mov     rbp, rsp\n")?;
        file.write_all(b"    sub     rsp, 64\n")?;
        file.write_all(b"    mov     qword [rbp-56], rdi\n")?;
        file.write_all(b"    mov     qword [rbp-8], 1\n")?;
        file.write_all(b"    mov     eax, 32\n")?;
        file.write_all(b"    sub     rax, qword [rbp-8]\n")?;
        file.write_all(b"    mov     BYTE [rbp-48+rax], 10\n")?;
        file.write_all(b".L3:\n")?;
        file.write_all(b"    mov     rcx, qword [rbp-56]\n")?;
        file.write_all(b"    mov     rdx, -3689348814741910323\n")?;
        file.write_all(b"    mov     rax, rcx\n")?;
        file.write_all(b"    mul     rdx\n")?;
        file.write_all(b"    shr     rdx, 3\n")?;
        file.write_all(b"    mov     rax, rdx\n")?;
        file.write_all(b"    sal     rax, 2\n")?;
        file.write_all(b"    add     rax, rdx\n")?;
        file.write_all(b"    add     rax, rax\n")?;
        file.write_all(b"    sub     rcx, rax\n")?;
        file.write_all(b"    mov     rdx, rcx\n")?;
        file.write_all(b"    mov     eax, edx\n")?;
        file.write_all(b"    lea     edx, [rax+48]\n")?;
        file.write_all(b"    mov     eax, 31\n")?;
        file.write_all(b"    sub     rax, qword [rbp-8]\n")?;
        file.write_all(b"    mov     byte [rbp-48+rax], dl\n")?;
        file.write_all(b"    add     qword [rbp-8], 1\n")?;
        file.write_all(b"    mov     rax, qword [rbp-56]\n")?;
        file.write_all(b"    mov     rdx, -3689348814741910323\n")?;
        file.write_all(b"    mul     rdx\n")?;
        file.write_all(b"    mov     rax, rdx\n")?;
        file.write_all(b"    shr     rax, 3\n")?;
        file.write_all(b"    mov     qword [rbp-56], rax\n")?;
        file.write_all(b"    cmp     qword [rbp-56], 0\n")?;
        file.write_all(b"    jne     .L3\n")?;
        file.write_all(b"    mov     eax, 32\n")?;
        file.write_all(b"    sub     rax, qword [rbp-8]\n")?;
        file.write_all(b"    lea     rdx, [rbp-48]\n")?;
        file.write_all(b"    add     rax, rdx\n")?;
        file.write_all(b"    mov     rsi, rax\n")?;
        file.write_all(b"    mov     rbx, qword [rbp-8]\n")?;
        file.write_all(b"    mov     rdx, rbx\n")?;
        file.write_all(b"    mov     rdi, 1\n")?;
        file.write_all(b"    mov     rax, 1\n")?;
        file.write_all(b"    syscall\n")?;
        file.write_all(b"    leave\n")?;
        file.write_all(b"    ret\n")?;

        for instruct in &self.blocks_buf {
            file.write_all(instruct.as_bytes())?;
        }

        file.flush().unwrap();
        Ok(())
    }
}

