use crate::tokens::TOKENS;

pub fn tokenize(code: String) -> Vec<TOKENS>{
    let mut tokens: Vec<TOKENS> = vec![];
    let mut opened_number: bool = false;
    let mut opened_string: bool = false;
    let mut string: String = "".to_owned();
    let mut number: String = "".to_owned();

    for (i, ch) in code.chars().enumerate() {
        if ch == ' '{
            continue;
        }
        if ch.is_numeric() || ch == '.'{
            opened_number = true;
            number.push_str(&ch.to_string());
            
        }else if !is_operation(ch){
            opened_string = true;
            string.push_str(&ch.to_string());
            
        }

        // Tokenize numbers
        if !ch.is_numeric() && !(ch == '.') || code.len() - i == 1  {
            if opened_number{
                tokens.append(&mut vec!(TOKENS::NUMBER(number.parse::<f64>().unwrap())));
                number = "".to_string(); 
            }
            opened_number = false
        }

        // Tokenize Strings
        if ch.is_numeric() || code.len() - i == 1 {
            if opened_string {
                tokens.append(&mut vec!(TOKENS::STRING(string)));
                string = "".to_string()
            }      
            opened_string = false; 
        }

        // Tokenize Operators
        if is_operation(ch) && !ch.is_numeric(){
            tokens.append(&mut vec![TOKENS::OPERATION(ch)]);         
        }       
    }
    tokens
}

fn is_operation(ch: char) -> bool{
    ch == '+' || ch == '-' || ch == '/' || ch == '*'
}