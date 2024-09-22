use core::str;

pub fn parse_exp(exp: &str) -> Result< Vec<Token>, String > {

    // 40.88
    //
    //

    let mut res = Vec::with_capacity(exp.len() / 2);
    let mut parser = Parser::new(exp.to_string().into_bytes());

    while parser.has_next() {
        match parser.consume() {
            b' ' | b'_' => {},

            b'(' => res.push(Token::Bracket(true)),
            b')' => res.push(Token::Bracket(false)),

            b'+' => res.push(Token::Operator(Operator::Plus)),
            b'-' => res.push(Token::Operator(Operator::Minus)),
            b'*' => res.push(Token::Operator(Operator::Mul)),
            b'/' => res.push(Token::Operator(Operator::Div)),

            _ => res.push(Token::Number(parse_number(&mut parser)?)),
        }
    }
    return Ok(res);
}
fn parse_number(p: &mut Parser) -> Result<f32, String> {
    
    p.back();
    let start = p.index;
    let mut end = p.index;

    while p.has_next() {
        let c = p.consume();

        if (c as char).is_ascii_digit() || c == b'.' {
            end += 1;
        }
        else {
            p.back();
            break;
        }
    }

    if start == end {
        return  Err((p.consume() as char).to_string());
    }
    
    let s = str::from_utf8(&p.buffer[start..end]).unwrap().trim();

    match s.parse() {
        Ok(f) => Ok(f),
        Err(_) => Err(s.to_string()),
    }
}
struct Parser {
    index: usize,
    buffer: Vec<u8>,
}
impl Parser {
    pub fn new(buffer: Vec<u8>) -> Self {
        Parser { buffer, index: 0 }
    }
    pub fn consume(&mut self) -> u8 {
        self.index += 1;
        self.buffer[self.index -1]
    }
    pub fn has_next(&self) -> bool {
        self.buffer.len() > self.index
    }
    pub fn back(&mut self) {
        self.index -= 1;
    }
}
#[derive(Clone, Copy)]
pub enum Token {
    Number(f32),
    Operator(Operator),
    Bracket(bool),
}
#[derive(PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div
}
pub fn token_to_string(t: &Token) -> String {
    match t {
        Token::Number(n) => format!("{n}"),
        Token::Operator(o) => format!("{}", operator_to_string(o)),
        Token::Bracket(b) => {
            if *b { "(".to_string() }
            else { ")".to_string() }
        },
    }
}
pub fn operator_to_string(o: &Operator) -> char {
    match o {
        Operator::Plus => '+',
        Operator::Minus => '-',
        Operator::Mul => '*',
        Operator::Div => '/',
    }
}
