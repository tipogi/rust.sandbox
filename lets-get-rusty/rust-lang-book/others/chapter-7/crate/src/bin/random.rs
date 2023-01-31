pub struct Random {
    number: u32
}

impl Random {
    pub fn new(number: u32) -> Self {
        Self {
            number
        }
    }

    pub fn display(&self) {
        println!("Random number: {}", self.number);
    }
}

pub fn main(){
    println!("main fn from bin folder");
}