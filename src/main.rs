extern crate VerseLang;
use VerseLang::lexer;
use VerseLang::parser;
use VerseLang::evalutor;

fn main(){
    let tokens = lexer::tokenize("50+20-20/2".to_string());
    let ast = parser::parse(tokens);
    evalutor::eval(ast);

}