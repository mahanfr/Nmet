use std::{fs, collections::HashSet};
use quote::quote;
use proc_macro::{TokenStream, TokenTree, Ident, Span, Punct, Group};

#[proc_macro_attribute]
pub fn import_instructions(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = args.into_iter();
    let content;
    if let Some(TokenTree::Literal(l)) = args.next() {
        content = fs::read_to_string(l.to_string().replace('"',""))
            .expect("File not found");
    } else {
        panic!("expected a path in macro attributes!");
    }
    let ast : syn::DeriveInput = syn::parse(input).unwrap();
    let enums = __internal_extract_enum(&content);
    __internal_define_insns_enum(&ast, enums)
}

#[allow(dead_code)]
enum __ModrmType {
    Ext(u8),
    Add,
    Modrm,
    None,
}
impl __ModrmType {
    pub fn from_str(str: &str) -> Self {
        match str {
            "r" => Self::Modrm,
            "n" => Self::None,
            "a" => Self::Add,
            _ => {
               Self::Ext(u8::from_str_radix(str, 10).unwrap())
            }
        }
    }
}

#[allow(dead_code)]
enum __AcceptedOprator {
    R64,
    R32,
    R16,
    R8,
    Special(String),
    Mem,
    Imm8,
    Imm32,
    Imm64,
}
impl __AcceptedOprator {
    pub fn from_str(op: &str) -> Self {
        match op {
            "r64" => Self::R64,
            "r32" => Self::R32,
            "r16" => Self::R16,
            "r8" => Self::R8,
            "imm8" => Self::Imm8,
            "imm32" => Self::Imm32,
            "imm64" => Self::Imm64,
            "m" => Self::Mem,
            _ => Self::Special(op.to_string()),
        }
    }
}

#[allow(dead_code)]
struct InstrData {
    name: String,
    opcode: u16,
    airity: u8,
    op1_accepted: Vec<__AcceptedOprator>,
    op2_accepted: Vec<__AcceptedOprator>,
    modrm: __ModrmType,
}

fn __internal_extract_opcode(num_str: &str) -> u16 {
    u16::from_str_radix(&num_str, 16).unwrap()
}

fn __internal_single_op(data: &str) -> Vec<__AcceptedOprator> {
    let mut list = Vec::new();
    for opt in data.trim().split('/').into_iter() {
        list.push(__AcceptedOprator::from_str(opt));
    }
    list
}

fn __internal_extract_op_info(data: &str) -> (Vec<__AcceptedOprator>, Vec<__AcceptedOprator>) {
    let new_data = data.replace('(',"").replace(')',"");
    let mut split_data = new_data.split(',');
    let Some(op1) = split_data.next() else {
        return (vec![],vec![]);
    };
    let Some(op2) = split_data.next() else {
        return (__internal_single_op(op1), vec![]);
    };
    return (__internal_single_op(op1), __internal_single_op(op2));
}

fn __internal_extract_enum(content: &str) -> TokenStream {
    let mut gen = TokenStream::new();
    let mut enum_set = HashSet::<String>::new();
    let mut instr_data = Vec::<InstrData>::new();
    for line in content.split('\n') {
        if line.len() < 2 {
            break;
        }
        let mut tokens = line.split('|');
        let name = tokens.next().unwrap().trim().to_string();
        let opcode = __internal_extract_opcode(
            &tokens.next().unwrap().trim().to_string()
        );
        let modrm = __ModrmType::from_str(
            &tokens.next().unwrap().trim().to_string()
        );
        let airity = u8::from_str_radix(&tokens.next().unwrap().trim().to_string(), 16).unwrap();
        let data = __internal_extract_op_info(&tokens.next().unwrap().trim().to_string());
        if enum_set.get(&name).is_none() {
            gen.extend(
                vec![
                    TokenTree::Ident(Ident::new(name.as_str(), Span::call_site())),
                    TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone))
                ]
            );
            enum_set.insert(name.clone());
        }
        instr_data.push(InstrData { name, opcode, airity, op1_accepted: data.0, op2_accepted: data.1, modrm})
    }
    gen
}

fn __internal_define_insns_enum(ast: &syn::DeriveInput, fileds: TokenStream) -> TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let mut token_vec = TokenStream::new();
    
    let enum_def = quote!(
        #vis enum #name 
    );
    let enum_def_tk_stream: TokenStream = enum_def.into();
    let block = Group::new(proc_macro::Delimiter::Brace, fileds);
    token_vec.extend(enum_def_tk_stream);
    token_vec.extend(vec![TokenTree::Group(block)]);
    token_vec
}
