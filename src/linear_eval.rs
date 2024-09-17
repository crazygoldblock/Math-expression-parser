use std::usize;

use crate::token_parse::{token_to_string, Operator, Token};

pub fn linear_eval(tokens: Vec<Token>) -> f64 {
    let tokens = tokens.into_iter().map(|x| Some(x)).collect();
    let mut parser = Parser::new(tokens);

    linear_eval_recursive(&mut parser)
}

fn linear_eval_recursive(tokens: &mut Parser) -> f64 {

    let mut adds = Vec::new();
    let mut muls = Vec::new();

    while tokens.has_next() {
        match tokens.consume() {
            Some(s) => match s {
                Token::Number(_) => {},
                Token::Operator(operator) => {
                    match operator {
                        Operator::Plus => adds.push(tokens.index),
                        Operator::Minus => adds.push(tokens.index),
                        Operator::Mul => muls.push(tokens.index),
                        Operator::Div => muls.push(tokens.index),
                    }
                },
                Token::Bracket(b) => {
                    if *b {
                        let ind = tokens.index;
                        linear_eval_recursive(tokens);
                        tokens.tokens[ind] = None;
                        print(&tokens.tokens);
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
        tokens.evaluate_operator(*mul);
    }

    for add in adds.iter() {
        tokens.evaluate_operator(*add);
    }

    let last = 
    if let Some(n) = adds.last()      { *n }
    else if let Some(n) = muls.last() { *n }
    else                                      {  0 };

    if let Token::Number(n) = tokens.tokens[last].as_ref().unwrap() {
        return *n;
    }

    unreachable!();
}
struct Parser {
    tokens: Vec<Option<Token>>,
    index: usize,
}
impl Parser {
    fn new(tokens: Vec<Option<Token>>) -> Self {
        Parser { tokens, index: usize::MAX }
    }
    fn consume(&mut self) -> &Option<Token> {
        self.index = self.index.wrapping_add(1);
        &self.tokens[self.index]
    }
    fn has_next(&self) -> bool {
        self.tokens.len() > self.index.wrapping_add(1)
    }
    fn evaluate_operator(&mut self, index: usize) {

        let o = if let Token::Operator(o) = self.tokens[index].as_ref().unwrap() {
            *o
        }
        else {
            unreachable!("index must point to operator");
        };

        let mut left = 0.0;
        let mut right = 0.0;
        let mut i1 = 0;
        let mut i2 = 0;

        for i in 1.. {
            if let Some(token) = &self.tokens[index - i] {
                if let Token::Number(n) = token {
                    left = *n;
                    i1 = index - i;
                    break;
                }
                else {
                    unreachable!("unreachable after validation  {}", token_to_string(&token));
                }
            }
        }

        for i in 1.. {
            if let Some(token) = &self.tokens[index + i] {
                if let Token::Number(n) = token {
                    right = *n;
                    i2 = index + i;
                    break;
                }
                else {
                    unreachable!("unreachable after validation  {}", token_to_string(&token));
                }
            }
        }

        let res = match o {
            Operator::Plus => left + right,
            Operator::Minus => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        };

        self.tokens[index] = Some(Token::Number(res));

        self.tokens[i1] = None;
        self.tokens[i2] = None;
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