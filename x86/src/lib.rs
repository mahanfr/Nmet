// Copywrite 2023-2024 MIT license
// Extention of Nmet Compiler
// proc-macro for generating Instrs Enum from file "./instr.txt"

use proc_macro::{Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use quote::quote;
use std::{collections::HashSet, fs};

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
            _ => Self::Ext(str.parse::<u8>().unwrap()),
        }
    }

    pub fn to_ts(&self) -> TokenStream {
        match self {
            Self::None => quote!(ModrmType::None).into(),
            Self::Add => quote!(ModrmType::Add).into(),
            Self::Modrm => quote!(ModrmType::Modrm).into(),
            Self::Ext(ex) => quote!(ModrmType::Ext(#ex)).into(),
        }
    }
}

#[allow(dead_code)]
enum __AcceptedOprator {
    R64,
    R32,
    R16,
    R8,
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
            _ => unreachable!("{op}"),
        }
    }

    pub fn to_ts(&self) -> TokenStream {
        let gen = match self {
            Self::R64 => quote!(Opr::R64(_)),
            Self::R32 => quote!(Opr::R32(_)),
            Self::R16 => quote!(Opr::R16(_)),
            Self::R8 => quote!(Opr::R8(_)),
            Self::Mem => quote!(Opr::Mem(_)),
            Self::Imm8 => quote!(Opr::Imm8(_)),
            Self::Imm32 => quote!(Opr::Imm32(_)),
            Self::Imm64 => quote!(Opr::Imm64(_)),
        };
        gen.into()
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

#[proc_macro_attribute]
pub fn import_instructions(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = args.into_iter();
    let content;
    if let Some(TokenTree::Literal(l)) = args.next() {
        content = fs::read_to_string(l.to_string().replace('"', "")).expect("File not found");
    } else {
        panic!("expected a path in macro attributes!");
    }
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let (instrs_list, enum_tokens) = __internal_extract_enum(&content);
    // Enum Generation
    let mut ts : TokenStream = quote!(#[derive(Debug, PartialEq, Clone, Copy)]).into();
    ts.extend(__internal_define_insns_enum(&ast, enum_tokens));
    // Impl of Enum
    let internal_functions = generate_instr_functions(&instrs_list);
    ts.extend(impl_instructions(&ast, internal_functions));
    // Impl of Display for the Enum
    let fmt_internals = impl_enum_fmt_internals(&instrs_list);
    ts.extend(impl_display(&ast, fmt_internals));
    ts
}

fn generate_instr_functions(data: &[InstrData]) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(generate_instr_opcode_function(data));
    ts.extend(generate_instr_helper_functions(data));
    ts
}

fn generate_instr_helper_functions(data: &[InstrData]) -> TokenStream {
    let mut ts = TokenStream::new();
    let mut set = HashSet::<String>::new();
    for item in data.iter() {
        if set.get(&item.name).is_none() {
            set.insert(item.name.clone());
        } else {
            continue;
        }
        let mut fn_args = TokenStream::new();
        let mut fn_iternal = TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("Self", Span::call_site())),
            TokenTree::Punct(Punct::new(':',proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new(':',proc_macro::Spacing::Alone)),
            TokenTree::Ident(Ident::new(&item.name, Span::call_site())),
        ]);
        match item.airity {
            0 => (),
            1 => {
                let q1: TokenStream = quote!(op1: impl Into<Opr>).into();
                let q2: TokenStream = quote!(op1.into()).into();
                fn_args.extend(q1);
                fn_iternal.extend(vec![
                   TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, q2)),
                ]);
            },
            2 => {
                let q1: TokenStream = quote!(op1: impl Into<Opr>, op2: impl Into<Opr>).into();
                let q2: TokenStream = quote!(op1.into(), op2.into()).into();
                fn_args.extend(q1);
                fn_iternal.extend(vec![
                   TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, q2)),
                ]);
            },
            _ => unreachable!(),
        }
        let fn_name = item.name.to_lowercase();
        ts.extend(vec![
            TokenTree::Ident(Ident::new("pub",Span::call_site())),
            TokenTree::Ident(Ident::new("fn",Span::call_site())),
            TokenTree::Ident(Ident::new(&fn_name, Span::call_site())),
            TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, fn_args)),
            TokenTree::Punct(Punct::new('-', proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new('>', proc_macro::Spacing::Alone)),
            TokenTree::Ident(Ident::new("Self",Span::call_site())),
            TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, fn_iternal)),
        ]);
    }
    ts
}

