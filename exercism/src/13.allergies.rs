// URL:https://exercism.org/tracks/rust/exercises/allergies

use self::Allergen::*;

pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALLERGENS: [Allergen; 8] =
    [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // Able to access to the value of the variant casting
        let allergen = *allergen as u32;
        self.score & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(a))
            // Because we have all the allergens as borrowing, we need to get the ownership
            .cloned()
            .collect()
    }
}

fn main() {
    // Left shift
    // (In Binary)
    //1000         ->    1111101000  
    // << 2                     |  left shift the bits
    // -----                    V  by 2 places
    // 4000         <-  111110100000 
    //                  (In Binary) 
    //          OR
    // It is equivalent to multiplying x by 2y (power y).
    println!("{}", 2 << 1);

    // Bitwise, AND operation
    // 1100    1100
    // 0100    1000
    //------   ----
    // 0100    1000
    // In the example case, gives the Allergen group
    println!("{}", 12 & 32);

    // Get the variant value
    println!("{:?}", Allergen::Eggs as u32);

    let alergy = Allergies::new(18);
    println!("{:?}", alergy.is_allergic_to(&Allergen::Tomatoes));
    println!("{:?}", alergy.allergies());
}   