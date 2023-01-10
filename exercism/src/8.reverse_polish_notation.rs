//URL: https://exercism.org/tracks/rust/exercises/rpn-calculator
// Reverse Polish notation (RPN)

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack :Vec<i32>= Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(x) => {
                stack.push(*x);
            },
            _ => {
                if stack.len() > 1 {
                    let right_number = stack.pop().unwrap();
                    let left_number = stack.pop().unwrap();
                    match input {
                        CalculatorInput::Add => stack.push(left_number + right_number),
                        CalculatorInput::Subtract => stack.push(left_number - right_number),
                        CalculatorInput::Multiply => stack.push(left_number * right_number),
                        _ => stack.push(left_number / right_number),
                    }
                } else {
                    return None;
                }
            }
        }
    };
    if stack.len() > 1 {
        return None;
    } else {
        stack.pop()
    }
}

fn main() {
    let input = vec![CalculatorInput::Value(4), CalculatorInput::Add];
    evaluate(&input);
}