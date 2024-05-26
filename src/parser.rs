use std::fmt::format;
use std::process;

use crate::token::Token;

static mut IR_CODE: String = String::new();
static mut VARIABLE_STACK: Vec<String> = vec![];
static mut VAR_NUM: i64 = 1;
static mut PRINTING: bool = false;

pub fn handle_function(tokens: &Vec<Token>, i: &mut usize) {
    if !matches!(&tokens[*i], Token::Ident(ref String)) {
        eprintln!("Expected name for function");
        process::exit(1);
    }

    let mut func_name = &tokens[*i];

    //println!("{:?}", tokens[*i]);
    *i += 1;

    if !matches!(&tokens[*i], Token::OpenParentheses) {
        eprintln!("Expected '(' for function signature");
        process::exit(1);
    }
    unsafe { IR_CODE.push_str(&format!("%func {}(", func_name)); }
    //println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingParentheses) {
        if !matches!(&tokens[*i], Token::Int) {
            eprintln!("Expected data type 'int' for parameter");
            process::exit(1);
        }
        //println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(&tokens[*i], Token::Ident(ref String)) {
            eprintln!("Expected identifier for parameter");
            process::exit(1);
        }

        unsafe { IR_CODE.push_str(&format!("%int {}", tokens[*i])); }

        //println!("{:?}", tokens[*i]);
        *i += 1;

        if matches!(&tokens[*i], Token::Comma) {
            //println!("{:?}", tokens[*i]);
            unsafe { IR_CODE.push_str(", "); }
            *i += 1;
            continue;
        } else if matches!(&tokens[*i+1], Token::ClosingParentheses) {
            break;
        }
    }
    unsafe { IR_CODE.push_str(")\n"); }
    //println!("{:?}", tokens[*i]);
    *i += 1;

    // done with the func(arga, argb) stuff;
    // if (arg_c == 0){
    //     unsafe {
    //         IR_CODE.push_str(&format!("%func {}()\n", func_name));
    //     }
    // } else {
    //     unsafe {
    //         IR_CODE.push_str(&format!("%func {}({}, {})\n", func_name, arg_a, arg_b));
    //     }
    // }

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after function signature");
        process::exit(1);
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingBrace) {
        handle(tokens, i);

        if matches!(&tokens[*i], Token::Return) {
            //println!("{:?}", tokens[*i]);
            if format!("{}", func_name).clone() == String::from("main") {
                eprintln!("main cannot return anything!");
                process::exit(1);
            }
            *i += 1;
            let return_var = create_temp();
            unsafe { VARIABLE_STACK.push(return_var.clone()); }
            handle_declaration_contents(tokens, i, Token::SemiColon);
             unsafe {
                 IR_CODE.push_str(&format!("%ret {}\n", return_var));
             }
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        }
    }
    if !matches!(&tokens[*i], Token::ClosingBrace) {
        eprintln!("Expected '}}' after function body");
        process::exit(1);
    }
    unsafe {
        IR_CODE.push_str("%endfunc\n\n");
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;
}

fn create_temp() -> String {
    unsafe {
        let identifer = format!("_temp{}", VAR_NUM);
        IR_CODE.push_str(&format!("%int {}\n", identifer));
        VAR_NUM += 1;
        return identifer;
    }
}

fn handle_condition(tokens: &Vec<Token>, i: &mut usize) {
    if matches!(&tokens[*i], Token::Ident(String)) || matches!(&tokens[*i], Token::Int) {
        //println!("{:?}", tokens[*i]);
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
        //println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32))
        {
            eprintln!("Error : Expected an identifier or number for the inequality.");
            process::exit(1);
        }
        //println!("{:?}", tokens[*i]);
        *i += 1;
    }
}

