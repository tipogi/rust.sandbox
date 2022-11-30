// We created a library called addr: `cargo new adder --lib`

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn sum_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Gues value must be between 1-100");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    
    // To bring the product code which is in default module, we bring it into the test scope 
    // We reference the parent module (super)
    use super::*;

    #[test]
    fn it_sums_two() {
        // NOTE: Traits like assert_eq! and assert_no! have to implement partial equal and debug traits
        assert_eq!(sum_two(2), 4);
    }

    // A test function, needs to have the test attribute 
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller)); 
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger)); 
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }
    #[test]
    // The test aim would be to panic and with that exact message
    #[should_panic(expected = "Gues value must be between 1-100")]
    fn greater_than_100() {
        Guess::new(200);
    } 

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
