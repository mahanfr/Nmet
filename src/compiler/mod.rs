pub mod bif;
mod expr;
mod function;
mod stmts;
mod variables;

use crate::codegen::instructions::{Instr, Opr};
use crate::codegen::{mnmemonic::Mnemonic::*, register::Reg, Codegen};
use crate::compiler::{bif::Bif, function::compile_function};
use crate::elf::generate_elf;
use crate::error_handeling::error;
use crate::output_generator::x86_64_nasm_generator;
use crate::parser::{
    block::{Block, BlockType},
    function::Function,
    parse_file,
    program::ProgramItem,
    stmt::StmtType,
    structs::StructDef,
    types::VariableType,
};
use std::collections::{HashMap, HashSet};

use self::stmts::compile_stmt;

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub offset_inner: usize,
    pub offset: usize,
    pub vtype: VariableType,
    pub vtype_inner: VariableType,
    pub is_mut: bool,
}

impl VariableMap {
    pub fn stack_offset(&self) -> usize {
        self.offset + self.vtype.size()
    }
}

pub type BLocation = (usize, usize);

pub struct ScopeBlock {
    pub id: usize,
    pub block_type: BlockType,
}
impl ScopeBlock {
    pub fn new(id: usize, block_type: BlockType) -> Self {
        Self { id, block_type }
    }
}

pub struct CompilerContext {
    pub codegen: Codegen,
    pub scoped_blocks: Vec<ScopeBlock>,
    pub block_id: usize,
    pub variables_map: HashMap<String, VariableMap>,
    pub functions_map: HashMap<String, Function>,
    pub structs_map: HashMap<String, StructDef>,
    pub bif_set: HashSet<Bif>,
    pub mem_offset: usize,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            codegen: Codegen::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            bif_set: HashSet::new(),
            variables_map: HashMap::new(),
            functions_map: HashMap::new(),
            structs_map: HashMap::new(),
            mem_offset: 0,
        }
    }
}

pub fn compile_to_elf(path: String) {
    let mut compiler_context = CompilerContext::new();

    compile(&mut compiler_context, path.clone());
    impl_bifs(&mut compiler_context);
    optimize(&mut compiler_context.codegen);
    generate_elf(path, &compiler_context);
}

pub fn compile_to_asm(path: String) {
    let mut compiler_context = CompilerContext::new();

    compile(&mut compiler_context, path.clone());
    impl_bifs(&mut compiler_context);
    optimize(&mut compiler_context.codegen);
    x86_64_nasm_generator(path, compiler_context.codegen).unwrap();
}

pub fn impl_bifs(cc: &mut CompilerContext) {
    for bif in cc.bif_set.iter() {
        bif.implement(&mut cc.codegen);
    }
}

fn optimize(code: &mut Codegen) {
    for i in 0..(code.instruct_buf.len() - 2) {
        if let Instr::Mov(op_a, op_b) = code.instruct_buf[i].clone() {
            if let Opr::R64(r) = op_a.clone() {
                if let Opr::Imm32(_) = op_b.clone() {
                    code.instruct_buf[i] = Instr::Mov(Opr::R32(r), op_b);
                }
            }
        }
        let Instr::Push(op_a) = code.instruct_buf[i].clone() else {
            continue;
        };
        let Instr::Pop(op_b) = code.instruct_buf[i + 1].clone() else {
            continue;
        };
        if op_a == op_b {
            code.instruct_buf[i] = Instr::Nop;
            code.instruct_buf[i + 1] = Instr::Nop;
        } else {
            code.instruct_buf[i] = Instr::new_instr2(Mov, op_b, op_a);
            code.instruct_buf[i + 1] = Instr::Nop;
        }
    }
    code.instruct_buf.retain(|x| x != &Instr::Nop);
}

