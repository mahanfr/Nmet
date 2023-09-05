use crate::{compiler::VariableMap, parser::{variable_decl::VariableDeclare, expr::CompareExpr}, llvm::expr::{self, compile_expr}, error_handeling::error};

use super::CompilerContext;

pub fn find_variable(cc: &CompilerContext, ident: String) -> Option<VariableMap> {
    let map_ident = ident;
    let map = cc.variables_map.get(&map_ident);
    if let Some(map) = map {
        return Some(map.clone());
    }
    None
}

pub fn insert_variable(cc: &mut CompilerContext, var: &VariableDeclare) -> Result<(), String> {
    let mut vtype = var.v_type.clone().unwrap();
    let code = format!("%{} = alloca {}, align {}\n",var.ident,vtype.to_llvm_type(),vtype.size());
    cc.instruct_buf.push(code);
    match &var.init_value {
        Some(v) => {
            let (tag,ttype) = compile_expr(cc,&v);
            let code = format!("store {ttype} {tag}, ptr %{}, align {}",var.ident.clone(),ttype.size());
            cc.instruct_buf.push(code);
            vtype = match vtype.cast(&ttype){
               Ok(t) => t,
               Err(msg) => return Err(msg),
            }
        },
        None => {}
    }
    cc.variables_map.insert(var.ident.clone(),VariableMap {
        _ident: var.ident.clone(),
        is_mut: var.mutable,
        offset: cc.mem_offset,
        vtype,
    });
    Ok(())
}

pub fn get_vriable_map(cc: &mut CompilerContext, var_ident: &str) -> Option<VariableMap> {
    find_variable(cc, var_ident.to_owned())
}
