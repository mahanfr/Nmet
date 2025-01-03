use std::collections::HashMap;

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
    assembler::{
        instructions::Opr,
        memory::MemAddr,
        mnemonic::Mnemonic::*,
        register::Reg::{self, *},
        utils::mov_unknown_to_register,
    },
    error_handeling::{error, CompilationError},
    memq,
    parser::{block::Block, types::VariableType, variable_decl::VariableDeclare},
};

use super::{expr::compile_expr, CompilerContext, NSType};

#[derive(Debug, Clone)]
pub enum VariableMapBase {
    Stack(String),
    Global(String),
}

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub base: VariableMapBase,
    pub offset: i32,
    pub vtype: VariableType,
    pub is_mut: bool,
}

impl VariableMap {
    pub fn new(base: VariableMapBase, offset: usize, vtype: VariableType, is_mut: bool) -> Self {
        let new_offset = Self::get_stack_offset(offset, &vtype);
        Self {
            base,
            is_mut,
            offset: new_offset,
            vtype,
        }
    }

    pub fn is_global(&self) -> bool {
        matches!(self.base, VariableMapBase::Global(_))
    }

    pub fn mem_with_offset_reg(&self, offset_reg: Reg) -> MemAddr {
        MemAddr::new_sib_s(
            self.vtype.item_size(),
            Reg::RBP,
            self.offset,
            offset_reg,
            self.vtype.item_size(),
        )
    }

    pub fn mem(&self) -> MemAddr {
        if let VariableMapBase::Global(g) = &self.base {
            return MemAddr::new_rela_s(self.vtype.item_size(), g.to_string());
        }
        match &self.vtype {
            VariableType::Int | VariableType::UInt => MemAddr::new_disp_s(4, Reg::RBP, self.offset),
            VariableType::Long
            | VariableType::ULong
            | VariableType::Custom(_)
            | VariableType::Pointer
            | VariableType::String => MemAddr::new_disp_s(8, Reg::RBP, self.offset),
            VariableType::Bool | VariableType::Char => {
                MemAddr::new_disp_s(1, Reg::RBP, self.offset)
            }
            VariableType::Any | VariableType::Void => unreachable!(),
            VariableType::Array(item_vtype, _) => {
                MemAddr::new_disp_s(item_vtype.item_size(), Reg::RBP, self.offset)
            }
            VariableType::Struct(_) => MemAddr::new_disp_s(8, Reg::RBP, self.offset),
            VariableType::Float => todo!(),
        }
    }

    pub fn get_stack_offset(offset: usize, vtype: &VariableType) -> i32 {
        -((offset + vtype.size()) as i32)
    }
}

#[derive(Debug, Clone)]
pub struct NameSpaceMapping {
    items: HashMap<String, Vec<VariableMap>>,
}

impl NameSpaceMapping {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    pub fn insert(&mut self, ident: &str, value: VariableMap) -> Result<(), CompilationError> {
        if value.is_global() {
            if self.items.contains_key(ident) {
                CompilationError::Err(format!(
                    "Local Variable with the name ({ident}) already exists"
                ));
            }
        } else if let Some(bucket) = self.items.get(ident) {
            for item in bucket {
                if item.is_global() {
                    CompilationError::Err(format!(
                        "Global Variable with the name ({ident}) already exists"
                    ));
                }
            }
        }
        self.items.entry(ident.to_string()).or_default().push(value);
        Ok(())
    }
    pub fn get(&self, ident: &str, block: &Block) -> Result<VariableMap, CompilationError> {
        let Some(bucket) = self.items.get(ident) else {
            return Err(CompilationError::UndefinedVariable(ident.to_string()));
        };
        for item in bucket {
            match &item.base {
                VariableMapBase::Global(_) => {
                    return Ok(item.clone());
                }
                VariableMapBase::Stack(id) => {
                    if *id == block.id || block.id.starts_with(id) {
                        return Ok(item.clone());
                    }
                }
            }
        }
        Err(CompilationError::UndefinedVariable(ident.to_string()))
    }

    pub fn purge(&mut self) {
        let copy = self.items.clone();
        for bucket in copy {
            if let Some(map) = bucket.1.first() {
                if !map.is_global() {
                    self.items.remove(&bucket.0);
                }
            } else {
                self.items.remove(&bucket.0);
            }
        }
    }
}

pub fn insert_variable(
    cc: &mut CompilerContext,
    block: &Block,
    var: &VariableDeclare,
    var_base: VariableMapBase,
) -> Result<(), CompilationError> {
    let mut vtype = var.v_type.clone();
    let mut var_base = var_base;
    // Declare variable memory
    // No need to do any thing if variable is on the stack
    if let VariableType::Custom(s) = &vtype {
        let Some(NSType::Struct(struct_map)) = cc.namespace_map.get(s) else {
            return Err(CompilationError::UnknownType(s.to_owned()));
        };
        let struct_tag = cc.codegen.add_bss_seg(struct_map.size());
        let mem_acss = memq!(RBP, -(cc.mem_offset as i32 + 8));
        cc.codegen.instr2(Mov, mem_acss, Opr::Rela(struct_tag));
        vtype = VariableType::Struct(struct_map.clone());
    }
    // compile initial value
    if var.init_value.is_some() {
        let init_value = var.init_value.clone().unwrap();
        let expro = compile_expr(cc, block, &init_value)?;
        match vtype.cast(&expro.vtype) {
            Ok(vt) => {
                let mem_acss = match &var_base {
                    VariableMapBase::Stack(_) => MemAddr::new_disp_s(
                        vt.item_size(),
                        RBP,
                        -((cc.mem_offset + vt.size()) as i32),
                    ),
                    VariableMapBase::Global(_) => {
                        let tag = cc.codegen.add_bss_seg(vt.size());
                        var_base = VariableMapBase::Global(tag.to_string());
                        MemAddr::new_rela_s(vt.item_size(), tag.to_string())
                    }
                };
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
    match &var_base {
        VariableMapBase::Stack(_) => {
            let var_map = VariableMap::new(var_base, cc.mem_offset, vtype.clone(), var.mutable);
            cc.codegen.instr2(Sub, RSP, vtype.size());
            cc.mem_offset += vtype.size();
            let _ = cc.variables_map.insert(&var.ident, var_map);
        }
        VariableMapBase::Global(_) => {
            let var_map = VariableMap::new(var_base, 0, vtype.clone(), var.mutable);
            let _ = cc.variables_map.insert(&var.ident, var_map);
        }
    }
    Ok(())
}
