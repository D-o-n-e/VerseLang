use crate::ast::*;
use crate::tokens::TOKENS;

pub fn parse(tokens: Vec<TOKENS>) -> ast_main{
    let mut ast_tree = ast_main::new("PROCESS".to_string());
    let mut previous_tok: TOKENS = TOKENS::EOF;
    let mut next_tok: &TOKENS;
    for (i, token) in tokens.iter().enumerate(){
        if tokens.len() > i+1{
            next_tok = &tokens[i+1];
        } else {
            next_tok = &TOKENS::EOF;
        }
        match token {
            TOKENS::OPERATION(value) => {
                let node: ast_node = ast_node{
                    operation: *value, 
                    left: ast_types::NUMBER(get_value_from_tok(&previous_tok).unwrap()),
                    right: ast_types::NUMBER(get_value_from_tok(next_tok).unwrap())
                };
                ast_tree.nodes.append(&mut vec![node]);
            }
            _ => {}
        }
        previous_tok = match token{
            TOKENS::NUMBER(value) => TOKENS::NUMBER(*value),
            TOKENS::OPERATION(value) => TOKENS::OPERATION(*value),
            _ => TOKENS::EOF
        };
        
    }   
    ast_tree
}

fn get_value_from_tok(token: &TOKENS) -> Option<f64>{
    match token{
        TOKENS::NUMBER(value) => Some(*value),
        _ => None
    }
}