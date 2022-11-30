// We created a library called addr: `cargo new adder --lib`

// Cargo test compiles the code in test mode and runs the resulting test binary
// By default all tests get run in parallel in a separete thread
// Also all generated output is captured and not printed to the screen
// For example: if we add println! that it would not get printed out when we do `cargo test`
// This is because by default standard output is captured for passing tests and we do not see
// on the screen but we could change that with command line options
// cargo test -- --show-output
// There are two sets of command line option
// 1. `cargo test` command 
// 2. Other goes to the resulting test binary: `cargo test --``. Sometimes we want to run the test
// serially not in parallel, for example a file write and after read. 
// COMMANDS:
// - show outputs: `cargo test -- --show-output`
// - run specific test: `cargo test fn_name` 
// - run tests base on module: `cargo test mod_name::`

// TEST ORGANIZATION
// - Unit Test: Small, focused module tests and could test private interfaces. Unit test lives
// in the same file as the product code
// - Integration Test: External test to the library. Lives in the folder called test at the root project

// CONVENTION
// Rust suggest to put the product code and the test code in the same file

pub fn print_ten(number: i32) -> i32 {
    println!("Print number {}", number);
    number
}

// That module attribute 'cfg' means configuration 
// test means that cargo will compile, when we will run `cargo test`
// Child modules are able to access all that is in the parent module even
// they are private
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works_b() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn it_prints_ten() {
        let number = print_ten(10);
        assert_eq!(number, 10);
    }

    // Run the ignored test: cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that take one hour to run
    }
}