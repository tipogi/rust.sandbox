//URL: https://exercism.org/tracks/rust/exercises/low-power-embedded-game

#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter
        .enumerate()
        .filter(|(index, element)| index % 2 == 0)
        .map(|(index, element)| element)
    // Or
    // iter.step_by(2)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}

fn main() {

}
