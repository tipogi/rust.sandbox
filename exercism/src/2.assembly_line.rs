// URL: https://exercism.org/tracks/rust/exercises/assembly-line

fn main() {
    production_rate_per_hour(3);
    working_items_per_minute(5);
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    //let produced_cars = (221 * speed) as f64;
    // If we do not do that conversion, we get `attempt to multiply with overflow`
    let produced_cars = 221.0 * speed as f64;
    match speed {
        5..=8   => final_production_number(90.0, produced_cars), 
        9..=10  => final_production_number(77.0, produced_cars),
        _ => produced_cars
    }
}

pub fn final_production_number(percentage: f64, produced_cars: f64) -> f64 {
    (produced_cars * percentage) / 100.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let produced_cars = production_rate_per_hour(speed);
    (produced_cars / 60.0) as u32
}