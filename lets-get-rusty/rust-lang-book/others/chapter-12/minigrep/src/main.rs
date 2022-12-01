use std::env;
// It will help us, exit the program without panic
use std::process;
// name of the library crate
use minigrep::Config;

// When the main function has too many responsabilities, the rust community has developed a 
// pattern for when the main function of a binary crate gets too large.
// To solve that, we have to create a library crate and then the binary crate (main.rs) can call functions
// in the library crate (lib.rs)

fn main() {
    // Add vector type becuase collets needs to know what kind of collection type has to return
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args)
        // it takes a clousere, as an argument
        // Also in the Ok case, it will return the object
        .unwrap_or_else(|err| {
            // This will print in the standard error stream
            // This is usefull if we want to redirect the output to one file
            // and we want to see the errors in the terminal, not in the file
            eprintln!("Problem parsing arguments: {}", err);
            // Terminate the program
            process::exit(1)
        });

    // Instead of bringing minigrep to the scope, use directly
    if let Err(e) = minigrep::run(config) {
        // This will print in the standard error stream
        // This is usefull if we want to redirect the output to one file
        // and we want to see the errors in the terminal, not in the file
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

