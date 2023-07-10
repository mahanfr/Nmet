use std::{fs::{File, self}, io::{BufWriter, Write, Read}, cmp::{max, min}, env::args, process::exit};

type Color = (u8,u8,u8,u8);
static NEMET_BLACK: Color = (0,0,0,0xff);

struct Image {
    path: String,
    size: (usize,usize),
    render_color: Color,
    data: Vec<Color>
}

impl Image {
    pub fn new() -> Self {
        let size = (800,600);
        let data = vec![(0xee,0xee,0xee,0);size.0*size.1];
        Self {
            path: "./test".to_string(),
            size,
            render_color: NEMET_BLACK,
            data
        }
    }

    pub fn get_loc(&self,x:usize,y:usize) -> usize {
        (y * (self.size.0)) + x - 1
    }

    pub fn get_pos(&self,loc: usize) -> (usize,usize) {
        let y = loc / self.size.0;
        let x = loc % self.size.0 + 1;
        (x,y)
    }

    pub fn set_render_color(&mut self, color: Color) {
        self.render_color = color;
    }

    pub fn set_pixel(&mut self,x:usize, y:usize, color: Color) {
        let index = self.get_loc(x,y);
        self.data[index] = color;
    }

    pub fn fill(&mut self,color: Color){
        self.data.fill(color);
    }

    pub fn generate_ppm_file(&self) {
        let header = format!("P6\n{} {}\n255\n",self.size.0,self.size.1);
        let stream = File::create(format!("{}.{}",self.path,"ppm")).unwrap();
        let mut wbuffer = BufWriter::new(stream);
        wbuffer.write(header.as_bytes()).unwrap();
        for pixel in self.data.iter() {
            wbuffer.write(vec![pixel.0,pixel.1,pixel.2].as_slice()).unwrap();
        }
    }
}

fn draw_circle(image: &mut Image,x: usize,y:usize,r:usize) {
    for y2 in y..y+r*2 {
        for x2 in x..x+r*2 {
            let x1 : i32 = x as i32 + r as i32;
            let y1 : i32 = y as i32 + r as i32;
            if (x2 as i32 - x1)*(x2 as i32 - x1) + (y2 as i32 - y1)*(y2 as i32 - y1) 
                <= r as i32 * r as i32 {
                image.set_pixel(x2,y2,(0xff,0xff,0xff,0xff));
            }
        }
    }
}

fn draw_rect(image: &mut Image,x: usize,y:usize,w:usize,h:usize) {
    for i in y..y+h {
        for j in x..x+w {
            image.set_pixel(j,i,(0xff,0xff,0xff,0xff));
        }
    }
}

fn draw_triangle(image: &mut Image, x1: usize,y1: usize,x2: usize,y2: usize,x3: usize,y3: usize) {
    let sy1 = y1 as i32; let sy2 = y2 as i32; let sy3 = y3 as i32;
    let sx1 = x1 as i32; let sx2 = x2 as i32; let sx3 = x3 as i32;
    for y in min(sy1,min(sy2,sy3))..max(sy1,max(sy2,sy3)) {
        let x_l1; let x_l2; let x_l3;
        if sy2 - sy1 != 0 {
            x_l1 = ((y-sy1) as f32* ((sx2 - sx1) as f32 / (sy2 - sy1) as f32)) as i32 + sx1;
        }else {
            x_l1 = (sx2-sx1) + sx1;
        }
        if sy3 - sy1 != 0 {
            x_l2 = ((y-sy1) as f32* ((sx3 - sx1) as f32 / (sy3 - sy1) as f32)) as i32 + sx1;
        }else {
            x_l2 = (sx3-sx1) + sx1;
        }
        if sy3 - sy2 != 0 {
            x_l3 = ((y-sy2) as f32* ((sx3 - sx2) as f32 / (sy3 - sy2) as f32)) as i32 + sx2;
        }else {
            x_l3 = (sx3-sx2) + sx2;
        }
        println!("{}:{}",x_l1,x_l2);
        for x in min(sx1,min(sx2,sx3))..max(sx1,max(sx2,sx3)) {
            if y <= sy2 {
                if x >= min(x_l1,x_l2) && x <= max(x_l1,x_l2) {
                    image.set_pixel(x as usize,y as usize,image.render_color);
                }
            } else {
                if x >= min(x_l2,x_l3) && x <= max(x_l2,x_l3) {
                    image.set_pixel(x as usize,y as usize,image.render_color);
                }
            }
        }
    }
}

