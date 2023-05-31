extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{fs::{self, File}, io::Write};

use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct NemetParser;

#[derive(Debug,PartialEq)]
enum Stmt {
    NOTIMPLEMENTED,
}
impl Stmt {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        Self::NOTIMPLEMENTED
    }
}

#[derive(Debug,PartialEq)]
enum MainBlock {
    Image {
        path: String,
        name: String,
        format: String,
        size: (usize,usize),
        stmts: Vec<Stmt>,
    },
}
impl MainBlock {
    fn parse_image(pairs: &mut Pairs<Rule>) -> Self {
        let mut path: String = "./".to_string();
        let mut name: String = "output".to_string();
        let mut format: String = "png".to_string();
        let mut size: (usize,usize) = (800,600);
        let mut stmts = Vec::<Stmt>::new();
        for pair in pairs {
            if pair.as_rule() == Rule::block_start_tag || 
                pair.as_rule() == Rule::block_end_tag {
                    continue;
            }
            if pair.as_rule() == Rule::option {
                let mut option_pair = pair.into_inner();
                let ident = option_pair.next().unwrap();
                match ident.as_str() {
                    "path" => {
                        path = option_pair.next().unwrap().as_str().to_string().replace("\"", "");
                    },
                    "name" => {
                        name = option_pair.next().unwrap().as_str().to_string().replace("\"", "");
                    },
                    "format" => {
                        format = option_pair.next().unwrap().as_str().to_string().replace("\"", "");
                    },
                    "size" => {
                        let mut tuple = option_pair.next().unwrap().into_inner();
                        size.0 = tuple.next().unwrap().as_str().parse::<usize>().unwrap();
                        size.1 = tuple.next().unwrap().as_str().parse::<usize>().unwrap();
                    }
                    _ => {
                        todo!();
                    }

                }
            }else {
                stmts.push(Stmt::parse(&mut pair.into_inner()));
            }

        }
        Self::Image { path, name, format, size, stmts }
    }
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let start = pairs.peek().unwrap();
        if start.as_rule() == Rule::block_start_tag {
            let kind = start.into_inner().peek().unwrap().as_str();
            if kind == "image" {
               return Self::parse_image(pairs); 
           }
        }else{
            unreachable!();
        }
        todo!();
    }
}

fn generate_ppm(block: &MainBlock) {
    let MainBlock::Image{ path, name, format, size, stmts } = block;
    let header = format!("P6\n{} {}\n255\n",size.0,size.1);
    let file_path = format!("{path}{name}.{format}");
    println!("Creating Media...");
    let mut output = File::create(file_path).unwrap();
    output.write(header.as_bytes()).unwrap();
    for _ in 0..size.0 * size.1 {
        output.write(vec![0,255,0].as_slice()).unwrap();
    }
    output.flush().unwrap();
}

fn generate(block: &MainBlock) {
    generate_ppm(block);
}

fn main() {
    let buf = fs::read("./test.nmt").unwrap();
    let source = String::from_utf8(buf).unwrap();
    let pairs = NemetParser::parse(Rule::program_file, source.as_str())
        .unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        if pair.as_rule() == Rule::EOI { break; }
        let main_block = MainBlock::parse(&mut pair.into_inner());
        generate(&main_block);
        println!("{:?}",main_block);
    }
}

