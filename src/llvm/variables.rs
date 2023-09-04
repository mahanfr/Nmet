use crate::{
    asm,
    error_handeling::error,
    parser::{types::VariableType, variable_decl::VariableDeclare},
};

use super::{expr::compile_expr, mem_word, rbs, CompilerContext};

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub _ident: String,
    pub offset: usize,
    pub vtype: VariableType,
    pub is_mut: bool,
}

pub fn find_variable(cc: &CompilerContext, ident: String) -> Option<VariableMap> {
    for block_id in &cc.scoped_blocks {
        let map_ident = format!("{ident}%{}", block_id);
        let map = cc.variables_map.get(&map_ident);
        if let Some(map) = map {
            return Some(map.clone());
        }
    }
    None
}

pub fn insert_variable(cc: &mut CompilerContext, var: &VariableDeclare) -> Result<(), String> {
    let ident = format!("{}%{}", var.ident, cc.block_id);
    let mut vtype = match var.v_type.as_ref() {
        Some(vt) => vt.clone(),
        None => VariableType::Any,
    };
    if var.init_value.is_some() {
        let init_value = var.init_value.clone().unwrap();
        // this pushes result in stack
        let texpr = compile_expr(cc, &init_value);
        match vtype.cast(&texpr) {
            Ok(vt) => {
                let mem_acss = format!("{} [rbp-{}]", mem_word(&vt), cc.mem_offset + vt.size());
                cc.instruct_buf.push(asm!("pop rax"));
                cc.instruct_buf
                    .push(asm!("mov {mem_acss},{}", rbs("a", &vt)));
                vtype = vt;
            }
            Err(msg) => {
                error(msg, init_value.loc.clone());
            }
        }
    }
    if vtype == VariableType::Any {
        return Err(format!(
            "Variable ({}) type is not known at compile time",
            var.ident
        ));
    }
    let var_map = VariableMap {
        _ident: var.ident.clone(),
        vtype: vtype.clone(),
        offset: cc.mem_offset,
        is_mut: var.mutable,
    };
    cc.mem_offset += vtype.size();
    cc.variables_map.insert(ident, var_map);
    Ok(())
}

pub fn get_vriable_map(cc: &mut CompilerContext, var_ident: &str) -> Option<VariableMap> {
    find_variable(cc, var_ident.to_owned())
}
