extern crate VerseLang;
use VerseLang::lexer;
use VerseLang::parser;
use VerseLang::evalutor;
use std::io::{stdin,stdout,Write};

fn main(){
    print!(">> ");
    let _ = stdout().flush();
    let mut input: String = String::new();  
    stdin().read_line(&mut input);
    let tokens = lexer::tokenize(input);
    let ast = parser::parse(tokens);
    evalutor::eval(ast);

}