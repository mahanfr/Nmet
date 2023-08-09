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
