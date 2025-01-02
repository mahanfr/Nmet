/**********************************************************************************************
*
*   utils: Utility funtions that are used eveywhere
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use std::path::PathBuf;

pub type IBytes = Vec<u8>;

/// Parse Program name from path
#[allow(unused)]
pub fn get_program_name(path: impl ToString) -> String {
    let path = path.to_string();
    return path
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string();
}

/// padding right (like the padding_left library in js)
pub fn padding_right(str: &str, mut len: usize) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    if len < str.len() {
        len = 20;
    }
    for _ in 0..(len - str.len()) {
        text.push(' ');
    }
    text
}

/// Generate default output path using input string
pub fn get_output_path_from_input(input: PathBuf) -> PathBuf {
    std::path::PathBuf::from("./build/out").with_file_name(input.file_name().unwrap())
}
