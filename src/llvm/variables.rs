use crate::{compiler::VariableMap, parser::variable_decl::VariableDeclare};

use super::CompilerContext;

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
    todo!();
}

pub fn get_vriable_map(cc: &mut CompilerContext, var_ident: &str) -> Option<VariableMap> {
    find_variable(cc, var_ident.to_owned())
}