fn generate_instr_opcode_function(data: &[InstrData]) -> TokenStream {
    let mut ts = TokenStream::new();
    let mut items = TokenStream::new();
    for item in data.iter() {
        let mut output = TokenStream::new();
        output.extend(vec![
            TokenTree::Literal(Literal::u16_suffixed(item.opcode)),
            TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone)),
        ]);
        output.extend(item.modrm.to_ts());

        let cond = generate_condition(item);

        let case = vec![
            TokenTree::Ident(Ident::new("Self", Span::call_site())),
            TokenTree::Punct(Punct::new(':', proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', proc_macro::Spacing::Alone)),
            TokenTree::Ident(Ident::new(item.name.as_str(), Span::call_site())),
            TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, cond)),
            TokenTree::Punct(Punct::new('=', proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new('>', proc_macro::Spacing::Alone)),
            TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, output)),
            TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone)),
        ];
        items.extend(case);
    }
    items.extend(vec![
        TokenTree::Ident(Ident::new("_", Span::call_site())),
        TokenTree::Punct(Punct::new('=', proc_macro::Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', proc_macro::Spacing::Alone)),
        TokenTree::Ident(Ident::new("unreachable", Span::call_site())),
        TokenTree::Punct(Punct::new('!', proc_macro::Spacing::Alone)),
        TokenTree::Group(Group::new(
            proc_macro::Delimiter::Parenthesis,
            TokenStream::new(),
        )),
    ]);
    let mut matches = TokenStream::new();
    matches.extend(vec![
        TokenTree::Ident(Ident::new("match", Span::call_site())),
        TokenTree::Ident(Ident::new("self", Span::call_site())),
        TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, items)),
    ]);
    let mut func = TokenStream::new();
    func.extend(vec![
        TokenTree::Ident(Ident::new("pub",Span::call_site())),
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new("opcode", Span::call_site())),
        TokenTree::Group(Group::new(
            proc_macro::Delimiter::Parenthesis,
            TokenStream::from_iter(vec![
                TokenTree::Punct(Punct::new('&', proc_macro::Spacing::Alone)),
                TokenTree::Ident(Ident::new("self", Span::call_site())),
            ]),
        )),
        TokenTree::Punct(Punct::new('-', proc_macro::Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', proc_macro::Spacing::Alone)),
        TokenTree::Group(Group::new(
            proc_macro::Delimiter::Parenthesis,
            TokenStream::from_iter(vec![
                TokenTree::Ident(Ident::new("u16", Span::call_site())),
                TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone)),
                TokenTree::Ident(Ident::new("ModrmType", Span::call_site())),
            ]),
        )),
        TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, matches)),
    ]);
    ts.extend(func);
    ts
}

fn generate_condition(item: &InstrData) -> TokenStream {
    let mut tk = TokenStream::new();
    if item.airity == 0 {
        return tk;
    }
    for (index, val) in item.op1_accepted.iter().enumerate() {
        if index != 0 {
            tk.extend(vec![TokenTree::Punct(Punct::new(
                '|',
                proc_macro::Spacing::Alone,
            ))]);
        }
        tk.extend(val.to_ts());
    }
    tk.extend(vec![TokenTree::Punct(Punct::new(
        ',',
        proc_macro::Spacing::Alone,
    ))]);
    for (index, val) in item.op2_accepted.iter().enumerate() {
        if index != 0 {
            tk.extend(vec![TokenTree::Punct(Punct::new(
                '|',
                proc_macro::Spacing::Alone,
            ))]);
        }
        tk.extend(val.to_ts());
    }
    tk
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

fn impl_instructions(ast: &syn::DeriveInput, internal_functions: TokenStream) -> TokenStream {
    let name = &ast.ident;
    let quote: TokenStream = quote!(impl #name).into();
    let mut ts = TokenStream::new();
    ts.extend(quote);
    let block = Group::new(proc_macro::Delimiter::Brace, internal_functions);
    ts.extend(vec![TokenTree::Group(block)]);
    ts
}

fn impl_enum_fmt_internals(instr_list: &[InstrData]) -> TokenStream {
    let mut block = TokenStream::new();
    let mut enum_set: HashSet<String> = HashSet::new();
    for item in instr_list.iter() {
        if enum_set.get(&item.name).is_none() {
            enum_set.insert(item.name.clone());
        } else {
            continue;
        }

        let enum_ident = match item.airity {
            // Syscall
            0 => TokenStream::from_iter(vec![TokenTree::Ident(Ident::new(
                &item.name,
                Span::call_site(),
            ))]),
            // Push(opr1)
            1 => TokenStream::from_iter(vec![
                TokenTree::Ident(Ident::new(&item.name, Span::call_site())),
                TokenTree::Group(Group::new(
                    proc_macro::Delimiter::Parenthesis,
                    TokenStream::from_iter(vec![TokenTree::Ident(Ident::new(
                        "opr1",
                        Span::call_site(),
                    ))]),
                )),
            ]),
            // Mov(opr1, opr2)
            2 => TokenStream::from_iter(vec![
                TokenTree::Ident(Ident::new(&item.name, Span::call_site())),
                TokenTree::Group(Group::new(
                    proc_macro::Delimiter::Parenthesis,
                    TokenStream::from_iter(vec![
                        TokenTree::Ident(Ident::new("opr1", Span::call_site())),
                        TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone)),
                        TokenTree::Ident(Ident::new("opr2", Span::call_site())),
                    ]),
                )),
            ]),
            _ => unreachable!(),
        };

        let formater = match item.airity {
            0 => item.name.to_lowercase().to_string(),
            1 => format!("{} {{opr1}}", item.name.to_lowercase()),
            2 => format!("{} {{opr1}}, {{opr2}}", item.name.to_lowercase()),
            _ => unreachable!(),
        };

        let mut item_ts = TokenStream::from_iter(vec![
            // Self::Mov
            TokenTree::Ident(Ident::new("Self", Span::call_site())),
            TokenTree::Punct(Punct::new(':', proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', proc_macro::Spacing::Alone)),
        ]);
        item_ts.extend(enum_ident);
        item_ts.extend(vec![
            // =>
            TokenTree::Punct(Punct::new('=', proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new('>', proc_macro::Spacing::Alone)),
            // write!
            TokenTree::Ident(Ident::new("write", Span::call_site())),
            TokenTree::Punct(Punct::new('!', proc_macro::Spacing::Alone)),
            TokenTree::Group(Group::new(
                proc_macro::Delimiter::Parenthesis,
                TokenStream::from_iter(vec![
                    TokenTree::Ident(Ident::new("f", Span::call_site())),
                    TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone)),
                    TokenTree::Literal(Literal::string(&formater)),
                ]),
            )),
            TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone)),
        ]);
        block.extend(item_ts);
    }
    
    TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("match", Span::call_site())),
        TokenTree::Ident(Ident::new("self", Span::call_site())),
        TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, block)),
    ])
}

