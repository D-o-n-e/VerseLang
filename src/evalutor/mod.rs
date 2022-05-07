use crate::{ast::{ast_main, ast_types}, tokens::TOKENS};

pub fn eval(ast: ast_main){
    let mut result = 0.0;
    for node in ast.nodes{
        match node.operation{
            '/' => {
                if result == 0.0{
                    result = get_value_from_tok(node.left) / get_value_from_tok(node.right);
                } else {
                    result  /= get_value_from_tok(node.right);
                }
            },
            '+' => {
                if result == 0.0{
                    result = get_value_from_tok(node.left) + get_value_from_tok(node.right);
                } else {
                    result  += get_value_from_tok(node.right);
                }
            },
            '-' => {
                if result == 0.0{
                    result = get_value_from_tok(node.left) - get_value_from_tok(node.right);
                } else {
                    result -= get_value_from_tok(node.right);
                }
            },
            '*' => {
                if result == 0.0{
                    result = get_value_from_tok(node.left) * get_value_from_tok(node.right);
                } else {
                    result  *= get_value_from_tok(node.right);
                }
            },
            _ => {}
        }
    }
    
    println!("{}", result);
    
}

fn get_value_from_tok(token: ast_types) -> f64{
    match token{
        ast_types::NUMBER(value) => value,
        _ => 0.0
    }
}