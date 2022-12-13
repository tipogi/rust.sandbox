// In rust all the iterators implements the iterator trait which is defined 
// in rust standard library
// This code said when implementing the iterator trait we also have to define
// the item type and that type is returned from our next() method.
// There is just one method we have to implement which is the next() method.
// All the other methods have a default implementations

// Iterator traits has various methods which have default implementations
// provided by the standard libray. There are two broad categories of methods:
// - Adapters which take in an iterator and return another iterator
// - Consumers which take in an iterator and return some other type such as an 
// integer, collection or any other type
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demostration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    // Note that the iterator return inmutable references.
    // If we want immutable references, instead of iter(), iter_mut()
    // And if we want own type into_iter()

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);


    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // This is a consumer
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

fn main() {
    let v1 = vec![1,2];
    // Create the iterator. In Rust iterators are lazy so,
    // when we create iterator for vector nothing special happens
    // until we use the iterator in the for loop
    let v1_iter = v1.iter();

    // In that case, we do not have any logic how to iterate a vector,
    // the sequence of elements, because that logic is encapsulated within
    // the iterator
    for value in v1_iter {
        println!("{}", value);
    }

    // Adapter methods as map
    let v2 = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(| x | x + 1).collect();

    assert_eq!(v3, vec![2,3,4]);
}