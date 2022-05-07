#[derive(Debug)]
pub enum TOKENS{
    NUMBER(f64),
    STRING(String),
    OPERATION(char),
    EOF
}
