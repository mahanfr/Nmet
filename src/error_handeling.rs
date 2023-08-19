use std::{fmt::Display, process::exit};

#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    pub file_path: String,
    pub line: usize,
    pub col: usize,
}

impl Loc {
    pub fn new(file_path: String, line: usize, col: usize) -> Self {
        Self {
            file_path,
            line,
            col,
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file_path, self.line, self.col)
    }
}

pub fn error(msg: impl ToString, loc: Loc) -> ! {
    eprintln!("ERROR: {} {loc}", msg.to_string());
    exit(-1);
}
