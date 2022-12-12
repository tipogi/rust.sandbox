// Capturing the environment with closures
fn main() {
    let x = 4;
    // Unlike functions, closures are able to access to variables that
    // are defined in the same scope. Because of that, they need to reserve
    // extra memory to store that context
    let equal_to_x = |z| z == x;

    let y = 4;
    // Assert macro
    assert!(equal_to_x(y));

    let y2 = vec![1,3,4];
    let equal_to_y2 = |z| z == y2;
    // move ownership to the closure with 'move' keyword
    //let equal_to_y2 = move |z| z == y2;
    println!("cannot use y2 here: {:?}", y2);

    let j = vec![1,3,4];
    assert!(equal_to_y2(j))
}

// Closures capture values from their environment in three ways which directly map
// to the three ways a function could take in input parameters: 
// - Taking ownership
// - Borrowing mutably
// - Borrowing immutably
// This are encoded in the function traits
// - FnOnce: takes ownership inside closure environment. Closure cannot take ownership of the same variable
// more than once, so this closure can be called once
// - FnMut: Mutably borrows values
// - Fn: Inmutably borrows values

// We could however, force the closure to take ownership of the values it uses inside its environment 
// by using 'move' keyword in front of the closure.
// This is mostly useful when we are passing a closure from one thread to another thread so, we can
// also pass the ownership of the variables 