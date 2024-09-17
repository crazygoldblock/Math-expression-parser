use crate::token_parse::{Operator, Token};

pub fn validate_tokens(tokens: &[Token]) -> Result<(), String> {

    let mut brackets = 0;
    let mut last_token = NONE;

    const NONE: i32 = -1;
    const OPEN: i32 = 0;
    const CLOSE: i32 = 1;
    const NUMBER: i32 = 2;
    const OPERATOR: i32 = 3;

    for t in tokens {
        match t {
            Token::Number(_) => {
                if last_token == NUMBER { return Err("Unexpected number after another number".to_string()); }
                last_token = NUMBER;
            },
            Token::Operator(o) => {

                if last_token == OPERATOR { return Err("Unexpected operator after another operator".to_string()); }

                if last_token == OPEN {
                    if *o != Operator::Minus {
                        return Err("Unexpected operator after opening bracket".to_string());
                    }
                } 
                last_token = OPERATOR;
            },
            Token::Bracket(b) => {
                if *b { brackets += 1; last_token = OPEN; }
                else {
                    if last_token == OPERATOR { return Err("Unexpected closing bracket after operator".to_string()); }
                    if last_token == OPEN { return Err("Unexpected closing bracket immediately after opening".to_string()); }
                    if brackets == 0 { return Err("Unexpected closing bracket without opening".to_string()); }
                    else { brackets -= 1; }
                    last_token = CLOSE;
                }
            },
        }
    }

    if brackets > 0 {
        return Err("Unclosed bracket/s".to_string());
    }
    if last_token == OPERATOR {
        return Err("Unexpected operator at the end".to_string());
    }

    return Ok(());
}
pub fn add_implicit_tokens(tokens: &mut Vec<Token>) {

    let mut indexes = Vec::new();
    let mut last_token = NONE;

    const UNKNOWN: i32 = -2;
    const NONE: i32 = -1;
    const OPEN: i32 = 0;
    const CLOSE: i32 = 1;
    const NUMBER: i32 = 2;
    const OPERATOR: i32 = 3;

    for (i, t) in tokens.iter().enumerate() {
        match t {
            Token::Number(_) => {
                if last_token == CLOSE { indexes.push(i); }
                last_token = NUMBER;
            },
            Token::Operator(_) => {
                last_token = OPERATOR;
            },
            Token::Bracket(b) => {
                if (last_token == NUMBER || last_token == CLOSE) && *b { 
                    indexes.push(i);
                }
                if *b { last_token = OPEN; }
                else { last_token = CLOSE; }
            },
        }
    }

    tokens.reserve(indexes.len());

    for i in indexes.iter().rev() {
        tokens.insert(*i, Token::Operator(Operator::Mul));
    }

    last_token = -1;
    indexes.clear();

    for (i, t) in tokens.iter().enumerate() {

        if let Token::Operator(o) = t {
            if *o == Operator::Minus {
                if last_token == NONE { indexes.push(i); }
                if last_token == OPEN { indexes.push(i); }
            }  
        }
        
        if let Token::Bracket(b) = t {
            if *b { 
                last_token = OPEN; 
                continue;
            }
        }
        last_token = UNKNOWN;
    }

    tokens.reserve(indexes.len());

    

    for i in indexes.iter().rev() {
        tokens.insert(*i, Token::Number(0.0));
    }
}