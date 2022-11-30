// Every file inside of test folder will be a crate
// If we add a folder, it is not threated as crates and are not going to compiile as a integration test

// We created a library called addr: `cargo new adder --lib` and we need to bring to the scope
// because every file inside of test directory is gonna be a new crate
use adder;

// This is a module declaration and it will look for the contents of the module in either
// a file called common.rs or a folder called common with a file called mod.rs.
// Then we can call a function inside of the module: common::fn_name
// We the mod module, we write the folder name and by convention will take mod.rs
mod common;

// Here, we do not have a module with the cfg annotation  because cargo knows that all the files that lives in test folder
// are test files
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::sum_two(2));  
} 

// One thing to know here is, we have a `lib.rs` file in the source directory which means, we have a library
// crate. If we have a main.rs file, we would have a binary crate and we cannot directly test binary crates with
// integration tests.
// This is why it is common to see a binary crate that's a thin wrapper around a library crate. So, that way, we can
// test the library crate with integration tests