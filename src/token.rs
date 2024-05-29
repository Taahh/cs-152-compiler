use std::process;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
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
    Continue,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(ref String) => write!(f, "{}" ,String),
            Token::Num(ref num) => write!(f, "{}", num),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::LessThanEqual => write!(f, "<="),
            Token::Equality => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Plus => write!(f, "+"),
            Token::Subtract => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulus => write!(f, "%"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub fn parse_word_ending(character: char) -> Token {
    return match character {
        '(' => Token::OpenParentheses,
        ')' => Token::ClosingParentheses,
        '{' => Token::OpeningBrace,
        '}' => Token::ClosingBrace,
        '[' => Token::OpeningBracket,
        ']' => Token::ClosingBracket,
        ',' => Token::Comma,
        ';' => Token::SemiColon,
        _ => Token::NotToken
    }
}

fn parse_symbol(string: &str) -> Token{
    return match string {
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
        "continue" => Token::Continue,
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

pub fn parse_token(string: &str) -> Token {
    let mut result = parse_symbol(string);
    if result == Token::NotToken {
        let try_as_number = string.parse::<i32>();
        if try_as_number.is_err() {
            if (!string.as_bytes()[0].is_ascii_alphabetic()) {
                eprintln!("Found error with token '{}', variable identifier must begin with a letter.", string);;
                process::exit(1);
            }
            for x in string.chars() {
                if !x.is_alphanumeric() && x != '_' {
                    eprintln!("Found error with token '{}', variable identifier must contain only numbers and letters.", string);;
                    process::exit(1);
                }
            }
            return Token::Ident(string.to_string());
        } else {
            return Token::Num(try_as_number.unwrap());
        }
    }
    return result;
}