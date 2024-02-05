use std::fmt::Display;

use crate::parser::types::VariableType;

fn data_type(dt: &VariableType) -> &'static str {
    match dt {
        VariableType::String | VariableType::Char => "db",
        VariableType::Int | VariableType::UInt => "dd",
        VariableType::Long | VariableType::ULong | VariableType::Pointer => "dq",
        VariableType::Array(t, _) => data_type(t.as_ref()),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct BssItem {
    pub index: usize,
    pub name: String,
    pub size: usize,
}

impl BssItem {
    pub fn new(name: String, index: usize, size: usize) -> Self {
        Self { index, name, size }
    }
}

impl Display for BssItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} resb {}", self.name, self.size)
    }
}

#[derive(Debug, Clone)]
pub struct DataItem {
    pub index: usize,
    pub name: String,
    pub data: Vec<u8>,
    pub dtype: VariableType,
}
impl DataItem {
    pub fn new(name: String, index: usize, data: Vec<u8>, dtype: VariableType) -> Self {
        Self {
            index,
            name,
            data,
            dtype,
        }
    }

    pub fn asmblized_data(&self) -> String {
        let mut asm_str = String::new();
        let mut ascii_stack = Vec::<u8>::new();
        for ch in self.data.iter() {
            if ch.is_ascii_alphanumeric() {
                ascii_stack.push(*ch);
            } else {
                if !ascii_stack.is_empty() {
                    let str = String::from_utf8(ascii_stack.clone()).unwrap();
                    if !asm_str.is_empty() {
                        asm_str.push(',');
                    }
                    asm_str.extend(format!("\"{str}\"").chars());
                    ascii_stack.clear();
                }
                if !asm_str.is_empty() {
                    asm_str.push(',');
                }
                asm_str.push_str(&ch.to_string());
            }
        }
        if !ascii_stack.is_empty() {
            let str = String::from_utf8(ascii_stack.clone()).unwrap();
            if !asm_str.is_empty() {
                asm_str.push(',');
            }
            asm_str.extend(format!("\"{str}\"").chars());
        }
        println!("({asm_str})");
        asm_str
    }
}

impl Display for DataItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name,
            data_type(&self.dtype),
            self.asmblized_data()
        )
    }
}
