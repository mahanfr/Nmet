mod binary_expr;

pub trait AsmParsable {
    fn parse_to_asm(&self) -> String;
}