fn handle_loop(tokens: &Vec<Token>, i: &mut usize) {
    //println!("{:?}", tokens[*i]);
    *i += 1;

    handle_condition(tokens, i);

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after loop condition");
        process::exit(1);
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingBrace) {
        handle(tokens, i);

        if matches!(&tokens[*i], Token::Return) {

            //println!("{:?}", tokens[*i]);
            *i += 1;
            handle_declaration_contents(tokens, i, Token::SemiColon);
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        } else if matches!(&tokens[*i], Token::Break) {
            //println!("{:?}", tokens[*i]);
            *i += 1;
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;

            if !matches!(&tokens[*i], Token::ClosingBrace) {
                eprintln!("Expected '}}' to close while loop");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;
            break;
        }
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;
}

fn handle_if(tokens: &Vec<Token>, i: &mut usize, is_else: bool) {
    //println!("{:?}", tokens[*i]);
    *i += 1;

    if !is_else {
        handle_condition(tokens, i);
    }

    if !matches!(&tokens[*i], Token::OpeningBrace) {
        eprintln!("Expected '{{' after loop condition");
        process::exit(1);
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;

    while !matches!(&tokens[*i], Token::ClosingBrace) {
        handle(tokens, i);

        if matches!(&tokens[*i], Token::Return) {
            //println!("{:?}", tokens[*i]);
            *i += 1;
            handle_declaration_contents(tokens, i, Token::SemiColon);
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            break;
        } else if matches!(&tokens[*i], Token::Break) {
            //println!("{:?}", tokens[*i]);
            *i += 1;
            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expected ';' after return statement");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;

            if !matches!(&tokens[*i], Token::ClosingBrace) {
                eprintln!("Expected '}}' to close if statement");
                process::exit(1);
            }
            break;
        }
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;
}

fn handle_assignment(tokens: &Vec<Token>, i: &mut usize) {
    if !matches!(&tokens[*i], Token::Assign) {
        eprintln!("'=' required for variable assignment.");
        process::exit(1);
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;
    handle_declaration_contents(tokens, i, Token::SemiColon);
    if !matches!(&tokens[*i], Token::SemiColon) {
        eprintln!("Expected ';' after assignment");
        process::exit(1);
    }
    //println!("{:?}", tokens[*i]);
    *i += 1;
}

// If array, subtract i by 4
// If not array, subtract i by 1
fn identifier_or_number<'a>(tokens: &Vec<Token>, i: &mut usize) -> (Token, i32) {
    if !matches!(&tokens[*i], Token::Ident(String)) && !matches!(&tokens[*i], Token::Num(i32)) {
        eprintln!("Expected to pass in parameter value");
        process::exit(1);
    }
    let mut identifier = tokens[*i].clone();
    *i += 1;
    let num = handle_array_size(tokens, i);
    return (identifier, num);
}

fn arithmetic_or_comparison(tokens: &Vec<Token>, i: &mut usize) -> String {
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
    let token = &tokens[*i];
    //println!("{:?}", tokens[*i]);
    *i += 1;

    return match token {
        Token::Plus => "%add".to_string(),
        Token::Subtract => "%sub".to_string(),
        Token::Multiply => "%mult".to_string(),
        Token::Divide => "%div".to_string(),
        Token::Modulus => "%mod".to_string(),
        Token::Equality => "%eq".to_string(),
        Token::NotEqual => "%neq".to_string(),
        Token::LessThanEqual => "%le".to_string(),
        Token::LessThan => "%lt".to_string(),
        Token::GreaterThanEqual => "%ge".to_string(),
        Token::GreaterThan => "%gt".to_string(),
        _ => "".to_string(), // Handle other cases if needed
    };
}

fn handle_array_size(tokens: &Vec<Token>, i: &mut usize) -> i32 {
    if matches!(&tokens[*i], Token::OpeningBracket) {
        //println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(tokens[*i], Token::Num(i32)) {
            eprintln!("Expected fixed size for array");
            process::exit(1);
        }
        let size = format!("{}", tokens[*i]).parse::<i32>().unwrap();
        //println!("{:?}", tokens[*i]);
        *i += 1;

        if !matches!(tokens[*i], Token::ClosingBracket) {
            eprintln!("Expected ']' to close array size init");
            process::exit(1);
        }
        //println!("{:?}", tokens[*i]);
        *i += 1;
        return size;
    }
    return -1;
}

fn handle_declaration_contents(tokens: &Vec<Token>, i: &mut usize, end_token: Token) {
    let mut balanced_parentheses: Vec<&Token> = vec![];
    let mut last_temp: String = "".to_string();
    let mut last_operation: String = "".to_string();
    while tokens[*i] != end_token { // go until end of return statement
        // //println!("hellooo");
        if matches!(tokens[*i], Token::OpenParentheses) {
            balanced_parentheses.push(&tokens[*i]);
            //println!("{:?}", tokens[*i]);
            *i += 1;
        }
        let ident1 = identifier_or_number(tokens, i);
        let mut function = false;
        let mut function_call = format!("{}(", ident1.0);

        if matches!(&tokens[*i-1], Token::Ident(String)) {
            if matches!(&tokens[*i], Token::OpenParentheses) { // function call
                //println!("{:?}", tokens[*i]);
                *i += 1;
                let mut adding_more = false;
                while !matches!(&tokens[*i], Token::ClosingParentheses) {
                    if adding_more {
                        arithmetic_or_comparison(tokens, i);
                        function_call.push_str(&format!("{}", tokens[*i-1]));
                    }
                    let fun_iden = identifier_or_number(tokens, i);
                    if matches!(&tokens[*i], Token::ClosingParentheses) {
                        function_call.push_str(&format!("{}", fun_iden.0));
                        break;
                    } else if matches!(&tokens[*i], Token::Comma) {
                        function_call.push_str(&format!("{}", fun_iden.0));
                        function_call.push_str(", ");
                        *i += 1;
                        continue;
                    } else {
                        let temp = create_temp();
                        let operation = arithmetic_or_comparison(tokens, i);
                        let iden = identifier_or_number(tokens, i);
                        unsafe {
                            IR_CODE.push_str(&format!("{} {}, {}, {}\n", operation, temp, fun_iden.0, iden.0));
                        }
                        function_call.push_str(&format!("{}", temp));
                        adding_more = true;
                    }
                }
                function_call.push_str(", )");
                function = true;
                println!("{}", function_call);
                //println!("{:?}", tokens[*i]);
                *i += 1;
            }
        }
        if tokens[*i] == end_token {
            if last_temp.is_empty() {
                unsafe {
                    if PRINTING {
                        let mut identifier = format!("{}", ident1.0);
                        if ident1.1 != -1 {
                            identifier = format!("_temp{}", VAR_NUM);
                            IR_CODE.push_str(&format!("%int {}\n", identifier));
                            IR_CODE.push_str(&format!("%mov {}, [{} + {}]\n", identifier, ident1.0, ident1.1));

                            VAR_NUM += 1;
                        }
                        IR_CODE.push_str(&format!("%out {}\n", identifier));
                    } else {
                        if function {
                            let temp = create_temp();
                            IR_CODE.push_str(&format!("%call {}, {}\n", temp, function_call));
                            IR_CODE.push_str(&format!("%mov {}, {}\n", VARIABLE_STACK[VARIABLE_STACK.len() - 1], temp));
                        } else {
                            IR_CODE.push_str(&format!("%mov {}, {}\n", VARIABLE_STACK[VARIABLE_STACK.len() - 1], ident1.0));
                        }
                    }
                    return;
                }
            } else {
                unsafe {
                    let new_temp = format!("_temp{}", VAR_NUM);
                    IR_CODE.push_str(&format!("%int {}\n", new_temp));
                    VAR_NUM += 1;
                    IR_CODE.push_str(&format!("{} {}, {}, {}\n", last_operation, new_temp, last_temp, ident1.0));
                    last_temp = new_temp;
                }
            }
            break;
        }

        let op = arithmetic_or_comparison(tokens, i);
        let mut ident2 = identifier_or_number(tokens, i);

        if matches!(&tokens[*i-1], Token::Ident(String)) {
            if matches!(&tokens[*i], Token::OpenParentheses) { // function call
                //println!("{:?}", tokens[*i]);
                *i += 1;
                while !matches!(&tokens[*i], Token::ClosingParentheses) {
                    identifier_or_number(tokens, i);
                    if matches!(&tokens[*i], Token::ClosingParentheses) {
                        break;
                    } else if matches!(&tokens[*i], Token::Comma) {
                        //println!("{:?}", tokens[*i]);
                        *i += 1;
                        continue;
                    } else {
                        arithmetic_or_comparison(tokens, i);
                        identifier_or_number(tokens, i);
                    }
                }
                //println!("{:?}", tokens[*i]);
                *i += 1;
            }
        }
        if tokens[*i] == end_token {

            unsafe {
                let mut identifier = format!("{}", ident1.0);
                let mut identifier2 = format!("{}", ident2.0);
                if ident1.1 != -1 {
                    identifier = format!("_temp{}", VAR_NUM);
                    IR_CODE.push_str(&format!("%int {}\n", identifier));
                    IR_CODE.push_str(&format!("%mov {}, [{} + {}]\n", identifier, ident1.0, ident1.1));

                    VAR_NUM += 1;
                }

                if ident2.1 != -1 {
                    identifier2 = format!("_temp{}", VAR_NUM);
                    IR_CODE.push_str(&format!("%int {}\n", identifier2));
                    IR_CODE.push_str(&format!("%mov {}, [{} + {}]\n", identifier2, ident2.0, ident2.1));

                    VAR_NUM += 1;
                }

                last_temp = format!("_temp{}", VAR_NUM);
                IR_CODE.push_str(&format!("%int {}\n", last_temp));
                IR_CODE.push_str(&format!("{} {}, {}, {}\n", op, last_temp, identifier, identifier2));
                VAR_NUM += 1;
            }

            break;
        } else {
            if matches!(tokens[*i], Token::ClosingParentheses) {
                balanced_parentheses.pop();
                //println!("{:?}", tokens[*i]);
                *i += 1;
            }
            unsafe {
                // we have to parse the expression here and then return it into ident2
                    let mut breaknow = 0;
                while tokens[*i] != Token::SemiColon {
                    let mut dumy_ident = format!("_temp{}", VAR_NUM);

                    // first expr is ident2.0
                    let opr = arithmetic_or_comparison(tokens, i);
                    let ident3 = identifier_or_number(tokens, i);
                    IR_CODE.push_str(&format!("%int {}\n", dumy_ident));
                    IR_CODE.push_str(&format!("{} {}, {}, {}\n", opr, dumy_ident, ident2.0, ident3.0));
                    ident2.0 = Token::Ident(dumy_ident);
                    VAR_NUM += 1;
                    breaknow = 1; // we need to break if we do this.
                }
                let mut identifier = format!("{}", ident1.0);
                let mut identifier2 = format!("{}", ident2.0);
                if ident1.1 != -1 {
                    identifier = format!("_temp{}", VAR_NUM);
                    IR_CODE.push_str(&format!("%int {}\n", identifier));
                    IR_CODE.push_str(&format!("%mov {}, [{} + {}]\n", identifier, ident1.0, ident1.1));

                    VAR_NUM += 1;
                }

                if ident2.1 != -1 {
                    identifier2 = format!("_temp{}", VAR_NUM);
                    IR_CODE.push_str(&format!("%int {}\n", identifier2));
                    IR_CODE.push_str(&format!("%mov {}, [{} + {}]\n", identifier2, ident2.0, ident2.1));

                    VAR_NUM += 1;
                }

                last_temp = format!("_temp{}", VAR_NUM);
                IR_CODE.push_str(&format!("%int {}\n", last_temp));
                IR_CODE.push_str(&format!("{} {}, {}, {}\n", op, last_temp, identifier, identifier2));
                VAR_NUM += 1;
                if breaknow == 1 {break;}
            }
            last_operation = arithmetic_or_comparison(tokens, i);
        }
    }

    unsafe {
        IR_CODE.push_str(&format!("%mov {}, {}\n", VARIABLE_STACK[VARIABLE_STACK.len() - 1], last_temp));
    }

}

fn handle_declaration(tokens: &Vec<Token>, i: &mut usize) {
    //println!("{:?}", tokens[*i]);
    *i += 1;

    let num = handle_array_size(tokens, i);

    if !matches!(&tokens[*i], Token::Ident(String)) {
        eprintln!("Expected identifier for int variable.");
        process::exit(1);
    }
    let identifier_name = format!("{}", tokens[*i]);
    //println!("{:?}", tokens[*i]);
    *i += 1;

    if num != -1 {
        unsafe { IR_CODE.push_str(&format!("%int[] {}, {}\n", identifier_name, num)); }
    } else {
        unsafe { IR_CODE.push_str(&format!("%int {}\n", identifier_name)); }
    }

    if matches!(&tokens[*i], Token::SemiColon) {
        //println!("{:?}", tokens[*i]);
        *i += 1;
        return;
    } else if matches!(&tokens[*i], Token::Assign) {
        //println!("{:?}", tokens[*i]);
        *i += 1;
        //println!("{:?}", tokens[*i]);
        handle_declaration_contents(tokens, i, Token::SemiColon);
    }
}

static mut func_id: String = String::new();
static mut last_token: Token = Token::NotToken;

fn handle(tokens: &Vec<Token>, i: &mut usize) {
    let current_token = &tokens[*i];
    match current_token {
        Token::Func => {
            //println!("{:?}", current_token);
            *i += 1;
            handle_function(tokens, i);
        }
        Token::While => { // while loop
            handle_loop(tokens, i);
        }
        Token::If => { // if statement
            handle_if(tokens, i, false);
        }
        Token::Else => {
            handle_if(tokens, i, true);
        }
        Token::Ident(s) => { // identifier
            //println!("{:?}", current_token);
            *i += 1;

            let num = handle_array_size(tokens, i);
            if num == -1 {
                unsafe { VARIABLE_STACK.push(format!("{}", s)); }
            } else {
                unsafe { VARIABLE_STACK.push(format!("[{} + {}]", s, num)); }
            }
            handle_assignment(tokens, i);
        }
        Token::Print => { // print statement
            //println!("{:?}", current_token);
            *i += 1;
            if !matches!(&tokens[*i], Token::OpenParentheses) {
                eprintln!("Expected '(' after print call");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;

            unsafe {
                PRINTING = true
            }

            handle_declaration_contents(tokens, i, Token::ClosingParentheses);

            unsafe {
                PRINTING = false;
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;

            if !matches!(&tokens[*i], Token::SemiColon) {
                eprintln!("Expeced ';' after print call");
                process::exit(1);
            }
            //println!("{:?}", tokens[*i]);
            *i += 1;
        }
        Token::Int => { // int variable declaration
            handle_declaration(tokens, i);
        }
        _ => {}
    }
}

pub unsafe fn parse_tokens(tokens: &Vec<Token>) -> String {
    let mut i = 0;
    while i < tokens.len() {
        handle(tokens, &mut i);
    }
    return IR_CODE.clone();
}