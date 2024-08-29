use std::io;

use token_parse::{parse_exp, token_to_string};
use token_tree::evaluate_exp;
use validation::{add_implicit_tokens, validate_tokens};

mod token_parse;
mod validation;
mod token_tree;

const DEBUG: bool = true;
// .5(1+2)(3+4) * 5 + 3 * 2(1*2_0)

fn main() {
    loop {
        let input = get_input("Input expression...");

        match eval_exp(&input) {
            Ok(result) => println!("result: {}", result),
            Err(err) => println!("{}", err),
        }
    }      
}
fn eval_exp(input: &str) -> Result<f64, String> {

    let mut tokens = match parse_exp(input) {
        Ok(e) => e,
        Err(s) => {
            if s.len() == 1 {
                return Err(format!("Unexpected character: \"{}\"", s));
            }
            else {
                return Err(format!("Invalid number format: \"{}\"", s));
            }
        },
    };

    if DEBUG {
        print!("Raw tokens:");
        for t in tokens.iter() {
            print!(" {}", token_to_string(t));
        }
        println!("");
    }

    if let Err(s) = validate_tokens(&tokens) {
        return Err(s);
    }

    add_implicit_tokens(&mut tokens);

    if DEBUG {
        print!("Implicit tokens:");
        for t in tokens.iter() {
            print!(" {}", token_to_string(t));
        }
        println!("");
    }

    Ok(evaluate_exp(&tokens))
}
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    return buf.trim().to_string();
}