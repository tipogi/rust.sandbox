#[derive(Debug)]
pub enum ForthWord {
    Add,
    Rest,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Number,
    Colon,
    SemiColon
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn parse_builtin(input: &str) -> Result<ForthWord, Error> {
    let number = input.parse::<i32>();
    if number.is_ok() {
        return Ok(ForthWord::Number);
    } else {
        // Create string literal
        match input.to_lowercase().as_str() {
            "+" => Ok(ForthWord::Add),
            "-" => Ok(ForthWord::Rest),
            "*" => Ok(ForthWord::Mul),
            "/" => Ok(ForthWord::Div),
            "dup" => Ok(ForthWord::Dup),
            "drop" => Ok(ForthWord::Drop),
            "swap" => Ok(ForthWord::Swap),
            "over" => Ok(ForthWord::Over),
            ":" => Ok(ForthWord::Colon),
            ";" => Ok(ForthWord::SemiColon),
            _ => Err(Error::UnknownWord)
        }
    }
    
}

fn main() {
    let word = "+";
    parse_builtin(word);
}