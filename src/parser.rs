use std::process;

use crate::token::Token;

pub fn handle_function(tokens: &Vec<Token>, i: &mut usize) {
    if !matches!(&tokens[*i], Token::Ident(ref String)) {
        eprintln!("Expected name for function");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;

    if !matches!(&tokens[*i], Token::OpenParentheses) {
        eprintln!("Expected '(' for function signature");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingParentheses) {
        if !matches!(&tokens[*i], Token::Int) {
            eprintln!("Expected data type 'int' for parameter");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(&tokens[*i], Token::Ident(String)) {
            eprintln!("Expected identifier for parameter");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if matches!(&tokens[*i], Token::Comma) {
            println!("{:?}", tokens[*i]);
            *i += 1;
            continue;
        } else if matches!(&tokens[*i+1], Token::ClosingParentheses) {
            break;
        }
    }
    println!("baba{:?}", tokens[*i]);
    *i += 1;

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after function signature");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::Return) {
        handle(tokens, i);

        if matches!(&tokens[*i+1], Token::Return) {
            break;
        }
    }
    println!("{:?}", tokens[*i]);
    *i += 1;
    handle_declaration_contents(tokens, i, Token::SemiColon);
    println!("baba{:?}", tokens[*i]);
    *i += 1;
    if !matches!(&tokens[*i], Token::ClosingBrace) {
        eprintln!("Expected '}}' after function body");
        process::exit(1);
    }
    println!("baba{:?}", tokens[*i]);
    *i += 1;
}

fn handle_condition(tokens: &Vec<Token>, i: &mut usize) {
    if matches!(&tokens[*i], Token::Ident(String)) || matches!(&tokens[*i], Token::Int) {
        println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(&tokens[*i], Token::LessThan) &&
            !matches!(&tokens[*i], Token::LessThanEqual) &&
            !matches!(&tokens[*i], Token::GreaterThan) &&
            !matches!(&tokens[*i], Token::GreaterThanEqual) &&
            !matches!(&tokens[*i], Token::Equality) &&
            !matches!(&tokens[*i], Token::NotEqual)
        {
            eprintln!("Error : Expected '<, <=, >, >=, ==, !=' for the boolean inequality.");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32))
        {
            eprintln!("Error : Expected an identifier or number for the inequality.");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;
    }
}

fn handle_loop(tokens: &Vec<Token>, i: &mut usize) {
    println!("{:?}", tokens[*i]);
    *i += 1;
}

fn handle_if(tokens: &Vec<Token>, i: &mut usize) {
    println!("{:?}", tokens[*i]);
    *i += 1;
}

fn handle_assignment(tokens: &Vec<Token>, i: &mut usize) {
    if !matches!(&tokens[*i], Token::Assign) {
        eprintln!("'=' required for variable assignment.");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;
    handle_declaration_contents(tokens, i, Token::SemiColon);
}

fn handle_declaration_contents(tokens: &Vec<Token>, i: &mut usize, end_token: Token) {
    while tokens[*i] != end_token { // go until end of return statement
        // println!("hellooo");
        if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32)) {
            eprintln!("Expected to reference to identifier or number");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if matches!(&tokens[*i-1], Token::Ident(String)) {
            println!("HELLo {:?}", tokens[*i+1]);
            if matches!(&tokens[*i], Token::OpenParentheses) { // function call
                println!("{:?}", tokens[*i]);
                *i += 1;
                while !matches!(&tokens[*i], Token::ClosingParentheses) {
                    if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32)) {
                        eprintln!("Expected to pass in parameter value");
                        process::exit(1);
                    }
                    println!("{:?}", tokens[*i]);
                    *i += 1;

                    if matches!(&tokens[*i], Token::Comma) {
                        println!("{:?}", tokens[*i]);
                        *i += 1;
                        continue;
                    }
                }
                println!("{:?}", tokens[*i]);
                *i += 1;
            }
        }
        if tokens[*i] == end_token {
            println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        }


        if !matches!(&tokens[*i], Token::Plus)
            && !matches!(&tokens[*i], Token::Subtract)
            && !matches!(&tokens[*i], Token::Divide)
            && !matches!(&tokens[*i], Token::Multiply)
            && !matches!(&tokens[*i], Token::Modulus)
            && !matches!(&tokens[*i], Token::Equality)
            && !matches!(&tokens[*i], Token::NotEqual)
            && !matches!(&tokens[*i], Token::LessThanEqual)
            && !matches!(&tokens[*i], Token::LessThan)
            && !matches!(&tokens[*i], Token::GreaterThanEqual)
            && !matches!(&tokens[*i], Token::GreaterThan)
        {
            eprintln!("Expected '+, -, /, *, ==, !=, <=, <, >=, >=' when performing arithmetic operations or comparing multiple identifiers / numbers");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32)) {
            eprintln!("Expected to reference to identifier or number");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if matches!(&tokens[*i-1], Token::Ident(String)) {
            if matches!(&tokens[*i], Token::OpenParentheses) { // function call
                println!("{:?}", tokens[*i]);
                *i += 1;
                while !matches!(&tokens[*i], Token::ClosingParentheses) {
                    if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32)) {
                        eprintln!("Expected to pass in parameter value");
                        process::exit(1);
                    }
                    println!("{:?}", tokens[*i]);
                    *i += 1;

                    if matches!(&tokens[*i], Token::Comma) {
                        println!("{:?}", tokens[*i]);
                        *i += 1;
                        continue;
                    }
                }
                println!("{:?}", tokens[*i]);
                *i += 1;
            }
        }

        if tokens[*i+1] == end_token {
            println!("{:?}", tokens[*i+1]);
            *i += 2;
            break;
        } else {
            continue;
        }
    }
}

fn handle_declaration(tokens: &Vec<Token>, i: &mut usize) {
    println!("{:?}", tokens[*i]);
    *i += 1;

    if !matches!(&tokens[*i], Token::Ident(String)) {
        eprintln!("Expected identifier for int variable.");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;

    if matches!(&tokens[*i], Token::SemiColon) {
        println!("{:?}", tokens[*i]);
        *i += 1;
        return;
    } else if matches!(&tokens[*i], Token::Assign) {
        println!("{:?}", tokens[*i]);
        *i += 1;
        println!("{:?}", tokens[*i]);
        handle_declaration_contents(tokens, i, Token::SemiColon);
    }
}

fn handle(tokens: &Vec<Token>, i: &mut usize) {
    let current_token = &tokens[*i];
    match current_token {
        Token::Func => {
            println!("{:?}", current_token);
            *i += 1;
            handle_function(tokens, i);
        }
        Token::While => { // while loop
            handle_loop(tokens, i);
        }
        Token::If => { // if statement
            handle_if(tokens, i);
        }
        Token::Ident(s) => { // identifier
            println!("{:?}", current_token);
            *i += 1;
            handle_assignment(tokens, i);
        }
        Token::Print => { // print statement
            println!("{:?}", current_token);
            *i += 1;
            if !matches!(&tokens[*i], Token::OpenParentheses) {
                eprintln!("Expected '(' after print call");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;
            handle_declaration_contents(tokens, i, Token::ClosingParentheses);
        }
        Token::Int => { // int variable declaration
            handle_declaration(tokens, i);
        }
        _ => {}
    }
}

pub fn parse_tokens(tokens: &Vec<Token>) {
    let mut i = 0;
    while i < tokens.len() {
        handle(tokens, &mut i);
    }
}