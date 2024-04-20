use std::{env, fs};

mod interpreter;

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

    for c in code.chars(){

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
                        if ch == ' ' || ch == '(' || ch == ')' || ch == '}' || ch == '{' || ch == ';'
                        {
                            // println!("{}",word_buffer);
                            if word_buffer.len() > 0
                            {  
                                tokens.push(word_buffer.clone());
                            }
                            if ch != ' ' && ch != '\n'
                            {
                                // println!("{}", ch);
                                tokens.push(ch.clone().to_string())
                            }
                            word_buffer.clear();
                        } else {
                            if ch != '\n'
                            {
                                word_buffer.push(ch);
                            }
                        }
                    }
                
                    
            };
            buffer.clear();
        }
    }
    // output
    println!("{:?}", tokens);
}
