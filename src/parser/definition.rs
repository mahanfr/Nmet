#[derive(Debug,PartialEq)]
pub struct Type {
    pub name: String,
}
#[derive(Debug)]
pub struct Arg {
    pub ident: String,
    pub kind: Type,
}