fn impl_display(ast: &syn::DeriveInput, fmt_internals: TokenStream) -> TokenStream {
    let name = &ast.ident;
    let mut tk: TokenStream = quote!(impl Display for #name).into();
    let mut disp_fn: TokenStream =
        quote!(fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result).into();
    disp_fn.extend(vec![TokenTree::Group(Group::new(
        proc_macro::Delimiter::Brace,
        fmt_internals,
    ))]);
    tk.extend(vec![TokenTree::Group(Group::new(
        proc_macro::Delimiter::Brace,
        disp_fn,
    ))]);
    tk
}

fn __internal_extract_opcode(num_str: &str) -> u16 {
    u16::from_str_radix(num_str, 16).unwrap()
}

fn __internal_single_op(data: &str) -> Vec<__AcceptedOprator> {
    let mut list = Vec::new();
    for opt in data.trim().split('/') {
        list.push(__AcceptedOprator::from_str(opt));
    }
    list
}

fn __internal_extract_op_info(data: &str) -> (Vec<__AcceptedOprator>, Vec<__AcceptedOprator>) {
    let new_data = data.replace(['(', ')'], "");
    let mut split_data = new_data.split(',');
    let Some(op1) = split_data.next() else {
        return (vec![], vec![]);
    };
    let Some(op2) = split_data.next() else {
        return (__internal_single_op(op1), vec![]);
    };
    (__internal_single_op(op1), __internal_single_op(op2))
}

fn __internal_set_enum_value_type(airity: u8) -> TokenStream {
    if airity == 0 {
        let gen = quote!(,);
        gen.into()
    } else if airity == 1 {
        let gen = quote!((Opr),);
        gen.into()
    } else if airity == 2 {
        let gen = quote!((Opr, Opr),);
        gen.into()
    } else {
        panic!("wrong value for instr airity");
    }
}

fn __internal_extract_enum(content: &str) -> (Vec<InstrData>, TokenStream) {
    let mut gen = TokenStream::new();
    let mut enum_set = HashSet::<String>::new();
    let mut instr_data = Vec::<InstrData>::new();
    for line in content.split('\n') {
        if line.len() < 2 {
            break;
        }
        let mut tokens = line.split('|');
        let name = tokens.next().unwrap().trim().to_string();
        let opcode = __internal_extract_opcode(tokens.next().unwrap().trim());
        let modrm = __ModrmType::from_str(tokens.next().unwrap().trim());
        let airity = u8::from_str_radix(tokens.next().unwrap().trim(), 16).unwrap();
        let data = __internal_extract_op_info(tokens.next().unwrap().trim());
        if enum_set.get(&name).is_none() {
            gen.extend(vec![
                TokenTree::Ident(Ident::new(name.as_str(), Span::call_site())),
                //TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone))
            ]);
            gen.extend(__internal_set_enum_value_type(airity).into_iter());
            enum_set.insert(name.clone());
        }
        instr_data.push(InstrData {
            name,
            opcode,
            airity,
            op1_accepted: data.0,
            op2_accepted: data.1,
            modrm,
        })
    }
    (instr_data, gen)
}
