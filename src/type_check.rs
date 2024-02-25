use crate::{
    codegen::optimization::ExprOpr,
    compiler::CompilerContext,
    error_handeling::CompilationError,
    parser::types::VariableType,
};

pub fn tc_compare_expr(
    _: &CompilerContext,
    left: &ExprOpr,
    right: &ExprOpr,
) -> Result<VariableType, CompilationError> {
    if left.vtype == right.vtype {
        match left.vtype {
            VariableType::String | VariableType::Array(_, _) | VariableType::Custom(_) => {
                unimplemented!()
            }
            _ => Ok(left.vtype.to_owned()),
        }
    } else {
        left.vtype.cast(&right.vtype)
    }
}
