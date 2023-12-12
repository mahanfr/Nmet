use crate::{
    asm,
    compiler::VariableMap,
    error_handeling::error,
    parser::{types::VariableType, variable_decl::VariableDeclare},
};

use super::{expr::compile_expr, mem_word, rbs, CompilerContext};

pub fn find_variable(cc: &CompilerContext, ident: String) -> Option<VariableMap> {
    for scoped_block in &cc.scoped_blocks {
        let map_ident = format!("{ident}%{}", scoped_block.id);
        let map = cc.variables_map.get(&map_ident);
        if let Some(map) = map {
            return Some(map.clone());
        }
    }
    None
}

fn _insert_global_variable(cc: &mut CompilerContext, atype: &VariableType, asize: &usize) {
    let bss_tag = format!("arr{}", cc.bss_buf.len());
    cc.bss_buf
        .push(format!("{}: resb {}", bss_tag, atype.item_size() * asize));
    let mem_acss = format!(
        "{} [rbp-{}]",
        mem_word(&VariableType::Pointer),
        cc.mem_offset + 8
        );
    cc.instruct_buf.push(asm!("mov {mem_acss},{}", bss_tag));
}

pub fn insert_variable(cc: &mut CompilerContext, var: &VariableDeclare) -> Result<(), String> {
    let ident = format!("{}%{}", var.ident, cc.block_id);
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
                return Err("Type dose not exists in the current scope".to_string());
            };
            let size: usize = struct_map.items.iter().map(|(_, v)| v.size()).sum();
            let struct_tag = format!("{}{}", struct_map.ident.clone(), cc.bss_buf.len());
            cc.bss_buf.push(format!("{struct_tag}: resb {}", size));
            let mem_acss = format!(
                "{} [rbp-{}]",
                mem_word(&VariableType::Pointer),
                cc.mem_offset + 8
            );
            cc.instruct_buf.push(asm!("mov {mem_acss}, {struct_tag}"));
        }
        VariableType::String => {}
        _ => (),
    }
    // compile initial value
    if var.init_value.is_some() {
        let init_value = var.init_value.clone().unwrap();
        // this pushes result in stack
        let texpr = compile_expr(cc, &init_value);
        // TODO: Strings should include size
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
    // Type checking
    if vtype == VariableType::Any {
        return Err(format!(
            "Variable ({}) type is not known at compile time",
            var.ident
        ));
    }
    let var_map = VariableMap {
        vtype: vtype.clone(),
        vtype_inner: VariableType::Any,
        offset: cc.mem_offset,
        is_mut: var.mutable,
        offset_inner: 0,
    };
    cc.mem_offset += vtype.size();
    cc.variables_map.insert(ident, var_map);
    Ok(())
}

pub fn get_vriable_map(cc: &mut CompilerContext, var_ident: &str) -> Option<VariableMap> {
    find_variable(cc, var_ident.to_owned())
}
