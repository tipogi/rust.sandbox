use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    // Create the random number        
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess: ");
        // Make mutable the binding. By default all bindings are inmutable
        let mut guess = String::new();
        // Get the input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Parse our string to integer
        // This calls shadowing: Convert one variable from one type to other but preserving the name
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare the numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".blue()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal =>  {
                println!("{}", "==> You did guess the number, congratulations! <==".green());
                break;
            }
        }
    }
}
