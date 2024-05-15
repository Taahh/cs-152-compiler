use std::collections::HashSet;
use std::fs;
use crate::parser::parse_tokens;
use crate::token::{parse_token, parse_word_ending, Token};

mod token;
mod parser;

fn main() {
    let filename = "examples/function.tt";
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

    for x in code.chars() {
        if word_breaks.contains(&x) {
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

    parse_tokens(&tokens);
}