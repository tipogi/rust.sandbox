// EXERCISE G
// We can declare modules using `mod` keyword and we can define the contents of the module in another
// file using semi-colom
// For that we need to create a file (.rs) with the same name
// Similarly we can define child modules: hosting.rs
mod front_of_house_c;

pub use crate::front_of_house_c::hosting;

pub fn eat_at_restaurant_d() {
    hosting::add_to_waitlist();
}


// EXERCISE G
//use rand::Rng;
// Use nested paths to import more modules from the parent module
use rand::{Rng, CryptoRng, ErrorKind::Transient};
//use std::io
//use std::io::Write
use std::io::{ self, Write};

fn create_random_number() {
    let secret_number = rand::thread_rng().gen_range(1, 10);
}

// EXERCISE F
// If we want that some external code could reference that module will use `pub` keyword
// use not necessary for that use case but it is possible if we want to bring to the scope
pub use self::front_of_house_b::hosting;

//EXERCISE E
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // -- snip --
    Ok(())
}

// EXERCISE D
mod front_of_house_b {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// use let to bring a path into the scope
// Absolute path
//use crate::front_of_house_b::hosting;
// self is referencing the current module
// For ideomatic reasons is not convinient to bring the function to the scope
// because it will seem that we get from the global scope or is a local function
// But if we bring struct, enum or others, it is ideomatic to specify the full path. But
// It could be some exceptions as above. In that case use `as` keyword or reference the parent module
use self::front_of_house_b::hosting;

pub fn eat_at_restaurant_c() {
    hosting::add_to_waitlist();
    front_of_house_b::hosting::add_to_waitlist();
    front_of_house_b::hosting::add_to_waitlist();
}

// EXERCISE C
mod back_of_house_c {
    // If we mark an enum as public, all the variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house_c::Appetizer::Soup;
    let order1 = back_of_house_c::Appetizer::Salad;
}



// EXERCISE B
mod back_of_house_b {
    // By default struct fields are private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant_b() {
    let mut meal = back_of_house_b::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // Because one of the fields of Breakfast are private we cannot create Breakfast directly
    //back_of_house_b::Breakfast { toast: String::from("Integral"), seasonal_fruit: String::from("strawery") };
}

// EXERCISE A
fn server_order() {}

mod front_of_house {
    // Make public to access
    pub mod hosting {
        // Make public to access
        pub fn add_to_waitlist() {}

        fn set_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        // It is defined in the same module so, we can call
        // We use relative path
        cook_order();
        // super allow us to reference the parent module which in that case is crate
        super::server_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    back_of_house::fix_incorrect_order();
}