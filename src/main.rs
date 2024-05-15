use std::collections::HashSet;
use std::fs;
use crate::parser::parse_tokens;
use crate::token::{parse_token, parse_word_ending, Token};

mod token;
mod parser;

fn main() {
    let filename = "examples/nested_loop.tt";
    let result = fs::read_to_string(filename);
    let code = match result {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => {
            code
        }
    };

    let word_breaks: HashSet<char> = HashSet::from(['(', ')', '{', '}', ';', ' ', '[', ']', ',', '.']);
    let mut word_buffer = String::new();
    let mut tokens: Vec<Token> = vec![];

    let mut is_comment = false;
    for x in code.chars() {
        if x == '#' {
            is_comment = true;
        }
        if is_comment && x == '\n' {
            is_comment = false;
            word_buffer.clear();
            continue;
        }
        if !is_comment && word_breaks.contains(&x) {
            word_buffer = word_buffer.trim().to_string();
            if !word_buffer.is_empty() {
                let token = parse_token(&word_buffer);
                tokens.push(token);
            }
            if x != ' ' {
                println!("{}", x);
                tokens.push(parse_word_ending(x));
            }
            word_buffer.clear();
        } else {
            word_buffer.push(x);
        }
    }

    println!("Tokens: {:?}", tokens);

    unsafe { parse_tokens(&tokens); }
}