
/*
 * // Code For Image generation
 * @[image]
 * <path = ./> // Default
 * <format = png> // Default
 * <size = 800*600> // Default
 * @.fill = 255 255 255 0;
 * @[0,0] = 255 0 0 255;
 * @[10..20,10..50] = 0 255 0 255;
 * @.fin
 * */
mod lexer;
use crate::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("test.nmt");
    loop {
        if let Some(token) = lexer.next_token() {
            println!("{:?}",token);
        } else { break; }
    }
}

