pub fn parse_exp(exp: &str) -> Result< Vec<Token>, String > {
    let mut res = Vec::new();

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
fn parse_number(p: &mut Parser) -> Result<f64, String> {

    let mut chars = Vec::new();
    p.back();

    while p.has_next() {
        let c = p.consume();

        if c == b' ' || c == b'_' {
            continue;
        }

        if (c as char).is_ascii_digit() || c == b'.' {
            chars.push(c);
        }
        else {
            p.back();
            break;
        }
    }

    

    if chars.len() == 0 {
        return  Err((p.consume() as char).to_string());
    }

    let s = String::from_utf8(chars).unwrap();

    match s.parse() {
        Ok(f) => Ok(f),
        Err(_) => Err(s.clone()),
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
#[derive(Clone)]
pub enum Token {
    Number(f64),
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
