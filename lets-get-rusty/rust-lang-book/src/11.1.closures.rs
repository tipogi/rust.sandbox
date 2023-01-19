// CLOSUREs: Anonymous functions which they do not have names, they could be stored as a variables
// and pass around. They could also be passed as an input parameters to a function and they CAPTURE 
// the variables INSIDE the SCOPE in which they are DEFINED

use std::thread;
use std::time::Duration;

fn main() {
    let simulated_insensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_insensity, simulated_random_number);
}

// Use the memoization pattern(keed the value in some attrib: value in our case)
// by creating a struct which will hold our clousure
// and the result of our clousre
// In order to define structs, enums or function parameters that use closures, 
// we need to use generics and trait bounds
struct Cacher<T> 
// generic and trait bound
// This is not the optinum because we are using static types. To fix that, we can use generics
where T: Fn(u32) -> u32,
{
    calculation: T,
    // it is optional because when the Cacher would be initialised
    // is going to be none
    // Once we call to calculation, we will store the return value  
    value: Option<u32>
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn generate_workout(intensity: u32, randon_number: u32) {
    // We did not define the input parameter of the clouse neither the return output type 
    // For regular functions we would have to specify the types
    // And that's because functions are part of an explicit interface exposed to users so
    // agreeing on the types being passed in and returned is important
    //
    // Closures usually are short and only relevant within a narrow context so, the compiler 
    // is able to determine the input parameter types and the types
    //
    // The same happen with variables
    // Clousures have one concrete type inferred for each input parameter. Compiler defines
    // as input type, the first type passed into the closure
    // That function level up adding in the Cacher struct
    let mut cached_result = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cached_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cached_result.value(intensity)
        );
    } else {
        if randon_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cached_result.value(intensity)
            );
        }
    }
}