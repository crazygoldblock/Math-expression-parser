use std::{fs::{self, File}, io::{self, Write}, path::Path, ptr::null, time::Instant};
use linear_eval::linear_eval;
use rand::{thread_rng, Rng};
use token_parse::{parse_exp, token_to_string, Token};
use validation::{add_implicit_tokens, validate_tokens};

mod token_parse;
mod validation;
mod token_tree;
mod linear_eval;

const DEBUG: bool = cfg!(debug_assertions);
// -.5(1+2)(-3+4) * 5 + 3 * 2(1*2_0)
// (1 + 2 * 3) * 4 (1 + 2 + 3) * 5
fn main() {

    measure();

    //measure();

    /*println!("{}", size_of::<Result<u64, Infallible>>() );

    let mut input = String::new();
    File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();

    let ins = Instant::now();

    match eval_exp(&input) {
        Ok(result) => println!("result: {}", result),
        Err(err) => println!("{}", err),
    }

    println!("{:?}", ins.elapsed());*/

    //println!("rr {}", eval_exp("-.5(1+2)(-3+4) * 5 + 3 * 2(1*2_0)").unwrap());

    loop {
        let input = get_input("Input expression...");

        match eval_exp(&input) {
            Ok(result) => println!("result: {}", result),
            Err(err) => println!("{}", err),
        }
    } 
}
fn eval_exp(input: &str) -> Result<f32, String> {

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

    validate_tokens(&tokens)?;

    add_implicit_tokens(&mut tokens);

    if DEBUG {
        print!("Implicit tokens:");
        for t in tokens.iter() {
            print!(" {}", token_to_string(t));
        }
        println!("");
    }

    Ok(linear_eval(tokens))
}
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    return buf.trim().to_string();
}
fn measure() {

    generate_test();

    let s = fs::read_to_string("./input.txt").unwrap();

    let ins = Instant::now();

    let r = eval_exp(&s).unwrap();

    println!("{:?}", ins.elapsed());
    println!("Result: {}", r);
}
fn generate_test() {

    if Path::new("./input.txt").exists() {
        return;
    }

    let mut res = String::new();

    let operators = vec!['+', '-', '*', '/'];
    let mut brackets = 0;

    for _ in 0..1_000_000 {

        if thread_rng().gen_range(0..8) == 0 {
            if brackets < 30 {
                res.push('(');
            brackets += 1;
            }
        }

        res.push_str(&(thread_rng().gen_range(0..10000) as f32 / 100f32).to_string());

        if thread_rng().gen_range(0..8) == 0 {
            if brackets > 0 {
                res.push(')');
                brackets -= 1;
            }
        }

        res.push(operators[thread_rng().gen_range(0..4)]);
    }
    res.push_str(&(thread_rng().gen_range(0..10000) as f32 / 100f32).to_string());

    for _ in 0..brackets {
        res.push(')');
    }

    fs::write("./input.txt", res).unwrap();
}