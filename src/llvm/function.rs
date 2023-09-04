use std::collections::HashMap;

use crate::{
    asm,
    parser::function::{Function, FunctionArg},
};

use super::{
    compile_block, frame_size, function_args_register_sized, mem_word, variables::VariableMap,
    CompilerContext,
};

pub fn function_args(cc: &mut CompilerContext, args: &[FunctionArg]) {
    for (args_count, arg) in args.iter().enumerate() {
        let ident = format!("{}%{}", arg.ident, cc.block_id);
        let map = VariableMap {
            _ident: arg.ident.clone(),
            offset: cc.mem_offset,
            is_mut: false,
            vtype: arg.typedef.clone(),
        };
        if args_count < 6 {
            let mem_acss = format!(
                "{} [rbp-{}]",
                mem_word(&map.vtype),
                map.offset + map.vtype.size()
            );
            let reg = function_args_register_sized(args_count, &map.vtype);
            cc.instruct_buf.push(asm!("mov {},{}", mem_acss, reg));
        } else {
            todo!();
            // let mem_overload = format!("{} [rbp+{}]", mem_word(8), 16 + (args_count - 6) * 8);
            //let mem_acss = format!("{} [rbp-{}]", mem_word(8), map.offset + map.size);
            //cc.instruct_buf
            //    .push(asm!("mov {},{}", mem_acss, mem_overload));
        }
        cc.variables_map.insert(ident, map);
        cc.mem_offset += 8;
    }
}

pub fn compile_function(cc: &mut CompilerContext, f: &Function) {
    cc.scoped_blocks = Vec::new();
    cc.block_id = 0;
    cc.scoped_blocks.push(0);
    cc.mem_offset = 0;
    cc.variables_map = HashMap::new();
    if f.ident == "main" {
        cc.instruct_buf.push("_start:\n".to_string());
    } else {
        cc.instruct_buf.push(format!("{}:\n", f.ident));
    }

    // set rbp to stack pointer for this block
    let index_1 = cc.instruct_buf.len();
    cc.instruct_buf.push(String::new());
    let index_2 = cc.instruct_buf.len();
    cc.instruct_buf.push(String::new());
    let index_3 = cc.instruct_buf.len();
    cc.instruct_buf.push(String::new());

    function_args(cc, &f.args);
    compile_block(cc, &f.block);
    cc.scoped_blocks.pop();
    // Call Exit Syscall
    if !cc.variables_map.is_empty() {
        cc.instruct_buf[index_1] = asm!("push rbp");
        cc.instruct_buf[index_2] = asm!("mov rbp, rsp");
        cc.instruct_buf[index_3] = asm!("sub rsp, {}", frame_size(cc.mem_offset));
    }
    if f.ident == "main" {
        cc.instruct_buf.push(asm!("mov rax, 60"));
        cc.instruct_buf.push(asm!("mov rdi, 0"));
        cc.instruct_buf.push(asm!("syscall"));
    } else {
        // revert rbp
        if !cc.variables_map.is_empty() {
            //cc.instruct_buf.push(asm!("pop rbp"));
            cc.instruct_buf.push(asm!("leave"));
            cc.instruct_buf.push(asm!("ret"));
        } else {
            cc.instruct_buf.push(asm!("ret"));
        }
    }
}
