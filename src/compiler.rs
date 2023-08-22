use std::collections::HashMap;
use std::error::Error;

use crate::asm_generator::x86_64_nasm_generator;
use crate::error_handeling::error;
use crate::nemet_macros::{Macro, MacroCall};
use crate::parser::assign::{Assign, AssignOp};
use crate::parser::block::Block;
use crate::parser::expr::{CompareOp, Expr, ExprType, FunctionCall, Op, UnaryExpr};
use crate::parser::function::{Function, FunctionArg};
use crate::parser::parse_file;
use crate::parser::program::ProgramItem;
use crate::parser::stmt::{ElseBlock, IFStmt, Stmt, StmtType, WhileStmt};
use crate::parser::types::VariableType;
use crate::parser::variable_decl::VariableDeclare;

macro_rules! asm {
    ($($arg:tt)+) => (
        format!("    {}\n",format_args!($($arg)+))
    );
}

pub fn compile_to_asm(path: String) {
    let mut compiler_context = CompilerContext::new();

    let (instr_buf, data_buf) =
        compile(&mut compiler_context, path.clone()).expect("Can not Compile Program");
    x86_64_nasm_generator(path, instr_buf, data_buf).unwrap();
}

pub fn mem_word(size: usize) -> String {
    match size {
        1 => "byte".to_string(),
        2 => "word".to_string(),
        4 => "dword".to_string(),
        8 => "qword".to_string(),
        _ => {
            unreachable!("Incurrect Size")
        }
    }
}

