fn main() {
  explain_variables();
  explain_datatypes();
  // For convention, use _ when it is space and all in lower case
  explain_function();
  explain_loops();
}

fn explain_loops() {
  let mut max_time = loop {
    let mut counter = 0;
    counter += 1;
    if counter == 10 {
      // return counter
      break counter;
    }
  };

  while max_time != 0 {
    max_time -= 1;
  }

  let numbers = [1, 2, 3, 4, 5, 6];
  for element in numbers.iter() {
    println!("The value: {}", element);
  }
  for number in 1..4 {
    println!("The number {}", number)
  }
}

fn explain_function() -> i32 {
  return 200;
}

fn explain_datatypes() {
  // Scalar datatypes, represents single value: Integer, Floating-point numbers, boolean, character
  // Compound datatypes, represents group of values: Tuple, Array
  let tup = ("Let's Get Rusty", 100_000);
  let (channel, sub_count) = tup;
  let sub_count = tup.1;
  // Array. Fixed size
  let error_codes = [404, 403, 200];
  let not_found = error_codes[0];
  // Create 8 values and all of the 0
  let byte = [0; 8];
}

fn explain_variables() {
  // Because all the bindings (variables) are inmutable by default, make x mutable
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // Shadowing allows you to create new variable using an existing name 
  // It gives two advangates, one we preserve mutability and the second one
  // we can change types
  let y = 10;
  println!("The value of x is: {}", y);
  let y = "ten";
  println!("The value of x is: {}", y);

  // This is a constant which cannot change, inmutable. As other variables, you cannot make it mutable
  // Const variables, must be type annotated
  // constants variables can also only be set to constant expressions so we cannot set as 
  // a return value or any value that is computed in a runtime
  // Common practise to be  in Uppercase
  const EXERCISE_NUMBER: u32 = 2;
}