fn main() {
  exercise_a();
  exercise_b();
  exercise_c();
  loop_vector();
  vector_diff_types();
}

fn vector_diff_types() {
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  // If we want to have different type elements in the array, we can use enum
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(21.40),
    SpreadsheetCell::Text(String::from("rust learing"))
  ];

  match &row[0] {
    SpreadsheetCell::Int(i) => println!("Integer is {}", i),
    _ => println!("Not found number")
  }
}

fn loop_vector() {
  // Inmutable vector
  let v3 = vec![1, 2, 3];
  for i in &v3 {
    println!("{}", i);
  }
  
  // Mutable vector
  let mut v4 = vec![1, 2, 3];
  for i in &mut v4 {
    // Dereference Operator gets the underlaying value and add 50 to it
    *i += 50;
  }
  for i in &mut v4 {
    println!("{}", i);
  }
}

fn exercise_c() {
  let v2 = vec!["h1", "h2", "h3"];
  // When we access to the elements that are in the heap, it returns a reference
  // Always need to return a reference
  let headline_2 = v2[2];
  println!("Vector string element {}", headline_2);
  let mut y = 0;
  let y1 = &mut y;
  let y2 = &mut y; //w
}

fn exercise_b() {
  let v2 = vec![1, 2, 3];
  // Access to the second index. Reference to vector (&v2), return always a reference
  // In that case the vector is IMMUTABLE, we are not mutating any value, we JUST
  // take an immutable reference to a value in the vector
  let second = &v2[2];
  // CAREFUL (uncomment to see): Here we take a mutable reference to push new element
  // v2.push(3);
  // NOTE (making reference to above line): We cannot have mutable and inmutable reference to the same thing(v2) and in the same time
  // We create a new variable (second) and before print it out, we add an element (v2.push(3)) to the vector
  // In that case v2, cannot be a mutable, because we already borrow an immutable reference to a value of vector
  // If we do not use println!, it would not be a problem
  // PROBLEM: When we have an immutable reference to something, we expect the underlying value to not change.
  // But if we have a mutable reference to the same thing, then the underlying value could change
  // With vector that happens when we push a new element onto a vector: We might need to allocate more memory
  // to make room to a new value and when we do that, we need to move all the elements in the vector to new 
  // memory locations. In that reallocation of memory, the 'second' varialbe would be pointing to something else
  // That happen in the println because second could be in another memory space
  println!("print the second index {}", second);
  let ownership_lost = v2[2];
  println!("ownership lost? {}, in that case NO", ownership_lost);
}

fn exercise_a() {
  // array
  let a = [1, 2, 3];
  // When we create a new vector without elements, ALWAYS add type
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  // Add vector scope
  {
    // Use vector macro to initialise a vector with values
    let v2 = vec![1, 2, 3];
    // Access to the second index. Reference to vector (&v2)
    let second = &v2[2];
    println!("The second element is {}", second);
    // Because the compiler does not know the size of the vector in the compile time (it is store
    // in the heap), we can just catch that error in the running time
    //let out_of_bound = &v2[9];
    // We can control accesing saver to the index using `.get` method
    match v2.get(2) {
      Some(in_bound) => println!("The third element is {}", in_bound),
      None => println!("Out of bound the inde")
    }
  } // Here v2 will be deallocated from the memory
}