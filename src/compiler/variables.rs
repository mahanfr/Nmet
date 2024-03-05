/**********************************************************************************************
*
*   compiler/variables: Compile Variables
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
use crate::{
    codegen::{
        instructions::Opr, memory::MemAddr, mnemonic::Mnemonic::*,
        utils::mov_unknown_to_register, register::Reg::*,
    },
    compiler::VariableMap,
    error_handeling::{error, CompilationError},
    memq,
    parser::{types::VariableType, variable_decl::VariableDeclare},
};

use super::{expr::compile_expr, CompilerContext};

pub fn insert_variable(
    cc: &mut CompilerContext,
    var: &VariableDeclare,
) -> Result<(), CompilationError> {
    let ident = format!("{}%{}", var.ident, cc.scoped_blocks.last().unwrap().id);
    let mut vtype = match var.v_type.as_ref() {
        Some(vt) => vt.clone(),
        None => VariableType::Any,
    };
    // Declare variable memory
    // No need to do any thing if variable is on the stack
    match &vtype {
        VariableType::Array(_, _) => (),
        VariableType::Custom(s) => {
            let Some(struct_map) = cc.structs_map.get(s) else {
                return Err(CompilationError::UnknownType(s.to_owned()));
            };
            let size: usize = struct_map.items.iter().map(|(_, v)| v.size()).sum();
            let struct_tag = cc.codegen.add_bss_seg(size);
            let mem_acss = memq!(RBP, -(cc.mem_offset as i32 + 8));
            // assert!(false, "Not Implemented yet!");
            cc.codegen.instr2(Mov, mem_acss, Opr::Rela(struct_tag));
        }
        VariableType::String => {}
        _ => (),
    }
    // compile initial value
    if var.init_value.is_some() {
        let init_value = var.init_value.clone().unwrap();
        let expro = compile_expr(cc, &init_value)?;
        match vtype.cast(&expro.vtype) {
            Ok(vt) => {
                let mem_acss =
                    MemAddr::new_disp_s(vt.item_size(), RBP, -((cc.mem_offset + vt.size()) as i32));
                if expro.value.is_register() {
                    cc.codegen.instr2(Mov, mem_acss, expro.value.sized(&vt));
                } else {
                    mov_unknown_to_register(cc, RAX, expro.value);
                    cc.codegen
                        .instr2(Mov, mem_acss, RAX.convert(vt.item_size()));
                }
                vtype = vt;
            }
            Err(msg) => {
                error(msg, init_value.loc.clone());
            }
        }
    }
    // Type checking
    if vtype == VariableType::Any {
        return Err(CompilationError::UnknownType(var.ident.to_owned()));
    }
    let var_map = VariableMap {
        vtype: vtype.clone(),
        vtype_inner: VariableType::Any,
        offset: cc.mem_offset,
        is_mut: var.mutable,
        offset_inner: 0,
    };
    cc.codegen.instr2(Sub, RSP, vtype.size());
    cc.mem_offset += vtype.size();
    cc.variables_map.insert(ident, var_map);
    Ok(())
}

pub fn get_vriable_map(
    cc: &mut CompilerContext,
    var_ident: &str,
) -> Result<VariableMap, CompilationError> {
    for scoped_block in &cc.scoped_blocks {
        let map_ident = format!("{var_ident}%{}", scoped_block.id);
        if let Some(map) = cc.variables_map.get(&map_ident) {
            return Ok(map.clone());
        }
    }
    Err(CompilationError::UndefinedVariable(var_ident.to_owned()))
}
