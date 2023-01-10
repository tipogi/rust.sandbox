//URL: https://exercism.org/tracks/rust/exercises/health-statistics

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User {
            name,
            age,
            weight
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

fn main() {
    let mut satoshi = User::new(String::from("Satoshi"), 2009, 1.0);
    println!("{} was born in {} at weight {}", satoshi.name(), satoshi.age(), satoshi.weight());
    satoshi.set_age(2023);
    satoshi.set_weight(2.0);
    println!("{} was born in {} at weight {}", satoshi.name(), satoshi.age(), satoshi.weight());
}