// ## Instructions ##
// Implement an evaluator for a very simple subset of Forth.
// Forth is a stack-based programming language. Implement a very basic evaluator for a small subset of Forth.
// Your evaluator has to support the following words:

//     +, -, *, / (integer arithmetic)
//     DUP, DROP, SWAP, OVER (stack manipulation)

// Your evaluator also has to support defining new words using the customary syntax: : word-name definition ;.
// To keep things simple the only data type you need to support is signed integers of at least 16 bits size.
// You should use the following rules for the syntax: a number is a sequence of one or more (ASCII) digits, 
// a word is a sequence of one or more letters, digits, symbols or punctuation that is not a number. 
// (Forth probably uses slightly different rules, but this is close enough.)

// Words are case-insensitive.

use std::collections::HashMap;

pub type Value = i32;


pub enum ForthTypes {
    Value
}

#[derive(Debug, PartialEq)]
pub enum ForthWord {
    Init,
    Add,
    Rest,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Number(i32),
    Colon,
    SemiColon,
    // Usually would be a string chain and might return that type 
    // when it is activated new_operation boolean
    NewExpr,
    CustomExpr
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Default)]
pub struct Forth {
    stack: Vec<i32>,
    new_op: HashMap<String, Vec<String>>
}

fn parse_builtin(input: &str, new_operation: bool) -> Result<ForthWord, Error> {
    // Create string literal
    // check if that keyword is in the hashmap
    let operation = match input.to_lowercase().as_str() {
        "+" => Ok(ForthWord::Add),
        "-" => Ok(ForthWord::Rest),
        "*" => Ok(ForthWord::Mul),
        "/" => Ok(ForthWord::Div),
        "dup" => Ok(ForthWord::Dup),
        "drop" => Ok(ForthWord::Drop),
        "swap" => Ok(ForthWord::Swap),
        "over" => Ok(ForthWord::Over),
        ":" => return Ok(ForthWord::Colon),
        ";" => return Ok(ForthWord::SemiColon),
        _ => if new_operation { Ok(ForthWord::NewExpr) } else { Err(Error::UnknownWord) }
    };
    if new_operation && operation.is_ok() {
        return Ok(ForthWord::NewExpr);
    }
    return operation;
}

impl Forth {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        let forth_sequence = input.split(" ");
        let mut new_operation = false;
        let mut new_op_vec: Vec<String> = Vec::new();
        let mut res: Result<(), Error> = Ok(());
        // Loop the the forth sequence
        for word in forth_sequence {
            // Initialise the operator
            let mut operation = Ok(ForthWord::Init);
            // If it is new operation, we are creating a new operation for the forth machine
            if !new_operation { operation = self.number_or_custom_op(word); }
            // The operation is not a number or custom expresion
            if operation == Ok(ForthWord::Init) { operation = parse_builtin(word, new_operation); }
            res = match operation {
                Ok(ForthWord::Add) => self.add(),
                Ok(ForthWord::Rest) => self.rest(),
                Ok(ForthWord::Mul) => self.mul(),
                Ok(ForthWord::Div) => self.div(),
                Ok(ForthWord::Dup) => self.dup(),
                Ok(ForthWord::Drop) => self.drop(),
                Ok(ForthWord::Swap) => self.swap(),
                Ok(ForthWord::Over) => self.over(),
                Ok(ForthWord::Colon) => {
                    new_operation = true;
                    Ok(())
                },
                Ok(ForthWord::SemiColon) => {
                    new_operation = false;
                    let result = self.create_new_operation(&mut new_op_vec);
                    if result.is_err() { return result; }
                    // Reset the vector
                    new_op_vec.clear();
                    Ok(())
                },
                Ok(ForthWord::NewExpr) => {
                    new_op_vec.push(String::from(word));
                    Ok(())
                },
                Ok(ForthWord::Number(i)) => {
                    self.push(i);
                    Ok(())
                },
                Ok(ForthWord::CustomExpr) => return self.exec_custom_expr(word),
                Err(error) => return Err(error),
                _ => return Err(Error::UnknownWord)
            };
        }
        if res.is_err() {
            return res;
        }
        if new_operation {
            return Err(Error::InvalidWord);
        }
        Ok(())
    }

    fn exec_custom_expr(&mut self, word: &str) -> Result<(), Error>{
        let custom_expresion = self.new_op.get(word).unwrap();
        let mut new_op = String::new();
        for op in custom_expresion.iter() {
            new_op.push_str(op);
            new_op.push_str(" ");
        }
        self.eval(&new_op)
    }

    fn number_or_custom_op(&self, input: &str) -> Result<ForthWord, Error>{
        let number = input.parse::<i32>();
        if number.is_ok() {
            return Ok(ForthWord::Number(number.unwrap()));
        } else if self.new_op.contains_key(input) {
            return Ok(ForthWord::CustomExpr)
        } else {
            return Ok(ForthWord::Init)
        }
    }

    fn create_new_operation(&mut self, new_op_vec: &mut Vec<String>) -> Result<(), Error> {
        if new_op_vec.len() < 2 {
            return Err(Error::InvalidWord);
        } else {
            let new_op = new_op_vec.remove(0);
            self.new_op.insert(
                new_op.to_lowercase(), 
                // Create new vector and get the ownership of the value
                new_op_vec.to_vec()
            );
        }
        Ok(())
    }

    // Error control in operations
    // Add colom and semi colum new op

    // Arithmetic Operations
    fn add(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        let sum = self.pop() + self.pop();
        self.stack.push(sum);
        Ok(())
    }

    fn rest(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        let right = self.pop();
        let rest = self.pop() - right;
        self.push(rest);
        Ok(())
    }

    fn mul(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        let sum = self.pop() * self.pop();
        self.stack.push(sum);
        Ok(())
    }

    fn div(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        // if we divide with 0. Error control
        let divident= self.pop();
        let sum = self.pop() / divident;
        self.stack.push(sum);
        Ok(())
    }

    // Stack Manipulation
    fn dup(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        let number = self.pop();
        self.push(number);
        self.push(number);
        Ok(())
    }

    fn drop(&mut self) -> Result<(), Error> {
        if self.stack.len() < 1 { return Err(Error::StackUnderflow); }
        self.pop();
        Ok(())
    }

    fn swap(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        let first = self.pop();
        let second = self.pop();
        self.push(second);
        self.push(first);
        Ok(())
    }

    fn over(&mut self) -> Result<(), Error> {
        if self.stack.len() < 2 { return Err(Error::StackUnderflow); }
        let over_number = self.stack[self.stack.len() - 1];
        self.push(over_number);
        Ok(())
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, number: i32) {
        self.stack.push(number)
    }
}

fn main() {

    let task_a= "1 2 + 1 2 + +";
    let task_b = "3 2 * 4 DUP";
    let task_c = ": 3 2 +";
    let task_d = "3 2 * 4 SWAp";
    let task_e = ": dup-twice dup dup ;";
    let task_e2 = "1 dup-twice";
    let task_ = ": EL 1 2 + ; : el 2 * 3 ; 1 2 + el +";
    let mut machine_one = Forth::new();
    println!("{:#?}", machine_one.eval(task_e)); 
    println!("{:#?}", machine_one.new_op); 
    println!("{:#?}", machine_one.eval(task_e2)); 
    println!("{:?}", machine_one.stack())
}