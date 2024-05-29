use std::iter::Peekable;
use std::process;

use crate::{Context, Expression, Scope};
use crate::token::Token;

fn create_temp(context: &mut Context) -> String {
    let identifier = format!("_temp{}", context.temp_val);
    context.temp_val += 1;
    return identifier;
}

fn parse_function<'a, T>(context: &mut Context, iterator: &mut Peekable<T>) -> String
    where T: Iterator<Item=&'a Token> {
    let mut ir_code = "".to_string();
    let mut scope = Scope {
        variables: vec![],
    };

    let function_identifier = match next_result(iterator) {
        Token::Ident(funcName) => funcName,
        _ => {
            eprintln!("Expected identifier after function token");
            process::exit(1);
        }
    };

    if context.functions.contains(&function_identifier) {
        eprintln!("The function '{}' has already been defined", function_identifier);
        process::exit(1);
    }

    context.functions.push(function_identifier.clone());

    ir_code += &format!("%func {}(", function_identifier);

    println!("Got function identifier: {}", function_identifier);
    if !check_next(iterator, Token::OpenParentheses) {
        eprintln!("Expected opening parentheses after function identifier");
        process::exit(1);
    }

    // Parse Signature
    while !check_peek(iterator, Token::ClosingParentheses) {
        if !check_next(iterator, Token::Int) {
            eprintln!("Expected int token for parameter data type");
            process::exit(1);
        }
        let parameter_identifier = match next_result(iterator) {
            Token::Ident(paramName) => paramName,
            _ => {
                eprintln!("Expected identifier after int token");
                process::exit(1);
            }
        };

        if scope.variables.contains(&parameter_identifier) {
            eprintln!("Duplicate parameter '{}' was found", parameter_identifier);
            process::exit(1);
        }
        scope.variables.push(parameter_identifier.clone());

        ir_code += &format!("%int {}", parameter_identifier);

        println!("Found parameter {}", parameter_identifier);

        if check_peek(iterator, Token::Comma) {
            iterator.next(); // we found a comma, let's progress and parse the next parameter
            ir_code += ", ";
        }
    }

    iterator.next(); // progress iterator, we found our closing parentheses
    ir_code += ")\n";

    if !check_next(iterator, Token::OpeningBrace) {
        eprintln!("Expected opening curly brace after function signature");
        process::exit(1);
    }

    // Parse Method Body
    while !check_peek(iterator, Token::Return) && !check_peek(iterator, Token::ClosingBrace) {
        ir_code += &parse(Some(&mut scope), context, iterator); // Parse anything
    }
    match next_result(iterator) {
        Token::Return => {
            println!("Got closing statement");
            let expression = parse_expression(&mut scope, context, iterator);
            ir_code += &expression.code;
            ir_code += &format!("%ret {}\n", expression.name);

            if !check_next(iterator, Token::SemiColon) {
                eprintln!("Expected opening semi colon after return statement");
                process::exit(1);
            }
            if !check_next(iterator, Token::ClosingBrace) {
                eprintln!("Expected opening closing brace after function body");
                process::exit(1);
            }
        }
        Token::ClosingBrace => {
            println!("Found closing brace, we good");
        }
        _ => {
            eprintln!("Expected opening closing brace after function body");
            process::exit(1);
        }
    }


    ir_code += "%endfunc\n";

    return ir_code;
}


