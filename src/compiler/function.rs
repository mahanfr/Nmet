/**********************************************************************************************
*
*   compiler/function: compile funtions and function arguments
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
use std::collections::HashMap;

use crate::{
    codegen::{memory::MemAddr, mnemonic::Mnemonic::*, register::Reg::*},
    compiler::{ScopeBlock, VariableMap},
    parser::{
        block::BlockType,
        function::{Function, FunctionArg},
        types::VariableType,
    },
};

use super::{block::compile_block, function_args_register_sized, CompilerContext};

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
            let mem_acss = MemAddr::new_disp_s(map.vtype.item_size(), RBP, map.stack_offset());
            let reg = function_args_register_sized(args_count, &map.vtype);
            cc.codegen.instr2(Mov, mem_acss, reg);
        } else {
            todo!();
        }
        cc.variables_map.insert(ident, map);
        cc.mem_offset += 8;
        cc.codegen.instr2(Sub, RSP, 8);
    }
}

pub fn compile_function(cc: &mut CompilerContext, f: &Function) {
    cc.scoped_blocks = Vec::new();
    cc.block_id = 0;
    cc.scoped_blocks
        .push(ScopeBlock::new(0, BlockType::Function(f.ident.clone())));
    cc.mem_offset = 0;
    cc.variables_map = HashMap::new();
    if f.ident == "main" {
        cc.codegen.set_lable("_start");
    } else {
        cc.codegen.set_lable(f.ident.clone());
    }

    cc.codegen.instr1(Push, RBP);
    cc.codegen.instr2(Mov, RBP, RSP);
    //cc.codegen.push_instr(Instr::sub(RSP, frame_size(cc.mem_offset)));
    function_args(cc, &f.args);
    compile_block(cc, &f.block, BlockType::Function(f.ident.clone()));
    cc.scoped_blocks.pop();
    // Call Exit Syscall
    if f.ident == "main" {
        cc.codegen.instr2(Mov, RAX, 60);
        cc.codegen.instr2(Mov, RDI, 0);
        cc.codegen.instr0(Syscall);
    } else {
        // revert rbp
        cc.codegen.instr0(Leave);
        cc.codegen.instr0(Ret);
    }
}
