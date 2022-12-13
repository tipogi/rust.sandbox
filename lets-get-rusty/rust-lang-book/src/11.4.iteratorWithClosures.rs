#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // shoe_size is inside of the context but it is not defined within the closure.
    // Because we use closure, it is accesible
    // collect() returns a collection
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 13,
                style: String::from("sandal")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            }
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }
}

// Custom Iterator
struct Counter {
    // Private field because we want to access just from the implementation of the struct
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

// Implement Iterator trait for Counter
impl Iterator for Counter {
    type Item = u32;

    // The only method required to implement Iterator
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

#[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            // Skip the first n elements, in our case 1
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        
        assert_eq!(18, sum)

    }
//