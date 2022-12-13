// POINTER: It is a general concept for a variable that stores a memory address
// That memory address refers to or points to some other data in memory
// The most common pointer in Rust, is a reference. References simply borrows the values
// they point to, meaning that they do not have ownership over the values. References does not have
// any special capabilities which also means they do not have much overhead unlike smart pointers
// Smart pointers are data structures that act like a pointer but have metadata and extra capabilities
// tacked on. 
// One example is a reference counting smart pointer which allows a piece of data to have multiple
// owners by keeping track of the owners and once there are no more owners, cleaning up the data
// In many cases, smart pointers own the data that they point to unlike references which simply 
// borrow the calues
// String and Vectors are smart pointers because they own some data and allow you to manipulate it,
// they store extra metadata such a capacity and they have extra capabilites.
// For example, the string type ensures that the data is valid utf-8
// Smart pointers are usually implemented using structs but unlike regular structs, they implement the 
// dref and drop traits.
// The dref traits allows instances of your smart pointer struct to be treated like references so we 
// can write code which works with either reference or smart pointer
// The drop traits allows to customise the code that is run when an instance of the smart pointer
// goes out of the scope.
// A smart pointer is a general design pattern used frequenly in rust. Also many libraries, implement
// their own smart pointers
// One of the common smart pointers is box which allows to allocate values on the heap

// Rust has to know how much space a type takes up at compile time
enum List {
    // Cons list is a datastructure the comes from LISP programming language
    // As we said, Rust at compile time has to know which is the size of each type but in that case,
    // we cannot know which size is the List because it is in a recursive loop and we cannot calculate the size
    // So adding a Box smart pointer, we have a pointer in the stack which is a fixed size,
    // and dynamic/arbitrary size in the heap and we can calculate the size of the Cons
    //Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};


fn main() {
    // In the stack, we store the pointer or memory address to the location of five on the heap
    // We will use in the following situations:
    // - When we have a type whose exact size cannot be known at compile time and we want to 
    // use a value of that type in a context which requires knowing the exact size
    // - When we have a large amount of data and we want to transfer ownership of the data but 
    // we want to make sure that data isn't copied because its a large amount of data
    // - When we own a value and we only care that the value implements a specific trait rather 
    // than it being a specific type. This third case is known as a "trait object"
    // That wouldn't be a good scenario
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}