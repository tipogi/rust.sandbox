// URL: https://exercism.org/tracks/rust/exercises/clock
use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}


// To use to_string() function, implement Display trait 
// (https://doc.rust-lang.org/std/fmt/trait.Display.html)
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pad 0 if the len is 1
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

// Implement PartialEq trait. 
// We could also add PartialEq in the clock struct ;)
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_mins = hours * 60 + minutes;
        if total_mins < 0 {
            total_mins += (1 - total_mins / (24 * 60)) * 24 * 60
        }
        Clock { hours : (total_mins / 60) % 24, minutes : total_mins % 60 }
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

fn main() {
    let clock_one = Clock::new(8, 0).add_minutes(3);
    println!("{:?}", clock_one.to_string());
}