fn parse_multiplication_expression<'a, T>(scope: &mut Scope, context: &mut Context, iterator: &mut Peekable<T>) -> Expression
    where T: Iterator<Item=&'a Token> {
    let mut ir_code = "".to_string();
    let mut temp_name = "".to_string();
    let mut last_operation = "";
    let mut operands = 0;

    loop {
        let mut term;
        if check_peek(iterator, Token::OpenParentheses) { // we need to handle this first...
            println!("Parsing parentheses");
            iterator.next();
            let mut expression = Expression { code: "".to_string(), name: "".to_string() };
            while !check_peek(iterator, Token::ClosingParentheses) {
                expression = parse_expression(scope, context, iterator);
                println!("Expression: {}", expression.code);
                ir_code += &expression.code;
            }
            println!("Curr term: {}", iterator.next().unwrap());
            println!("Finished parsing parentheses");
            term = Expression {
                code: ir_code.clone(),
                name: expression.name
            };
            operands += 1;
        } else {
            term = parse_term(scope, context, iterator);
            operands += 1;
        }

        if operands == 2 {
            let temp = create_temp(context);
            ir_code += &format!("%int {}\n", temp);
            ir_code += &format!("{} {}, {}, {}\n", last_operation, temp, temp_name, term.name);
            temp_name = temp;
        } else if operands >= 3 {
            ir_code += &format!("{} {}, {}, {}\n", last_operation, temp_name, temp_name, term.name);
        } else {
            temp_name = term.name;
        }

        last_operation = match peek_result(iterator) {
            Token::Multiply => "%mult",
            Token::Divide => "%div",
            Token::Modulus => "%mod",
            _ => {
                if ir_code.is_empty() { // Meaning there was nothing done beforehand
                    return Expression {
                        code: term.code,
                        name: temp_name,
                    };
                }
                break;
            }
        };
        iterator.next(); // We can progress since we didn't encounter a break and we had a valid operation
    }
    return Expression {
        name: temp_name,
        code: ir_code,
    };
}

fn parse_expression<'a, T>(scope: &mut Scope, context: &mut Context, iterator: &mut Peekable<T>) -> Expression
    where T: Iterator<Item=&'a Token> {
    let expression = parse_multiplication_expression(scope, context, iterator);
    let first_temp = &expression.name;
    let mut final_temp = first_temp.clone();
    let mut ir_code = expression.code.clone();
    println!("Got Code: \n{}name: {}\n", expression.code, expression.name);
    loop {
        let operation = match peek_result(iterator) {
            Token::Plus => {
                iterator.next(); // We can progress since we didn't encounter a break and we had a valid operation
                "%add"
            }
            Token::Subtract => {
                iterator.next();
                "%sub"
            }
            _ => { // this means we ran into a semicolon
                break;
            }
        };

        let next_expression = parse_multiplication_expression(scope, context, iterator);
        // 2 + 2 - 2 * 2
        let expr_code = &next_expression.code;
        let expr_name = &next_expression.name;
        if expr_code.is_empty() { // We were returning an empty code if it was a single operand and no operators
            // if this is the first expression we've parsed we need to use our first term
            if ir_code.is_empty() { // because the ir_code will only be added on to AFTER the first expression
                final_temp = create_temp(context); // create a new temporary
                ir_code += &format!("%int {}\n", final_temp);
                ir_code += &format!("{} {}, {}, {}\n", operation, final_temp, first_temp, expr_name);
            } else { // if we've already added on to something, meaning we're getting our third operand or something
                ir_code += &format!("{} {}, {}, {}\n", operation, final_temp, final_temp, expr_name);
            }
        } else { // Meaning we've found a multiplicative term
            ir_code += expr_code; // Add the multiplication to the IR CODE
            ir_code += &format!("{} {}, {}, {}\n", operation, final_temp, final_temp, expr_name);
        }
    }

    return Expression {
        code: ir_code, // this will be empty if we only had a single thing assigned
        name: final_temp,
    };
}

