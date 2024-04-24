use std::{env, fs, process};
use std::num::ParseIntError;

mod interpreter;

#[derive(Debug, Clone)]
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
    NotEqual
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

    for c in code.chars() {
        buffer.push(c);
        if c == '#' // dump buffer
        {
            is_comment = true;
        }
        if c == '\n'
        {
            // comment identification
            if is_comment == true
            {
                print!("Filtered comment: ");
                print!("{}", buffer);
                is_comment = false;
            } else {
                for ch in buffer.chars()
                {
                    if ch == ' ' || ch == '(' || ch == ')' || ch == '}' || ch == '{' || ch == '[' || ch == ']' || ch == ';'
                    {
                        if (word_buffer.len() > 0) {
                            word_buffer = word_buffer.to_lowercase();
                            token_types.push(parse(word_buffer.clone()));

                            tokens.push(word_buffer.clone());
                        }

                        if (ch != ' ') {
                            token_types.push(parse(ch.to_string().clone()));
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
    }

    fn parse(s: String) -> Token {
        let mut arr: Vec<Token> = vec![];
        if (s.eq("+")) {
            return Token::Plus;
        } else if (s.eq("-")) {
            return Token::Subtract;
        } else if (s.eq("*")) {
            return Token::Multiply;
        } else if (s.eq("/")) {
            return Token::Divide;
        } else if (s.eq("%")) {
            return Token::Modulus;
        } else if (s.eq("=")) {
            return Token::Assign;
        } else if (s.eq("if")) {
            return Token::If;
        }  else if (s.eq("else")) {
            return Token::Else;
        } else if (s.eq("while")) {
            return Token::While;
        }  else if (s.eq("break")) {
            return Token::Break;
        } else if (s.eq("read")) {
            return Token::Read;
        } else if (s.eq("func")) {
            return Token::Func;
        } else if (s.eq("return")) {
            return Token::Return;
        } else if (s.eq("int")) {
            return Token::Int;
        } else if (s.eq("(")) {
            return Token::OpenParentheses;
        }  else if (s.eq(")")) {
            return Token::ClosingParentheses;
        }  else if (s.eq("{")) {
            return Token::OpeningBrace;
        }  else if (s.eq("}")) {
            return Token::ClosingBrace;
        }  else if (s.eq("[")) {
            return Token::OpeningBracket;
        }  else if (s.eq("]")) {
            return Token::ClosingBracket;
        }  else if (s.eq(",")) {
            return Token::Comma;
        }   else if (s.eq(";")) {
            return Token::SemiColon;
        }  else if (s.eq("==")) {
            return Token::Equality;
        }  else if (s.eq("!=")) {
            return Token::NotEqual;
        } else if (s.eq("print")) {
            return Token::Print;
        }  else if (s.eq(">")) {
            return Token::GreaterThan;
        }  else if (s.eq("<")) {
            return Token::LessThan;
        }  else if (s.eq(">=")) {
            return Token::GreaterThanEqual;
        }  else if (s.eq("<=")) {
            return Token::LessThanEqual;
        }
        else {
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
            }
        }

        return Token::NotToken;
    }
    // output
    println!("{:?}", tokens);
    println!("{:?}", token_types);
}
