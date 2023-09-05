use crate::parser::function::{Function, FunctionArg};

use super::{CompilerContext, compile_block};

pub fn function_args(cc: &mut CompilerContext, args: &[FunctionArg]) {
    todo!()
}

pub fn compile_function(cc: &mut CompilerContext, f: &Function) {
    if f.ident == "main" {
        cc.instruct_buf.push("define i32 @main() {\nentry:\n".to_string());
    }
    compile_block(cc,&f.block);
    cc.instruct_buf.push("}".to_string());
}