fn parse_term<'a, T>(scope: &mut Scope, context: &mut Context, iterator: &mut Peekable<T>) -> Expression
    where T: Iterator<Item=&'a Token> {
    match next_result(iterator) {
        Token::Ident(identifier) => {
            // TODO: Check for function call, if so, put into a temp var, but this can only be done after parse_expression
            let mut ir_code = "".to_string();
            let is_array = parse_array_size(iterator);
            let mut temp = identifier.clone();

            if !is_array.0 {
                if check_peek(iterator, Token::OpenParentheses) { // looks like a function call
                    temp = create_temp(context);
                    ir_code += &format!("%int {}\n", temp);

                    let mut function_call = format!("{}(", identifier);

                    if !context.functions.contains(&identifier) {
                        eprintln!("You attempted to call a function '{}' that doesn't exist", identifier);
                        process::exit(1);
                    }

                    iterator.next();
                    while !check_peek(iterator, Token::ClosingParentheses) {
                        let term = parse_expression(scope, context, iterator);
                        function_call += &format!("{}", term.name);
                        ir_code += &term.code;
                        if check_peek(iterator, Token::Comma) {
                            iterator.next();
                            function_call += ", ";
                        }
                    }
                    iterator.next(); // found closing parentheses
                    function_call += ", )";
                    ir_code += &format!("%call {}, {}\n", temp, function_call);
                    println!("Doing ir code: {}", ir_code);
                } else {
                    if !scope.variables.contains(&identifier) {
                        eprintln!("You attempted to access variable '{}' that doesn't exist", identifier);
                        process::exit(1);
                    }
                }
            } else {
                if !scope.variables.contains(&identifier) {
                    eprintln!("You attempted to index an array variable '{}' that doesn't exist", identifier);
                    process::exit(1);
                }
                temp = create_temp(context);
                ir_code += &format!("%int {}\n", temp);
                ir_code += &format!("%mov {}, [{} + {}]\n", temp, identifier, is_array.1);
            }


            return Expression {
                code: ir_code,
                name: temp,
            };
        }
        Token::Num(identifier) => {
            return Expression {
                code: "".to_string(),
                name: format!("{}", identifier),
            };
        }
        _ => {
            println!("What is it? {}", iterator.peek().unwrap());
            eprintln!("Invalid term specified");
            process::exit(1);
        }
    }
}

fn parse_array_size<'a, T>(iterator: &mut Peekable<T>) -> (bool, i32)
    where T: Iterator<Item=&'a Token> {
    if check_peek(iterator, Token::OpeningBracket) { // array size!!!
        iterator.next();
        let size = match next_result(iterator) {
            Token::Num(num) => {
                num
            },
            _ => {
                eprintln!("Expected integer size for array");
                process::exit(1);
            }
        };
        if !check_next(iterator, Token::ClosingBracket) {
            eprintln!("Expected closing bracket after array size");
            process::exit(1);
        }
        return (true, size);
    }
    return (false, -1);
}

