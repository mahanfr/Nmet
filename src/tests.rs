#[cfg(test)]
mod parser_tests {
    use crate::parser::variable::VariableDelclear;
    use crate::parser::definition::Type;
    use crate::lexer::Lexer;

    #[test]
    fn dynamic_variable_declearation() {
        let mut lexer = Lexer::from_str("hello u32;\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: false,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "".to_string(),
        });
        let mut lexer = Lexer::from_str("hello = \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: false,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "undifiend".to_string(),
            },
            init_value: "facts".to_string(),
        });
        let mut lexer = Lexer::from_str("hello u32 = \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: false,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "facts".to_string(),
        });
    }

    #[test]
    fn const_variable_declearation() {
        let mut lexer = Lexer::from_str("hello : \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: true,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "undifiend".to_string(),
            },
            init_value: "facts".to_string(),
        });
        let mut lexer = Lexer::from_str("hello u32 : \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: true,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "facts".to_string(),
        });
    }

    #[test]
    fn static_variable_declearation() {
        let mut lexer = Lexer::from_str("hello u32 :: \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: true,
            is_static: true,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "facts".to_string(),
        });
    }
}
