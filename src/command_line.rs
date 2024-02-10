/**********************************************************************************************
*
*   commmand_line: Structures and funtions for handeling command line arguments and control
*   complation process
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
use std::process::exit;

use crate::{COPYRIGHT, DEBUG, VERSION};

pub fn help_command() {
    println!("Nemet {VERSION}");
    println!("Nemet Programmin Language Copyright: {COPYRIGHT}");
    println!("Project distributed Under MIT License");
    if DEBUG {
        println!("--- DEBUG MODE ---");
    }
    println!("\nnemet [Command] <path> (Options)");
    println!("Commands:");
    println!("\t{} Show help", padding_right("help"));
    println!("Options:");
    println!("\t{} Show help", padding_right("--help"));
    println!("\t{} Show Version", padding_right("--version"));
}

pub fn padding_right(str: &str) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    for _ in 0..(20 - str.len()) {
        text.push(' ');
    }
    text
}

pub struct CliArgs {
    pub args: Vec<String>,
    pub index: usize,
}

impl CliArgs {
    pub fn new(args: Vec<String>) -> Self {
        Self { args, index: 1 }
    }

    pub fn get(&self) -> String {
        self.args[self.index].clone()
    }

    pub fn next(&mut self) {
        if self.index < self.args.len() {
            self.index += 1;
        } else {
            help_command();
            exit(-1);
        }
    }
}
