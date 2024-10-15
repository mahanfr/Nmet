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
    codegen::{mnemonic::Mnemonic::*, register::Reg::*},
    compiler::VariableMap,
    parser::{block::Block, function::{FunctionArg, FunctionDef}},
};

use super::{
    block::compile_function_block_alrady_scoped, function_args_register_sized, CompilerContext,
    VariableMapBase,
};

pub fn function_args(cc: &mut CompilerContext, block: &Block, args: &[FunctionArg]) {
    for (args_count, arg) in args.iter().enumerate() {
        let ident = format!("{}%{}", arg.ident, cc.scoped_blocks.last().unwrap().id);
        let map = VariableMap::new(
            VariableMapBase::Stack(block.id.to_string()),
            cc.mem_offset,
            arg.typedef.clone(),
            false,
        );
        if args_count < 6 {
            let mem_acss = map.mem();
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

pub fn compile_function(cc: &mut CompilerContext, f: &FunctionDef) {
    cc.scoped_blocks = Vec::new();
    cc.scoped_blocks.push(f.block.clone());
    cc.mem_offset = 0;
    cc.variables_map = HashMap::new();
    cc.codegen.set_lable(f.block.start_name());

    cc.codegen.instr1(Push, RBP);
    cc.codegen.instr2(Mov, RBP, RSP);
    function_args(cc, &f.block, &f.decl.args);
    /*--- Scoping function variables ---*/
    cc.scoped_blocks.push(f.block.clone());
    compile_function_block_alrady_scoped(cc, &f.block);
    //compile_block(cc, &f.block);
    cc.scoped_blocks.pop();
    // revert rbp
    cc.codegen.set_lable(f.block.end_name());
    cc.codegen.instr1(Push, RAX);
    // TODO: Issue a warning for assgigning variables in defer block
    compile_function_block_alrady_scoped(cc, &f.defer_block);
    cc.scoped_blocks.pop().unwrap();
    /*--- Unscoping function variables ---*/
    cc.codegen.instr1(Pop, RAX);

    cc.codegen.instr0(Leave);
    cc.codegen.instr0(Ret);
}
