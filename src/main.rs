use std::{env, fs, process};

mod interpreter;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    NotToken,
    Plus,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Assign,
    Comma,
    SemiColon,
    Num(i32),
    Ident(String),
    If,
    Else,
    Break,
    While,
    Func,
    Return,
    Int,
    OpenParentheses,
    ClosingParentheses,
    OpeningBrace,
    ClosingBrace,
    OpeningBracket,
    ClosingBracket,
    Print,
    Read,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    Equality,
    NotEqual,
}


fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
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

    // Start Here!!
    let mut buffer = String::new();
    let mut word_buffer = String::new();
    let mut is_comment = true & false | false;
    let mut tokens: Vec<String> = vec![];
    let mut token_types: Vec<Token> = vec![];
    let mut code_index: usize = 0;

    for (c) in code.chars() {
        buffer.push(c);
        if c == '#' // dump buffer
        {
            is_comment = true;
        }
        if c == '\n' || code_index == code.clone().len() - 1
        {
            // comment identification
            if is_comment == true
            {
                // print!("Filtered comment: ");
                // print!("{}", buffer);
                is_comment = false;
            } else {
                for ch in buffer.chars()
                {
                    if ch == ' ' || ch == '(' || ch == ')' || ch == '}' || ch == '{' || ch == '[' || ch == ']' || ch == ';' || ch == ','
                    {
                        if (word_buffer.len() > 0) {
                            word_buffer = word_buffer.to_lowercase();

                            token_types.push(parse(word_buffer.clone()));

                            tokens.push(word_buffer.clone());
                        }

                        if (ch != ' ') {
                            token_types.push(parse(ch.to_string().clone()));
                            tokens.push(ch.to_string().clone());
                        }
                        word_buffer.clear();
                    } else {
                        if ch != '\n'
                        {
                            // let token = parse(ch.clone().to_string());
                            // if token != Token::NotToken {
                            //     token_types.push(token);
                            // }

                            word_buffer.push(ch);
                        }
                    }
                }
            };
            buffer.clear();
        }
        code_index += 1;
    }

    fn parse_symbols(s: String) -> Token {
        return match s.as_str() {
            "+" => Token::Plus,
            "-" => Token::Subtract,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "%" => Token::Modulus,
            "=" => Token::Assign,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "break" => Token::Break,
            "read" => Token::Read,
            "func" => Token::Func,
            "return" => Token::Return,
            "int" => Token::Int,
            "(" => Token::OpenParentheses,
            ")" => Token::ClosingParentheses,
            "{" => Token::OpeningBrace,
            "}" => Token::ClosingBrace,
            "[" => Token::OpeningBracket,
            "]" => Token::ClosingBracket,
            "," => Token::Comma,
            ";" => Token::SemiColon,
            "==" => Token::Equality,
            "!=" => Token::NotEqual,
            "print" => Token::Print,
            ">" => Token::GreaterThan,
            "<" => Token::LessThan,
            ">=" => Token::GreaterThanEqual,
            "<=" => Token::LessThanEqual,
            _ => Token::NotToken,
        };
    }

    fn parse(s: String) -> Token {
        let token = parse_symbols(s.clone());
        if token == Token::NotToken {
            let test_if_num = s.parse::<i32>();
            return match test_if_num {
                Ok(num) => {
                    Token::Num(num)
                }
                Err(_) => {
                    if (s.as_bytes()[0].is_ascii_digit()) {
                        eprintln!("Incorrect identifier! {}", s);
                        process::exit(1);
                    }
                    Token::Ident(s)
                }
            };
        }

        return token;
    }
    // output
    println!("{:?}", tokens);
    println!("{:?}", token_types);
    // println!("{}, {}", token_types.len(), tokens.len());
    let mut symbol_table = String::new();

    fn parse_condition(tokens: &Vec<String>, i: &mut usize) {
        if matches!(parse(tokens[*i].clone()), Token::Ident(String)) || matches!(parse(tokens[*i].clone()), Token::Int) {
            println!("{}, {}", i, tokens[*i].clone());
            *i += 1;
            if !matches!(parse(tokens[*i].clone()), Token::LessThan) &&
                !matches!(parse(tokens[*i].clone()), Token::LessThanEqual) &&
                !matches!(parse(tokens[*i].clone()), Token::GreaterThan) &&
                !matches!(parse(tokens[*i].clone()), Token::GreaterThanEqual) &&
                !matches!(parse(tokens[*i].clone()), Token::Equality) &&
                !matches!(parse(tokens[*i].clone()), Token::NotEqual)
            {
                eprintln!("Error : Expected '<, <=, >, >=, ==, !=' for the boolean inequality.");
                process::exit(1);
            }
            println!("{}, {}", i, tokens[*i].clone());
            *i += 1;

            if !matches!(parse(tokens[*i].clone()), Token::Ident(String)) && !matches!(parse(tokens[*i].clone()), Token::Num(i32))
            {
                println!("{}", tokens[*i]);
                eprintln!("Error : Expected an identifier or number for the inequality.");
                process::exit(1);
            }
            println!("{}, {}", i, tokens[*i].clone());
            *i += 1;
        }
    }

    fn parse_expr_content(tokens: &Vec<String>, i: &mut usize)
    {
        if !matches!(parse(tokens[*i].clone()), Token::Assign) {
            if matches!(parse(tokens[*i].clone()) , Token::SemiColon) {
                return;
            } else {
                eprintln!("Error : Expected '=' for the variable assignment or ';' for end of line.");
                process::exit(1);
            }
        } else { //token is equal to assignment(=), now handle right hand side
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;
            while !matches!(parse(tokens[*i].clone()) , Token::SemiColon) {
                if !matches!(parse(tokens[*i].clone()), Token::Ident(String)) && !matches!(parse(tokens[*i].clone()), Token::Num(i32)) {
                    eprintln!("Error : Expected an identifier for the variable declaration.");
                    process::exit(1);
                } else {
                    println!("{}, {}", *i, tokens[*i].clone());
                    // its an identifier, now move on to the next token
                    *i += 1;
                    if matches!(parse(tokens[*i].clone()) , Token::SemiColon) { // is it a ; ?
                        // println!("{}, {}", *i, tokens[*i].clone());
                        return; // ok, exit
                    } else { // no? then it should be an operator
                        /*
                        ">" => Token::GreaterThan,
                        "<" => Token::LessThan,
                        ">=" => Token::GreaterThanEqual,
                        "<=" => Token::LessThanEqual,
                        "+" => Token::Plus,
                        "-" => Token::Subtract,
                        "*" => Token::Multiply,
                        "/" => Token::Divide,
                        "%" => Token::Modulus,
                        "=" => Token::Assign,
                        */
                        match parse(tokens[*i].clone()) {
                            Token::Plus => {}
                            Token::Subtract => {}
                            Token::Multiply => {}
                            Token::Divide => {}
                            Token::Modulus => {}
                            _ => {}
                        };
                    }
                }
                println!("{}, {}", *i, tokens[*i].clone());
                *i += 1;
            }
        }
    }

    fn parse_expr(tokens: &Vec<String>, i: &mut usize)
    {
        if matches!(parse(tokens[*i].clone()), Token::Int) {
            // int kwd read, expect identifier
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;
            if !matches!(parse(tokens[*i].clone()), Token::Ident(String)) {
                eprintln!("Error : Expected an identifier for the variable declaration.");
                process::exit(1);
            }
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;

            parse_expr_content(tokens, i);
        } else if matches!(parse(tokens[*i].clone()), Token::Ident(String)) {
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;

            parse_expr_content(tokens, i);
        } else if matches!(parse(tokens[*i].clone()), Token::Print) {
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;

            if !matches!(parse(tokens[*i].clone()), Token::OpenParentheses) {
                eprintln!("Expected '(' before identifier to print");
                process::exit(1);
            }
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;

            if !matches!(parse(tokens[*i].clone()), Token::Ident(String)) {
                eprintln!("Expected valid identifier to print its content");
                process::exit(1);
            }
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;

            if !matches!(parse(tokens[*i].clone()), Token::ClosingParentheses) {
                eprintln!("Expected ')' after identifier to print");
                process::exit(1);
            }
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;

            if !matches!(parse(tokens[*i].clone()), Token::SemiColon) {
                eprintln!("Expected ';' after print call");
                process::exit(1);
            }
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;
        } else if matches!(parse(tokens[*i].clone()), Token::SemiColon) {
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;
        } else if !matches!(parse(tokens[*i].clone()), Token::NotToken) {
            println!("{}, {}", *i, tokens[*i].clone());
            *i += 1;
        }
    }

    fn parse_body(tokens: &Vec<String>, i: &mut usize) -> bool {
        if matches!(parse(tokens[*i].clone()), Token::While)
        {
            println!("{}, {}", i, tokens[*i].clone());
            *i += 1;
            parse_condition(tokens, i);
            if !matches!(parse(tokens[*i].clone()), Token::OpeningBrace) {
                eprintln!("Expected '{{' before loop body");
                process::exit(1);
            }
            println!("{}, {}", i, tokens[*i].clone());
            *i += 1;

            while !matches!(parse(tokens[*i].clone()), Token::ClosingBrace) {
                // parse expression
                parse_body(tokens, i);
            }
            println!("{}, {}", i, tokens[*i].clone());
            return true;
        } else {
            parse_expr(tokens, i);
            return true;
        }

        return false;
    }

    fn parse_statements(tokens: &Vec<String>) -> Result<Option<()>, String>
    {
        let mut i: usize = 0;
        while i < tokens.len()
        {
            match parse_symbols(tokens[i].clone()) {
                Token::Func =>
                    {
                        // identifier needed
                        i += 1;
                        println!("{}, {}", i, tokens[i].clone());
                        if !matches!(parse(tokens[i].clone()), Token::Ident(String)) {
                            return Err(String::from("Expected an identifier for the function definition"));
                        }
                        // got identifier
                        i += 1;
                        println!("{}, {}", i, tokens[i].clone());
                        // ' ( ' needed
                        if !matches!(parse(tokens[i].clone()), Token::OpenParentheses) {
                            return Err(String::from("Expected a '(' for the function definition"));
                        }
                        i += 1;
                        println!("{}, {}", i, tokens[i].clone());
                        // either ) or type declaration
                        if matches!(parse(tokens[i].clone()), Token::ClosingParentheses)
                        {
                            i += 1;
                            println!("{}, {}", i, tokens[i].clone());
                        } else if matches!(parse(tokens[i].clone()), Token::Int) {
                            loop {
                                if !matches!(parse(tokens[i].clone()), Token::Int) {
                                    return Err(String::from("Expected variable type"));
                                } // variable type, good
                                i += 1; // now ne need an identifier
                                println!("{}, {}", i, tokens[i].clone());

                                if !matches!(parse(tokens[i].clone()), Token::Ident(String)) {
                                    return Err(String::from("Expected an identifier "));
                                }
                                i += 1; // got our identifier
                                println!("{}, {}", i, tokens[i].clone());

                                if matches!(parse(tokens[i].clone()), Token::ClosingParentheses) {
                                    i += 1;
                                    println!("{}, {}", i, tokens[i].clone());
                                    break;
                                } else if matches!(parse(tokens[i].clone()), Token::Comma) {
                                    i += 1;
                                    println!("{}, {}", i, tokens[i].clone());
                                } else {
                                    return Err(String::from("Expected either ')' or ',' "));
                                }
                            }
                        } else {
                            return Err(String::from("Expected variable type"));
                        }
                        /*Now that we evaluated the func inent(args), evaluate the function body*/
                        if !matches!(parse(tokens[i].clone()), Token::OpeningBrace) {
                            return Err(String::from("Expected '{' before function body"));
                        }
                        i += 1;

                        // loop until closing bracket
                        while i < tokens.len() - 1 {
                            // parse expression
                            parse_body(&tokens, &mut i);
                        }
                        println!("{}, {}", i, tokens[i].clone());
                    }
                _ => { i += 1; }
            }
        }

        println!("Program compiled successfully");
        return Ok(None);
    }

    println!("{:?}", parse_statements(&tokens));
}