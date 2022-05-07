#[derive(Debug)]
pub struct ast_main{
    pub name: String,
    pub nodes: Vec<ast_node>
}

impl ast_main{
    pub fn new(name: String) -> Self{
        Self{name: name, nodes: vec![]}
    }
}

#[derive(Debug)]
pub struct ast_node{
    pub operation: char,
    pub right: ast_types,
    pub left: ast_types
}
#[derive(Debug)]
pub enum ast_types{
    NUMBER(f64),
    STRING{
        value: String,
    }
}