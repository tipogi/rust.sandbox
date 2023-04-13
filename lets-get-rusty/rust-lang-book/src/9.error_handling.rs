use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

// WHEN TO USE ERROR PROPAGATION or PANIC
// By default use Result enum for error propagation. This prevents the program crashing
// and like this the caller can decide which is the best way to handle that error 

// We should only use panic!, in exceptional circunstances as the recover of that error is
// not possible and the program cannot continue from that state

// We can use unwrap or expect in prototype code when we do not want to do error handling
// Then when we pass the prototyping phase, we can do all the error handling identifying 
// all the unwrap and expect functions

// Lastly, we might want to use unwrap or expect, when we know that the function will
// succeed

// Check: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation

fn main() {
  // Panic macro inmidiately stop and exit the program
  //panic!("crash and burn")
  backtrace_a();
  recoverable_errors();
  recoverable_errors_w_clousures();
  unwrap_fn();
  error_propagation();
  error_propagation_final();
}

// We will chain the results, simplyfied version
fn error_propagation_final() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("rust.rs")?.read_to_string(&mut s)?;
  Ok(s)
}

// When we want to pass the error to the caller. Not directly panic in the function
// This gives more control to the caller and what to do with that error
fn error_propagation() -> Result<String, io::Error> {
  /*let file = File::open("rust.rs");
  let mut file = match file {
    Ok((file)) => file,
    Err(e) => return Err(e)
  };*/
  // Equal as above code. In Error case, the function will finish and it will return
  // the error to the caller. This processing happen because we add '?' symbol at the end
  let mut file = File::open("rust.rs")?;

  let mut s = String::new();

  /*match file.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e)
  }*/

  // Same as above but if we do not have error return String
  // Ok is Result type
  file.read_to_string(&mut s)?;
  Ok(s)
}

fn unwrap_fn() {
  // Equal below code. In that case, in error case it will panic
  let file = File::open("rust.rs").unwrap();
  let file = File::open("rust.rs").expect("Failed to open rust.rs file");
  /*
  let file = File::open("rust.rs");
  let file = match file {
    Ok(fc) => println!("File created successfully"),
    Err(error) => panic!("Problem creating file: {:?}", error),
  };*/
}

fn recoverable_errors_w_clousures() {

  // this is anonymous function or clousure. Inside `unwrap_or_else`
  File::open("rust.rs").unwrap_or_else(|error| {
    if (error.kind() == ErrorKind::NotFound) {
      File::create("rust.rs").unwrap_or_else(|error| {
        panic!("Problem creating file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file {:?}", error)
    }
  });
}

fn recoverable_errors() {
  let f = File::open("rust.rs");
  match f {
    Ok(file) => println!("File already exists"),
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("rust.rs") {
        Ok(fc) => println!("File created successfully"),
        Err(error) => panic!("Problem creating file: {:?}", error),
      },
      _ => panic!("Error"),
    }
  };
}
fn backtrace_a() {
  backtrace_b();
}

fn backtrace_b() {
  backtrace_c(21);
}

// To check the trace of the error: RUST_BACKTRACE=1 cargo run
fn backtrace_c(num: i32) {
  if num == 22 {
    panic!("Do not pass 22!")
  }
}