// fn x86_64_nasm_cleanup(code: &mut Codegen) {
//     for i in 0..(code.instruct_buf.len() - 2) {
//         if code.instruct_buf[i].trim_start().starts_with("push")
//             && code.instruct_buf[i + 1].trim_start().starts_with("pop")
//         {
//             let merged: String = merge_instr(&code.instruct_buf[i], &code.instruct_buf[i + 1]);
//             code.instruct_buf[i].clear();
//             code.instruct_buf[i + 1] = merged;
//         }
//     }
//     code.instruct_buf.retain(|x| !x.is_empty());
// }
//
// fn merge_instr(ins1: &str, inst2: &str) -> String {
//     let data1 = ins1.split(' ').last().unwrap();
//     let data2 = inst2.split(' ').last().unwrap();
//     if data1 == data2 {
//         String::new()
//     } else {
//         Instr::new_instr2(Mnemonic::Mov, data2.to_string(), data1.to_string()).to_string().to_string()
//         // format!("    mov {data2}, {data1}")
//     }
// }
//
pub fn function_args_register_sized(arg_numer: usize, vtype: &VariableType) -> Reg {
    match arg_numer {
        0 => Reg::DI_sized(vtype),
        1 => Reg::Si_sized(vtype),
        2 => Reg::DX_sized(vtype),
        3 => Reg::CX_sized(vtype),
        4 => Reg::R8_sized(vtype),
        5 => Reg::R9_sized(vtype),
        _ => unreachable!(),
    }
}

pub fn function_args_register(arg_numer: usize) -> Reg {
    match arg_numer {
        0 => Reg::RDI,
        1 => Reg::RSI,
        2 => Reg::RDX,
        3 => Reg::RCX,
        4 => Reg::R8,
        5 => Reg::R9,
        _ => unreachable!(),
    }
}

fn frame_size(mem_offset: usize) -> usize {
    2 << mem_offset.ilog2() as usize
}

pub fn compile_lib(cc: &mut CompilerContext, path: String, exports: Vec<String>) {
    let program = parse_file(path);
    let is_importable = |ident: &String| {
        if !exports.is_empty() {
            exports.contains(ident)
        } else {
            true
        }
    };
    for item in program.items {
        match item {
            ProgramItem::Struct(_) => {
                todo!();
            }
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                if is_importable(&f.ident) && !cc.functions_map.contains_key(&f.ident) {
                    cc.functions_map.insert(f.ident.clone(), f.clone());
                }
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents);
            }
        }
    }
}

// TODO: Handle Compilation Error
pub fn compile(cc: &mut CompilerContext, path: String) {
    let program = parse_file(path);
    for item in program.items {
        match item {
            ProgramItem::Struct(s) => {
                cc.structs_map.insert(s.ident.clone(), s.clone());
            }
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                cc.functions_map.insert(f.ident.clone(), f.clone());
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents);
            }
        }
    }
    let functions = cc.functions_map.clone();
    for f in functions.values() {
        compile_function(cc, f);
    }
    assert!(
        cc.scoped_blocks.is_empty(),
        "Somting went wrong: Scope has not been cleared"
    );
}

/*
 *  keep in mind there could be a problem when a variable wants to access
 *  somthing that added after in code but it could be a feature too :)
 */
fn compile_block(cc: &mut CompilerContext, block: &Block, block_type: BlockType) {
    cc.block_id += 1;
    cc.scoped_blocks
        .push(ScopeBlock::new(cc.block_id, block_type));
    for stmt in &block.stmts {
        match stmt.stype {
            StmtType::Break => {
                let mut did_break: bool = false;
                for s_block in cc.scoped_blocks.iter().rev() {
                    if let BlockType::Loop(loc) = s_block.block_type {
                        cc.codegen.instr1(Jmp, format!(".LE{}", loc.1));
                        did_break = true;
                        break;
                    }
                }
                if !did_break {
                    error("Can not break out of non-loop blocks!", stmt.loc.clone());
                }
            }
            StmtType::Continue => {
                let mut did_cont: bool = false;
                for s_block in cc.scoped_blocks.iter().rev() {
                    if let BlockType::Loop(loc) = s_block.block_type {
                        cc.codegen.instr1(Jmp, format!(".L{}", loc.0));
                        did_cont = true;
                        break;
                    }
                }
                if !did_cont {
                    error("Can not continue in non-loop blocks!", stmt.loc.clone());
                }
            }
            _ => {
                compile_stmt(cc, stmt);
            }
        }
    }
    cc.block_id -= 1;
    cc.scoped_blocks.pop().unwrap();
}