type Loc = (String,usize,usize);

#[derive(Debug)]
enum TokenType {
    Identifier,
    Int(u32),
    Float(f32),
    CharToken(char),
    Unknown,
}

#[derive(Debug)]
struct Token {
    file_path: String,
    col: usize,
    line: usize,
    literal: String,
    t_type: TokenType,
}

impl Token {
    pub fn new(t_type: TokenType,literal: String, loc: Loc) -> Self {
        Self{
            literal,
            t_type,
            file_path: loc.0,
            line: loc.1,
            col: loc.2,
        }
    }
}

#[derive(Debug)]
struct Lexer {
    file_path: String,
    source: Vec<char>,
    cur: usize,
    bol: usize,
    row: usize,
}

impl Lexer {
    pub fn new(file_path: String,source: String) -> Self {
        Self {
            file_path,
            source: source.chars().collect::<Vec<char>>(),
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn drop_line(&mut self) {
        while !self.is_empty() {
            if self.source[self.cur] == '\n'{
                self.drop();
                break;
            } else {
                self.drop();
            }
        }
    }

    fn drop(&mut self) {
        if !self.is_empty() {
            let char = self.source[self.cur];
            self.cur += 1;
            if char == '\n' {
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_whitespace() {
            self.drop();
        } 
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.trim_left();
        while !self.is_empty() {
            if self.source[self.cur] == '~' {
                self.drop_line();
                self.trim_left();
            } else {
                break;
            }
        }
        if self.is_empty() {
            return None
        }
        let loc:Loc = (self.file_path.clone(), self.row + 1, self.cur - self.bol + 1);
        let first = self.source[self.cur];
        if first.is_ascii_alphabetic() || first == '_' {
            let index = self.cur;
            while !self.is_empty() && 
                (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '_') {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            return Some(Token::new(TokenType::Identifier,literal,loc));
        }
        if first.is_ascii_digit() || first == '#' {
            let index = self.cur;
            self.drop();
            while !self.is_empty() && 
                (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '.') {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            let ttype_and_val = parse_numeric_token(&literal);
            return Some(Token::new(ttype_and_val,literal,loc));
        }
        // Fix This 
        if !self.is_empty(){
            self.drop();
        }
        return Some(Token::new(TokenType::Unknown,first.to_string(),loc));
    }
}

fn parse_numeric_token(literal: &String) -> TokenType {
    if literal.starts_with('#') {
        // #ffffffff #fff #fff0 #ffffff
        
    } else if literal.contains('.') {
        
    } else {
    }
    return TokenType::Int(0);
}

fn main() {
    let mut arg = args().into_iter();
    arg.next();
    loop {
        let Some(command) = arg.next() else {
            break;
        };
        match command.as_str() {
            "help" => {
                println!("De Do be bo poo pee");
                exit(0);
            },
            "--help" => {
                println!("De Do be bo poo pee");
                exit(0);
            },
            "--version" => {
                println!("De Do be bo poo pee");
                exit(0);
            },
            _ => {
                let source = fs::read_to_string(command.clone())
                    .expect("Can not Read the file");
                let mut lexer = Lexer::new(command.clone(),source);
                let mut token = lexer.next_token();
                while !token.is_none() {
                    println!("{:?}",token.unwrap());
                    token = lexer.next_token();
                }
            },
        }
    }
    // let mut media = Image::new();
    // media.fill((0,0,0,0));
    // draw_rect(&mut media,10,10,100,50);
    // draw_circle(&mut media,10,10,100);
    // media.set_render_color((0x20,0xe0,0xc4,0xff));
    // draw_triangle(&mut media,100,100,50,200,150,200);
    // media.generate_ppm_file();
}
