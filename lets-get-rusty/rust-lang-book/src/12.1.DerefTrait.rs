use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Without the Deref trait, the compiler only knows how to de-reference references
// The Deref trait allows the rust compiler to take any value that implements deref
// call the deref method to get the reference which the compiler knows how to dereference
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {// or &T {
        // MyBox is a Tuple struct (check above) so, inside the dref method
        // we are returning the reference to the first element
        &self.0
    }
}

// Deref trait allows us to customise the behavior of dereference operator(*)
fn main() {
    let x = 5;
    // Reference to x. In that case, y is a memory address or pointer
    // that point to the location where 5 is stored
    let y = &x;
    // Box is pointing to a value stored somewhere in memory, in that case value 5
    // The difference between y and z is that z is pointing to a copy of five because
    // with primitive values, when we pass to a function it copies instead of ownership
    // being transferred
    let z = Box::new(x);

    assert_eq!(5, x);
    // Deferencecing, it will follow the memory address that is stored in y
    // to the actual value 
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    
    let z1 = MyBox::new(x);
    // in the background, rust compiler does with smart pointers
    // assert_eq!(5, *(y.deref()))
    assert_eq!(5, *z1);

    let m = MyBox::new(String::from("Rust"));
    // Even m is MyBox type and we pass a reference to MyBox (&MyBox<String>), we pass to hello 
    // function that expects a string slice. Because MyBox implements deref trait and if we call 
    // dref on "m", we will get back a reference to a string
    // &MyBox<String> -> &String, it calls deref coercion as we describe below
    // Because String also implements deref, we will get &String -> &str
    hello(&m);
    // Without deref coercion, we will need to add m in hello function
    // hello(&(*m)[..]);
}

// Implicit deref coercion is a feature that does happen for types that implements the deref trait
// Deref coercion will convert a reference to one type to a reference to a different type
fn hello(name: &str) {
    println!("Hello, {}", name);
}

// More about deref coercion and mutability: https://youtu.be/dYEC6NElVOg?t=527