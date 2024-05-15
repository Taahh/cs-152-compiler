use std::process;

use crate::token::Token;

const BALANCED_BRACES: Vec<Token> = vec![];

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
    println!("{:?}", tokens[*i]);
    *i += 1;

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after function signature");
        process::exit(1);
    }
    BALANCED_BRACES.push(Token::OpeningBrace);
    println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingBrace) {
        handle(tokens, i);

        if matches!(&tokens[*i], Token::Return) {
            println!("{:?}", tokens[*i]);
            *i += 1;
            handle_declaration_contents(tokens, i, Token::SemiColon);
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        }
    }
    if !matches!(&tokens[*i], Token::ClosingBrace) {
        eprintln!("Expected '}}' after function body");
        process::exit(1);
    }
    BALANCED_BRACES.pop();
    println!("{:?}", tokens[*i]);
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

    handle_condition(tokens, i);

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after loop condition");
        process::exit(1);
    }
    BALANCED_BRACES.push(Token::OpeningBrace);
    println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingBrace) {
        handle(tokens, i);

        if matches!(&tokens[*i], Token::Return) {
            println!("{:?}", tokens[*i]);
            *i += 1;
            handle_declaration_contents(tokens, i, Token::SemiColon);
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        } else if matches!(&tokens[*i], Token::Break) {
            println!("{:?}", tokens[*i]);
            *i += 1;
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;

            if !matches!(&tokens[*i], Token::ClosingBrace) {
                eprintln!("Expected '}}' to close while loop");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        }
    }
    println!("{:?}", tokens[*i]);
    *i += 1;

}

fn handle_if(tokens: &Vec<Token>, i: &mut usize, is_else: bool) {
    println!("{:?}", tokens[*i]);
    *i += 1;

    if !is_else {
        handle_condition(tokens, i);
    }

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after loop condition");
        process::exit(1);
    }
    BALANCED_BRACES.push(Token::OpeningBrace);
    println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingBrace) {
        handle(tokens, i);

        if matches!(&tokens[*i], Token::Return) {
            println!("{:?}", tokens[*i]);
            *i += 1;
            handle_declaration_contents(tokens, i, Token::SemiColon);
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            break;
        } else if matches!(&tokens[*i], Token::Break) {
            println!("{:?}", tokens[*i]);
            *i += 1;
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;

            if !matches!(&tokens[*i], Token::ClosingBrace) {
                eprintln!("Expected '}}' to close if statement");
                process::exit(1);
            }
            break;
        }
    }
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
    if !matches!(&tokens[*i], Token::SemiColon) {
        eprintln!("Expected ';' after assignment");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;
}

fn identifier_or_number(tokens: &Vec<Token>, i: &mut usize) {
    if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32)) {
        eprintln!("Expected to pass in parameter value");
        process::exit(1);
    }
    println!("{:?}", tokens[*i]);
    *i += 1;
    handle_array_size(tokens, i);
}

fn arithmetic_or_comparison(tokens: &Vec<Token>, i: &mut usize) {
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
}

fn handle_array_size(tokens: &Vec<Token>, i: &mut usize) {
    if matches!(&tokens[*i], Token::OpeningBracket) {
        println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(tokens[*i], Token::Num(i32)) {
            eprintln!("Expected fixed size for array");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(tokens[*i], Token::ClosingBracket) {
            eprintln!("Expected ']' to close array size init");
            process::exit(1);
        }
        println!("{:?}", tokens[*i]);
        *i += 1;
    }
}

fn handle_declaration_contents(tokens: &Vec<Token>, i: &mut usize, end_token: Token) {
    let mut balanced_parentheses: Vec<&Token> = vec![];
    while tokens[*i] != end_token { // go until end of return statement
        // println!("hellooo");
        if matches!(tokens[*i], Token::OpenParentheses) {
            balanced_parentheses.push(&tokens[*i]);
            println!("{:?}", tokens[*i]);
            *i += 1;
        }
        identifier_or_number(tokens, i);

        if matches!(&tokens[*i-1], Token::Ident(String)) {
            if matches!(&tokens[*i], Token::OpenParentheses) { // function call
                println!("{:?}", tokens[*i]);
                *i += 1;
                while !matches!(&tokens[*i], Token::ClosingParentheses) {
                    identifier_or_number(tokens, i);
                    if matches!(&tokens[*i], Token::ClosingParentheses) {
                        break;
                    } else if matches!(&tokens[*i], Token::Comma) {
                        println!("{:?}", tokens[*i]);
                        *i += 1;
                        continue;
                    } else {
                        arithmetic_or_comparison(tokens, i);
                        identifier_or_number(tokens, i);
                    }
                }
                println!("{:?}", tokens[*i]);
                *i += 1;
            }
        }
        if tokens[*i] == end_token {
            break;
        }

        arithmetic_or_comparison(tokens, i);
        identifier_or_number(tokens, i);

        if matches!(&tokens[*i-1], Token::Ident(String)) {
            if matches!(&tokens[*i], Token::OpenParentheses) { // function call
                println!("{:?}", tokens[*i]);
                *i += 1;
                while !matches!(&tokens[*i], Token::ClosingParentheses) {
                    identifier_or_number(tokens, i);
                    if matches!(&tokens[*i], Token::ClosingParentheses) {
                        break;
                    } else if matches!(&tokens[*i], Token::Comma) {
                        println!("{:?}", tokens[*i]);
                        *i += 1;
                        continue;
                    } else {
                        arithmetic_or_comparison(tokens, i);
                        identifier_or_number(tokens, i);
                    }
                }
                println!("{:?}", tokens[*i]);
                *i += 1;
            }
        }
        if tokens[*i] == end_token {
            break;
        } else {
            if matches!(tokens[*i], Token::ClosingParentheses) {
                balanced_parentheses.pop();
                println!("{:?}", tokens[*i]);
                *i += 1;
            }
            arithmetic_or_comparison(tokens, i);
        }
    }
}

fn handle_declaration(tokens: &Vec<Token>, i: &mut usize) {
    println!("{:?}", tokens[*i]);
    *i += 1;

    handle_array_size(tokens, i);

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
            handle_if(tokens, i, false);
        },
        Token::Else => {
            handle_if(tokens, i, true);
        }
        Token::Ident(s) => { // identifier
            println!("{:?}", current_token);
            *i += 1;

            handle_array_size(tokens, i);

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
            println!("{:?}", tokens[*i]);
            *i += 1;

            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expeced ';' after print call");
                process::exit(1);
            }
            println!("{:?}", tokens[*i]);
            *i += 1;
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