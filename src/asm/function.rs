use std::collections::HashMap;

use crate::{
    asm,
    compiler::{ScopeBlock, VariableMap},
    parser::{
        block::BlockType,
        function::{Function, FunctionArg},
        types::VariableType,
    },
};

use super::{compile_block, frame_size, function_args_register_sized, mem_word, CompilerContext};

pub fn function_args(cc: &mut CompilerContext, args: &[FunctionArg]) {
    for (args_count, arg) in args.iter().enumerate() {
        let ident = format!("{}%{}", arg.ident, cc.block_id);
        let map = VariableMap {
            offset: cc.mem_offset,
            offset_inner: 0,
            is_mut: false,
            vtype: arg.typedef.clone(),
            vtype_inner: VariableType::Any,
        };
        if args_count < 6 {
            let mem_acss = format!(
                "{} [rbp-{}]",
                mem_word(&map.vtype),
                map.offset + map.vtype.size()
            );
            let reg = function_args_register_sized(args_count, &map.vtype);
            cc.codegen.mov(mem_acss, reg);
        } else {
            todo!();
            // let mem_overload = format!("{} [rbp+{}]", mem_word(8), 16 + (args_count - 6) * 8);
            //let mem_acss = format!("{} [rbp-{}]", mem_word(8), map.offset + map.size);
            //cc.instruct_buf
            //    .push(asm!("mov {},{}", mem_acss, mem_overload));
        }
        cc.variables_map.insert(ident, map);
        cc.mem_offset += 8;
    }
}

pub fn compile_function(cc: &mut CompilerContext, f: &Function) {
    cc.scoped_blocks = Vec::new();
    cc.block_id = 0;
    cc.scoped_blocks
        .push(ScopeBlock::new(0, BlockType::Function));
    cc.mem_offset = 0;
    cc.variables_map = HashMap::new();
    if f.ident == "main" {
        cc.codegen.tag("_start");
    } else {
        cc.codegen.tag(f.ident.clone());
    }

    // set rbp to stack pointer for this block
    let index_1 = cc.codegen.place_holder();
    let index_2 = cc.codegen.place_holder();
    let index_3 = cc.codegen.place_holder();

    function_args(cc, &f.args);
    compile_block(cc, &f.block, BlockType::Function);
    cc.scoped_blocks.pop();
    // Call Exit Syscall
    if !cc.variables_map.is_empty() {
        cc.codegen
            .insert_into_raw(index_1, asm!("push rbp"))
            .unwrap();
        cc.codegen
            .insert_into_raw(index_2, asm!("mov rbp, rsp"))
            .unwrap();
        cc.codegen
            .insert_into_raw(index_3, asm!("sub rsp, {}", frame_size(cc.mem_offset)))
            .unwrap();
    }
    if f.ident == "main" {
        cc.codegen.mov("rax", 60);
        cc.codegen.mov("rdi", 0);
        cc.codegen.syscall();
    } else {
        // revert rbp
        if !cc.variables_map.is_empty() {
            //cc.instruct_buf.push(asm!("pop rbp"));
            cc.codegen.leave();
            cc.codegen.ret();
        } else {
            cc.codegen.ret();
        }
    }
}