fn parse<'a, T>(scope: Option<&mut Scope>, context: &mut Context, iterator: &mut Peekable<T>) -> String
    where T: Iterator<Item=&'a Token> {
    let mut ir_code = "".to_string();

    match peek_result(iterator) {
        Token::Func => {
            iterator.next();
            ir_code += &parse_function(context, iterator);
            println!("Broke out of function");
        }
        Token::Int => {
            iterator.next();
            let is_array = parse_array_size(iterator);
            let int_identifier = match next_result(iterator) {
                Token::Ident(funcName) => funcName,
                _ => {
                    eprintln!("Expected identifier after int token");
                    process::exit(1);
                }
            };

            if is_array.0 && is_array.1 <= 0 {
                eprintln!("Cannot create an array with size 0 or less");
                process::exit(1);
            }

            if scope.as_ref().expect("Variables should be defined within a function").variables.contains(&int_identifier) {
                eprintln!("The variable '{}' has already been declared", int_identifier);
                process::exit(1);
            }

            if !check_next(iterator, Token::SemiColon) {
                eprintln!("Expected semicolon after int variable declaration");
                process::exit(1);
            }
            if is_array.0 {
                ir_code += &format!("%int[] {}, {}\n", int_identifier, is_array.1);
            } else {
                ir_code += &format!("%int {}\n", int_identifier);
            }

            println!("Pushing {}", int_identifier);
            // if scope.is_some() {
                scope.expect("Variables should be defined within a function").variables.push(int_identifier.clone());
            // }

        }
        Token::Ident(ident) => {
            iterator.next();

            let is_array = parse_array_size(iterator);

            if !scope.as_ref().expect("Variables should be defined within a function").variables.contains(&ident) {
                eprintln!("You attempted to access a variable '{}' that doesn't exist", ident);
                process::exit(1);
            }

            if !check_next(iterator, Token::Assign) {
                eprintln!("Expected assignment operator after identifier");
                process::exit(1);
            }
            // if scope.clone().is_none() {
            //     eprintln!("Identifier assignment should be done in a function scope");
            //     process::exit(1);
            // }
            let expression = parse_expression(scope.expect("Variables should be defined within a function"), context, iterator);
            if !check_next(iterator, Token::SemiColon) {
                eprintln!("Expected semicolon after assignment");
                process::exit(1);
            }
            ir_code += &expression.code;
            if is_array.0 {
                ir_code += &format!("%mov [{} + {}], {}\n", ident, is_array.1, expression.name);
            } else {
                ir_code += &format!("%mov {}, {}\n", ident, expression.name);
            }
        }
        Token::Print => {
            println!("Parsing print");
            iterator.next();
            if !check_next(iterator, Token::OpenParentheses) {
                eprintln!("Expected opening parentheses after print token");
                process::exit(1);
            }
            // if scope.is_none() {
            //     eprintln!("Printing to stdout should be done in a function scope");
            //     process::exit(1);
            // }
            let expression = parse_expression(scope.unwrap(), context, iterator);
            if !check_next(iterator, Token::ClosingParentheses) {
                eprintln!("Expected closing parentheses after print expression");
                process::exit(1);
            }
            if !check_next(iterator, Token::SemiColon) {
                eprintln!("Expected semicolon after print");
                process::exit(1);
            }
            ir_code += &expression.code;
            ir_code += &format!("%out {}\n", expression.name);
        }
        _ => {
            iterator.next();
        }
    };

    return ir_code;
}

fn peek_is_operator<'a, T>(iterator: &mut Peekable<T>) -> bool
    where T: Iterator<Item=&'a Token> {
    return match peek_result(iterator) {
        Token::Plus | Token::Subtract | Token::Divide | Token::Multiply => true,
        _ => false
    };
}

fn next_result<'a, T>(iterator: &mut Peekable<T>) -> Token
    where T: Iterator<Item=&'a Token> {
    if iterator.peek().is_none() {
        eprintln!("Expected token, got nothing");
        process::exit(1);
    }
    let token = iterator.next().unwrap();
    return token.clone();
}

fn peek_result<'a, T>(iterator: &mut Peekable<T>) -> Token
    where T: Iterator<Item=&'a Token> {
    if iterator.peek().is_none() {
        eprintln!("Expected token, got nothing");
        process::exit(1);
    }
    let token = iterator.peek().unwrap();
    return (*token).clone();
}

fn check_next<'a, T>(iterator: &mut Peekable<T>, checked_token: Token) -> bool
    where T: Iterator<Item=&'a Token> {
    if iterator.peek().is_none() {
        return false;
    }
    let token = iterator.next().unwrap();
    return *token == checked_token;
}

fn check_peek<'a, T>(iterator: &mut Peekable<T>, checked_token: Token) -> bool
    where T: Iterator<Item=&'a Token> {
    if iterator.peek().is_none() {
        return false;
    }
    let token = iterator.peek().unwrap();
    return **token == checked_token;
}

pub fn parse_tokens(context: &mut Context, tokens: &Vec<Token>) -> String {
    let mut ir_code = "".to_string();
    let mut iter = tokens.iter().peekable();
    while iter.peek().is_some() {
        println!("Current IR CODE: \n{}", ir_code);
        let result = &format!("{}", parse(None, context, &mut iter));
        ir_code += &format!("{}", result);
    }
    return ir_code.clone();
}