pub fn rbs(register: &str, size: usize) -> String {
    match register {
        "a" | "b" | "c" | "d" => match size {
            1 => format!("{register}l"),
            2 => format!("{register}x"),
            4 => format!("e{register}x"),
            8 => format!("r{register}x"),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        "sp" | "bp" => match size {
            1 => format!("{register}l"),
            2 => register.to_string(),
            4 => format!("e{register}"),
            8 => format!("r{register}"),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        "si" | "di" => match size {
            1 => format!("{register}l"),
            2 => register.to_string(),
            4 => format!("e{register}"),
            8 => format!("r{register}"),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        "r8" | "r9" | "r10" | "r11" => match size {
            1 => format!("{register}b"),
            2 => format!("{register}w"),
            4 => format!("{register}d"),
            8 => register.to_string(),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        _ => {
            panic!("Wrong register identifier!");
        }
    }
}

pub fn function_args_register(arg_numer: usize, size: usize) -> String {
    match arg_numer {
        0 => rbs("di", size),
        1 => rbs("si", size),
        2 => rbs("d", size),
        3 => rbs("c", size),
        4 => rbs("r8", size),
        5 => rbs("r9", size),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct VariableMap {
    _ident: String,
    offset: usize,
    size: usize,
    item_size: usize,
    is_mut: bool,
}

pub struct CompilerContext {
    instruct_buf: Vec<String>,
    data_buf: Vec<String>,
    scoped_blocks: Vec<usize>,
    block_id: usize,
    variables_map: HashMap<String, VariableMap>,
    functions_map: HashMap<String, Function>,
    macros_map: HashMap<String, Macro>,
    mem_offset: usize,
}

impl CompilerContext {
    // TODO: handle Error for Parsing
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            data_buf: Vec::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            variables_map: HashMap::new(),
            functions_map: HashMap::new(),
            macros_map: HashMap::new(),
            mem_offset: 0,
        }
    }
}
fn frame_size(mem_offset: usize) -> usize {
    2 << mem_offset.ilog2() as usize
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

pub fn insert_variable(cc: &mut CompilerContext, var: &VariableDeclare) {
    let ident: String;
    let var_map: VariableMap;
    let mut size = 8;
    let mut item_size = 8;
    if var.v_type.is_some() {
        let typ = var.v_type.clone().unwrap();
        if let VariableType::Array(a, s) = typ {
            match *a.as_ref() {
                VariableType::Int => {
                    size = 8 * s;
                    item_size = 8;
                }
                VariableType::Char => {
                    size = s;
                    item_size = 1;
                }
                _ => {
                    todo!("Unsuported Array Type");
                }
            }
        }
    }
    if var.is_static {
        todo!();
    } else {
        ident = format!("{}%{}", var.ident, cc.block_id);
        var_map = VariableMap {
            _ident: var.ident.clone(),
            offset: cc.mem_offset,
            // TODO: Change size
            size,
            item_size,
            is_mut: var.mutable,
        };
    }
    cc.mem_offset += size;
    if var.init_value.is_some() {
        // TODO: Type check
        let init_value = var.init_value.clone().unwrap();
        // this pushes result in stack
        compile_expr(cc, &init_value);
        let mem_acss = format!(
            "{} [rbp-{}]",
            mem_word(var_map.item_size),
            var_map.offset + var_map.size
        );
        cc.instruct_buf.push(asm!("pop rax"));
        cc.instruct_buf
            .push(asm!("mov {mem_acss},{}", rbs("a", var_map.item_size)));
    }
    cc.variables_map.insert(ident, var_map);
}

pub fn function_args(cc: &mut CompilerContext, args: &[FunctionArg]) {
    for (args_count, arg) in args.iter().enumerate() {
        let ident = format!("{}%{}", arg.ident, cc.block_id);
        let map = VariableMap {
            _ident: arg.ident.clone(),
            offset: cc.mem_offset,
            is_mut: false,
            item_size: 8,
            size: 8,
        };
        if args_count < 6 {
            let mem_acss = format!("{} [rbp-{}]", mem_word(8), map.offset + map.size);
            let reg = function_args_register(args_count, 8);
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

pub fn function(cc: &mut CompilerContext, f: &Function) {
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

pub fn compile_lib(
    cc: &mut CompilerContext,
    path: String,
    exports: Vec<String>,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let program = parse_file(path);
    let is_importable = |ident: &String| {
        if !exports.is_empty() {
            return exports.contains(&ident);
        } else {
            return true;
        }
    };
    for item in program.items {
        match item {
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                if is_importable(&f.ident) && !cc.functions_map.contains_key(&f.ident) {
                    cc.functions_map.insert(f.ident.clone(), f.clone());
                }
            }
            ProgramItem::Macro(i, m) => {
                if is_importable(&i) && !cc.macros_map.contains_key(&i) {
                    cc.macros_map.insert(i, m);
                }
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents)?;
            }
        }
    }
    Ok((cc.instruct_buf.clone(), cc.data_buf.clone()))
}

// TODO: Handle Compilation Error
pub fn compile(
    cc: &mut CompilerContext,
    path: String,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let program = parse_file(path);
    for item in program.items {
        match item {
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Macro(i, m) => {
                cc.macros_map.insert(i, m);
            }
            ProgramItem::Func(f) => {
                cc.functions_map.insert(f.ident.clone(), f.clone());
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents)?;
            }
        }
    }
    let functions = cc.functions_map.clone();
    for f in functions.values() {
        function(cc, f);
    }
    assert!(
        cc.scoped_blocks.is_empty(),
        "Somting went wrong: Scope has not been cleared"
    );
    Ok((cc.instruct_buf.clone(), cc.data_buf.clone()))
}

/*
 *  keep in mind there could be a problem when a variable wants to access
 *  somthing that added after in code but it could be a feature too :)
 */
fn compile_block(cc: &mut CompilerContext, block: &Block) {
    cc.block_id += 1;
    cc.scoped_blocks.push(cc.block_id);
    for stmt in &block.stmts {
        compile_stmt(cc, stmt);
    }
    cc.block_id -= 1;
    cc.scoped_blocks.pop().unwrap();
}

fn compile_if_stmt(cc: &mut CompilerContext, ifs: &IFStmt, exit_tag: usize) {
    compile_expr(cc, &ifs.condition);
    let next_tag = match ifs.else_block.as_ref() {
        ElseBlock::None => exit_tag,
        _ => cc.instruct_buf.len(),
    };
    cc.instruct_buf.push(asm!("pop rax"));
    cc.instruct_buf.push(asm!("test rax, rax"));
    cc.instruct_buf.push(asm!("jz .L{}", next_tag));

    compile_block(cc, &ifs.then_block);
    match ifs.else_block.as_ref() {
        ElseBlock::None => {
            cc.instruct_buf.push(asm!(".L{}:", next_tag));
        }
        ElseBlock::Else(b) => {
            cc.instruct_buf.push(asm!("jmp .L{}", exit_tag));
            cc.instruct_buf.push(asm!(".L{}:", next_tag));
            compile_block(cc, b);
            cc.instruct_buf.push(asm!(".L{}:", exit_tag));
        }
        ElseBlock::Elif(iff) => {
            cc.instruct_buf.push(asm!("jmp .L{}", exit_tag));
            cc.instruct_buf.push(asm!(".L{}:", next_tag));
            compile_if_stmt(cc, iff, exit_tag);
        }
    }
}

fn compile_macro_call(cc: &mut CompilerContext, macro_call: &MacroCall) -> Result<(), String> {
    let Some(macro_) = cc.macros_map.get(&macro_call.ident) else {
            return Err(format!("Undifined macro_call {}",macro_call.ident));
        };
    if (macro_.args as usize) < macro_call.call_args.len() {
        return Err(format!(
            "Args Suplied to this macro is more than the definition allows"
        ));
    }
    for _ in macro_call.call_args.iter() {
        todo!();
    }
    Ok(())
}

fn compile_stmt(cc: &mut CompilerContext, stmt: &Stmt) {
    match &stmt.stype {
        StmtType::MacroCall(m) => match compile_macro_call(cc, m) {
            Ok(_) => (),
            Err(msg) => error(msg, stmt.loc.clone()),
        },
        StmtType::VariableDecl(v) => {
            insert_variable(cc, v);
        }
        StmtType::Print(e) => {
            compile_expr(cc, e);
            match e.etype {
                ExprType::String(_) => {
                    cc.instruct_buf.push(asm!("mov rax, 1"));
                    cc.instruct_buf.push(asm!("mov rdi, 1"));
                    cc.instruct_buf.push(asm!("pop rbx"));
                    cc.instruct_buf.push(asm!("pop rcx"));
                    cc.instruct_buf.push(asm!("mov rsi, rcx"));
                    cc.instruct_buf.push(asm!("mov rdx, rbx"));
                    cc.instruct_buf.push(asm!("syscall"));
                }
                _ => {
                    cc.instruct_buf.push(asm!("pop rdi"));
                    cc.instruct_buf.push(asm!("call print"));
                }
            }
        }
        StmtType::If(ifs) => {
            let exit_tag = cc.instruct_buf.len();
            compile_if_stmt(cc, ifs, exit_tag);
        }
        StmtType::Assign(a) => match compile_assgin(cc, a) {
            Ok(_) => (),
            Err(msg) => error(msg, stmt.loc.clone()),
        },
        StmtType::While(w) => {
            compile_while(cc, w);
        }
        StmtType::Expr(e) => match e.etype {
            ExprType::FunctionCall(_) => {
                compile_expr(cc, e);
            }
            _ => {
                println!("Warning: Expression with no effect ignored!");
            }
        },
        StmtType::Return(e) => {
            compile_expr(cc, e);
            cc.instruct_buf.push(asm!("pop rax"));
            cc.instruct_buf.push(asm!("leave"));
            cc.instruct_buf.push(asm!("ret"));
        }
        StmtType::InlineAsm(instructs) => {
            for instr in instructs {
                match compile_inline_asm(cc, instr) {
                    Ok(_) => (),
                    Err(msg) => error(msg, stmt.loc.clone()),
                }
            }
        }
        _ => {
            todo!();
        }
    }
}

fn compile_inline_asm(cc: &mut CompilerContext, instr: &String) -> Result<(), String> {
    if instr.contains('%') {
        let mut final_instr = instr.clone();
        let chars = final_instr.chars().collect::<Vec<char>>();
        let mut index = 0;
        let is_empty = |index: usize| (index >= chars.len());
        while !is_empty(index) {
            if chars[index] == '%' {
                let mut ident = String::new();
                let first_index = index;
                index += 1;
                while !is_empty(index) && (chars[index].is_alphanumeric() || chars[index] == '_') {
                    ident.push(chars[index]);
                    index += 1;
                }
                if !ident.is_empty() {
                    let Some(v_map) = find_variable(cc,ident.clone()) else {
                            return Err(format!(
                                "Could not find variable {} in this scope",
                                ident.clone()
                            ));
                        };
                    let mem_acss = format!("{} [rbp-{}]", mem_word(8), v_map.offset + v_map.size);
                    let mut temp = String::new();
                    temp.push_str(chars[0..(first_index)].iter().collect::<String>().as_str());
                    temp.push_str(mem_acss.as_str());
                    temp.push_str(chars[index..].iter().collect::<String>().as_str());
                    final_instr = temp;
                    index += mem_acss.len()
                } else {
                    return Err("Invalid Identifier for Inline Asm".to_string());
                }
            } else {
                index += 1;
            }
        }
        cc.instruct_buf.push(asm!("{}", final_instr));
    } else {
        cc.instruct_buf.push(asm!("{}", instr));
    }
    Ok(())
}

fn compile_while(cc: &mut CompilerContext, w_stmt: &WhileStmt) {
    let cond_tag = cc.instruct_buf.len();
    cc.instruct_buf.push(asm!("jmp .L{}", cond_tag));
    let block_tag = cond_tag + 1;
    cc.instruct_buf.push(asm!(".L{}:", block_tag));
    compile_block(cc, &w_stmt.block);
    cc.instruct_buf.push(asm!(".L{}:", cond_tag));
    // Jump after a compare
    compile_expr(cc, &w_stmt.condition);
    cc.instruct_buf.push(asm!("pop rax"));
    cc.instruct_buf.push(asm!("test rax, rax"));
    cc.instruct_buf.push(asm!("jnz .L{}", block_tag));
}

fn assgin_op(cc: &mut CompilerContext, op: &AssignOp, v_map: &VariableMap) {
    let mem_acss = if v_map.item_size != v_map.size {
        format!(
            "{} [rbp-{}+rbx*{}]",
            mem_word(v_map.item_size),
            v_map.offset + v_map.size,
            v_map.item_size
        )
    } else {
        format!(
            "{} [rbp-{}]",
            mem_word(v_map.item_size),
            v_map.offset + v_map.size
        )
    };
    let reg = rbs("a", v_map.item_size);
    cc.instruct_buf.push(asm!("pop rax"));
    match op {
        AssignOp::Eq => {
            cc.instruct_buf.push(asm!("mov {mem_acss},{reg}"));
        }
        AssignOp::PlusEq => {
            cc.instruct_buf.push(asm!("add {mem_acss},{reg}"));
        }
        AssignOp::SubEq => {
            cc.instruct_buf.push(asm!("sub {mem_acss},{reg}"));
        }
        AssignOp::MultiEq => {
            let b_reg = rbs("b", v_map.item_size);
            cc.instruct_buf.push(asm!("mov {b_reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("imul {reg},{b_reg}"));
            cc.instruct_buf.push(asm!("mov {mem_acss},{reg}"));
        }
        AssignOp::DevideEq => {
            let b_reg = rbs("b", v_map.item_size);
            cc.instruct_buf.push(asm!("mov {b_reg},{reg}"));
            cc.instruct_buf.push(asm!("mov {reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("cqo"));
            cc.instruct_buf.push(asm!("idiv rbx"));
            cc.instruct_buf.push(asm!("mov {mem_acss},{reg}"));
        }
        AssignOp::ModEq => {
            let b_reg = rbs("b", v_map.item_size);
            cc.instruct_buf.push(asm!("mov {b_reg},{reg}"));
            cc.instruct_buf.push(asm!("mov {reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("cqo"));
            cc.instruct_buf.push(asm!("idiv rbx"));
            let d_reg = rbs("d", v_map.item_size);
            cc.instruct_buf.push(asm!("mov {mem_acss},{d_reg}"));
        }
    }
}

fn compile_assgin(cc: &mut CompilerContext, assign: &Assign) -> Result<(), String> {
    match &assign.left.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc,v) else {
                    return Err("Trying to access an Undifined variable".to_string());
                };
            if !v_map.is_mut {
                return Err("Error: Variable is not mutable. Did you forgot to define it with '=' insted of ':=' ?".to_string());
            }
            compile_expr(cc, &assign.right);
            assgin_op(cc, &assign.op, &v_map);
            Ok(())
        }
        ExprType::ArrayIndex(ai) => {
            let Some(v_map) = get_vriable_map(cc,&ai.ident) else {
                    return Err("Trying to access an Undifined variable".to_string());
                };
            if !v_map.is_mut {
                return Err("Error: Variable is not mutable. Did you forgot to define it with '=' insted of ':=' ?".to_string());
            }
            compile_expr(cc, &assign.right);
            compile_expr(cc, &ai.indexer);
            cc.instruct_buf.push(asm!("pop rbx"));
            assgin_op(cc, &assign.op, &v_map);
            Ok(())
        }
        _ => Err("Error: Expected a Variable type expression found Value".to_string()),
    }
}

fn get_vriable_map(cc: &mut CompilerContext, var_ident: &str) -> Option<VariableMap> {
    find_variable(cc, var_ident.to_owned())
}

fn compile_expr(cc: &mut CompilerContext, expr: &Expr) {
    // left = compile expr
    // right = compile expr
    // +
    match &expr.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc,v) else {
                    error("Trying to access an Undifined variable",expr.loc.clone());
                };
            let mem_acss = format!(
                "{} [rbp-{}]",
                mem_word(v_map.item_size),
                v_map.offset + v_map.size
            );
            cc.instruct_buf
                .push(asm!("mov {},{mem_acss}", rbs("a", v_map.item_size)));
            cc.instruct_buf.push(asm!("push rax"));
        }
        ExprType::Char(x) => {
            cc.instruct_buf.push(asm!("push {x}"));
        }
        ExprType::Int(x) => {
            // push x
            cc.instruct_buf.push(asm!("push {x}"));
        }
        ExprType::Compare(c) => {
            // TODO: Convert exprs to 0 or 1 and push into stack
            compile_expr(cc, c.left.as_ref());
            compile_expr(cc, c.right.as_ref());
            cc.instruct_buf.push(asm!("mov rcx, 0"));
            cc.instruct_buf.push(asm!("mov rdx, 1"));
            cc.instruct_buf.push(asm!("pop rbx"));
            cc.instruct_buf.push(asm!("pop rax"));
            cc.instruct_buf.push(asm!("cmp rax, rbx"));
            match c.op {
                CompareOp::Eq => {
                    cc.instruct_buf.push(asm!("cmove rcx, rdx"));
                }
                CompareOp::NotEq => {
                    cc.instruct_buf.push(asm!("cmovne rcx, rdx"));
                }
                CompareOp::Bigger => {
                    cc.instruct_buf.push(asm!("cmovg rcx, rdx"));
                }
                CompareOp::Smaller => {
                    cc.instruct_buf.push(asm!("cmovl rcx, rdx"));
                }
                CompareOp::BiggerEq => {
                    cc.instruct_buf.push(asm!("cmovge rcx, rdx"));
                }
                CompareOp::SmallerEq => {
                    cc.instruct_buf.push(asm!("cmovle rcx, rdx"));
                }
            }
            cc.instruct_buf.push(asm!("push rcx"));
        }
        ExprType::Binary(b) => {
            compile_expr(cc, b.left.as_ref());
            compile_expr(cc, b.right.as_ref());
            cc.instruct_buf.push(asm!("pop rbx"));
            cc.instruct_buf.push(asm!("pop rax"));
            match b.op {
                Op::Plus => {
                    cc.instruct_buf.push(asm!("add rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Sub => {
                    cc.instruct_buf.push(asm!("sub rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Multi => {
                    cc.instruct_buf.push(asm!("imul rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Devide => {
                    cc.instruct_buf.push(asm!("cqo"));
                    cc.instruct_buf.push(asm!("idiv rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Mod => {
                    cc.instruct_buf.push(asm!("cqo"));
                    cc.instruct_buf.push(asm!("idiv rbx"));
                    cc.instruct_buf.push(asm!("push rdx"));
                }
                Op::Or => {
                    cc.instruct_buf.push(asm!("or rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::And => {
                    cc.instruct_buf.push(asm!("and rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Lsh => {
                    cc.instruct_buf.push(asm!("mov rcx, rbx"));
                    cc.instruct_buf.push(asm!("sal rax, cl"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Rsh => {
                    cc.instruct_buf.push(asm!("mov rcx, rbx"));
                    cc.instruct_buf.push(asm!("sar rax, cl"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Not => {
                    panic!("Unvalid binary operation");
                }
            }
        }
        ExprType::String(str) => {
            let id = cc.data_buf.len();
            let data_array = asmfy_string(str);
            cc.data_buf.push(asm!("data{id} db {}", data_array));
            cc.data_buf.push(asm!("len{id} equ $ - data{id}"));
            cc.instruct_buf.push(asm!("push data{id}"));
            cc.instruct_buf.push(asm!("push len{id}"));
        }
        ExprType::Unary(u) => {
            compile_unary(cc, u);
            cc.instruct_buf.push(asm!("pop rax"));
            match u.op {
                Op::Sub => {
                    cc.instruct_buf.push(asm!("neg rax"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Plus => {
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Not => {
                    cc.instruct_buf.push(asm!("not rax"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                _ => {
                    unreachable!();
                }
            }
        }
        ExprType::FunctionCall(fc) => match compile_function_call(cc, fc) {
            Ok(_) => (),
            Err(msg) => error(msg, expr.loc.clone()),
        },
        ExprType::Ptr(e) => {
            compile_ptr(cc, e);
        }
        ExprType::ArrayIndex(ai) => {
            let v_map = find_variable(cc, ai.ident.clone()).unwrap_or_else(|| {
                error(
                    format!(
                        "Error: Trying to access an Undifined variable ({})",
                        ai.ident
                    ),
                    expr.loc.clone(),
                );
            });
            compile_expr(cc, &ai.indexer);
            cc.instruct_buf.push(asm!("pop rbx"));
            // TODO: Add Item size to v_map
            let mem_acss = format!(
                "{} [rbp-{}+rbx*{}]",
                mem_word(v_map.item_size),
                v_map.offset + v_map.size,
                v_map.item_size
            );
            let reg = rbs("a", v_map.item_size);
            cc.instruct_buf.push(asm!("mov {reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("push {reg}"));
        }
    }
}

fn compile_unary(cc: &mut CompilerContext, unary: &UnaryExpr) {
    compile_expr(cc, &unary.right);
}

fn compile_ptr(cc: &mut CompilerContext, expr: &Expr) {
    match &expr.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc,v) else {
                    error("Trying to access an Undifined variable",expr.loc.clone());
                };
            cc.instruct_buf.push(asm!("mov rax, rbp"));
            cc.instruct_buf
                .push(asm!("sub rax, {}", v_map.offset + v_map.size));
            cc.instruct_buf.push(asm!("push rax"));
        }
        _ => {
            todo!("Impl Pointers");
        }
    }
}

fn compile_function_call(cc: &mut CompilerContext, fc: &FunctionCall) -> Result<(), String> {
    for (index, arg) in fc.args.iter().enumerate() {
        compile_expr(cc, arg);
        match arg.etype {
            ExprType::String(_) => {
                cc.instruct_buf.push(asm!("pop rax"));
                cc.instruct_buf
                    .push(asm!("pop {}", function_args_register(index, 8)));
            }
            _ => {
                cc.instruct_buf
                    .push(asm!("pop {}", function_args_register(index, 8)));
            }
        }
    }
    // TODO: Setup a unresolved function table
    let Some(fun) = cc.functions_map.get(&fc.ident) else {
            return Err(
            format!(
                "Error: Function {} is not avaliable in this scope. {}",
                &fc.ident,
                "Make sure you are calling the correct function"
            ))
        };
    cc.instruct_buf.push(asm!("mov rax, 0"));
    cc.instruct_buf.push(asm!("call {}", fc.ident));
    if fun.ret_type.is_some() {
        cc.instruct_buf.push(asm!("push rax"));
    }
    Ok(())
}

fn asmfy_string(str: &str) -> String {
    let mut res = String::new();
    let source: Vec<char> = str.chars().collect();
    let mut i = 0;
    while i < source.len() {
        match source[i] {
            '\n' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 10");
                i += 1;
            }
            '\t' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 9");
                i += 1;
            }
            '\r' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 13");
                i += 1;
            }
            '\"' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 34");
                i += 1;
            }
            _ => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push('\"');
                while i < source.len() {
                    if source[i] == '\n'
                        || source[i] == '\"'
                        || source[i] == '\t'
                        || source[i] == '\r'
                    {
                        break;
                    }
                    res.push(source[i]);
                    i += 1;
                }
                res.push('\"');
            }
        }
    }
    res
}
