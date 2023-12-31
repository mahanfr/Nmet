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

fn __internal_extract_enum(content: &str) -> TokenStream {
    let mut gen = TokenStream::new();
    let mut enum_set = HashSet::<String>::new();
    for line in content.split('\n') {
        if line.len() < 2 {
            break;
        }
        let mut tokens = line.split('|');
        let name = tokens.next().unwrap().to_string();
        if enum_set.get(&name).is_none() {
            gen.extend(
                vec![
                    TokenTree::Ident(Ident::new(name.as_str(), Span::call_site())),
                    TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone))
                ]
            );
            enum_set.insert(name);
        }
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
