use std::usize;

use crate::{token_parse::{token_to_string, Operator, Token}, DEBUG};

pub fn linear_eval(tokens: Vec<Token>) -> f32 {
    let tokens = tokens.into_iter().map(|x| Some(x)).collect();
    let mut parser = Parser::new(tokens);

    linear_eval_recursive(&mut parser, &mut 0, &mut 0)
}

fn linear_eval_recursive(tokens: &mut Parser, count: &mut usize, max: &mut usize) -> f32 {

    let mut adds = Vec::new();
    let mut muls = Vec::new();

    

    while tokens.has_next() {
        match tokens.consume() {
            Some(s) => match s {
                Token::Number(_) => {},
                Token::Operator(operator) => {
                    match operator {
                        Operator::Plus => adds.push((tokens.index, operator)),
                        Operator::Minus => adds.push((tokens.index, operator)),
                        Operator::Mul => muls.push((tokens.index, operator)),
                        Operator::Div => muls.push((tokens.index, operator)),
                    }
                },
                Token::Bracket(b) => {
                    if b {
                        let ind = tokens.index;
                        linear_eval_recursive(tokens, count, max);
                        tokens.tokens[ind] = None;
                        
                        if DEBUG { print(&tokens.tokens); }
                        else {
                            //tokens.tokens = tokens.tokens.drain(..).filter(|x| x.is_some()).collect(); 
                        } 
                        tokens.index = ind;
                    } 
                    else {
                        tokens.tokens[tokens.index] = None;
                        break;
                    }
                },
            },
            None => {},
        }
    }

    for mul in muls.iter() {
        tokens.evaluate_operator(mul.0, mul.1);
    }

    for add in adds.iter() {
        tokens.evaluate_operator(add.0, add.1);
    }

    let last = 
    if let Some(n) = adds.last()      { n.0 }
    else if let Some(n) = muls.last() { n.0 }
    else                                                  { 0   };

    let n = if let Token::Number(n) = tokens.tokens[last].as_ref().unwrap() {
        *n
    }
    else {
        unreachable!();
    };

    *count += muls.len() + adds.len();
    
    //let len = tokens.tokens.len();

    if *count > 10_000 && (*max - tokens.index) > 10_000  {
        filter_vec(&mut tokens.tokens, tokens.index + 1, *max);
        *count = 0;
        *max = 0;
    }

    *max = tokens.index.max(*max);

    return n;    
}
struct Parser {
    tokens: Vec<Option<Token>>,
    index: usize,
}
impl Parser {
    fn new(tokens: Vec<Option<Token>>) -> Self {
        Parser { tokens, index: usize::MAX }
    }
    fn consume(&mut self) -> Option<Token> {
        self.index = self.index.wrapping_add(1);
        self.tokens[self.index].clone()
    }
    fn has_next(&self) -> bool {
        self.tokens.len() > self.index.wrapping_add(1)
    }
    fn evaluate_operator(&mut self, index: usize, o: Operator) {

        let (i1, left_opt) = self.tokens[..index].iter()
            .rev()
            .enumerate()
            .find_map(|x| x.1.clone().map(|a| (x.0, a)))
            .unwrap();

        let (i2, right_opt) = self.tokens[index + 1..].iter()
            .enumerate()
            .find_map(|x| x.1.clone().map(|a| (x.0, a)))
            .unwrap();

        let (left, right) = match (left_opt, right_opt) {
            (Token::Number(n1), Token::Number(n2)) => (n1, n2),
            _ => panic!("should always be numbers"),
        };

        let res = match o {
            Operator::Plus => left + right,
            Operator::Minus => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        };

        self.tokens[index] = Some(Token::Number(res));

        self.tokens[index - i1 - 1] = None;
        self.tokens[index + i2 + 1] = None;
    }
}

fn print(tokens: &[Option<Token>]) {
    print!("Tokens: ");
    for t in tokens {
        match t {
            Some(t2) => print!("{} ", token_to_string(t2)),
            None => print!("_ "),
        }
    }
    println!("");
}
fn filter_vec(vec: &mut Vec<Option<Token>>, start: usize, end: usize) {
    let mut a = start;
    let mut b = start;

    while a < end {
        if vec[a].is_some() {
            vec.swap(a, b);
            b += 1;
        }
        a += 1;
    }

    vec.copy_within(end.., b);

    vec.truncate(b + vec.len() - end);
}
#[test]
fn testtt() {
    let mut vec = vec![ 
        None, None, None, Some(Token::Number(3f32)), None, 
        None, Some(Token::Number(3f32)), Some(Token::Number(3f32)), Some(Token::Number(3f32)), Some(Token::Number(3f32)), 
        None, None, None ];

    filter_vec(&mut vec, 5, 11);

    for n in vec {
        match n {
            Some(a) => println!("{}", token_to_string(&a)),
            None => println!("None"),
        }
    }

    panic!("heh");
}