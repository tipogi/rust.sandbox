fn main() {
  stack_vs_heap_variables();
  fn_ownership();
  fn_reference();
  fn_restrictions();
  fn_dangling();
  fn_string_slices();
  fn_literal_string();
}

fn fn_literal_string() {
  let mut sentence = String::from("Hello Rust");
  let rust = &sentence[6..];
  // The string literals are string slices that are located in the binary
  let s2 = "Hello Rust";
  // In that case the String gets automatically coerced to a string slice
  find_first_word(&sentence);
}

// &str: String slice
fn find_first_word(sentence: &str) -> &str {
  let bytes = sentence.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &sentence[0..i];
    }
  }
  return &sentence[..];
}

/**
 * Slices let us reference a contiguous sequence of elements within a collection,
 * instead of referencing the entire collection
 * Slices DO NOT TAKE the ownership of the underlying data
 */
fn fn_string_slices() {
  let mut s = String::from("Hello Rust");
  // Omit the first index
  let hello = &s[..5];//[0..5]
  // In that case the pointer would point to the index 6
  // omit the last index
  let rust = &s[6..]; //[6..9]

  let mut s2 = String::from("Hello Rust 2");
  let hello2 = &s2[..6];
  // Below code is wrong because s2 is mutable but it cannot be like that
  // s2 is borrowing the ownership to hello2 and it is inmutable
  // We cannot mix MUTABLE and INMUTABLE in the same scope
  //s2.clear();
  //println!("{}", hello2);
}

fn fn_dangling() {
  //let reference_to_nothing = dangle();
}

/*fn dangle() -> &String{
  let s = String::from("dangle")
  // In that case, we are returning a reference but rust will drop out 
  // or deallocate from the heap when the variable will be out of scope
  // In that case, s will be pointing to invalid memory
  // RUST compiler will prevent to do that
  &s;
}*/

fn fn_restrictions() {

  let mut s = String::from("Restrictions");

  // UNCOMMENT TO SEE THE ERROR (command s1, s2 below)
  //let s1 = &mut s;
  //let s2 = &mut s;
  // We cannot borrow s as inmutable more than once at a time
  // The big benefit of this restriction is that Rust can prevent data races at compile
  // time
  // A data race occurs, if we have two pointers pointing to the same piece of data and 
  // one of those pointers is used to write the data and there's no mechanism to synchronise 
  // data access between those pointers.
  // In that situation, we could imagine that one pointer will try to read the data in the 
  // middle of the other pointer modifying the data. In that case, we will get corrupt 
  // data back
  let s1 = &s;
  let s2 = &s;
  // Another restrictions is that we cannot have inmutable and mutable references but we can
  // have mutiple inmutable references

  println!("{}, {}", s1, s2);

  // In that case s1 and s2 would be out of the scope because is not anymore in use and 
  // we could make mutable the s variable but just once
  let s3 = &mut s;
  println!("{}", s3);

}

fn fn_reference() {
  let s1 = String::from("Crusty");
  let len = calculate_len(&s1);

  let mut s2 = String::from("Muted");
  mutate_now(&mut s2);
  // If we do not pass reference to the calculate_len, it will be moved the s1 
  println!("LEN: {}, WORD: {}, MUTED: {}", len, s1, s2)
}

// We do not take the ownership of the underlying value but we can mutate
fn mutate_now(s2: &mut String) {
  s2.push_str(" by RUST")
}

// We pass the reference of the string because it does not take the ownership
// of the underlying value. Passing arguments with references, it calls borrowing
// because it does not take the ownership. References are inmutable by default
fn calculate_len(s1: &String) -> usize {
  let len = s1.len();
  len
}

fn fn_ownership() {
  let s = String::from("Rust2");
  takes_ownership_correct(&s);
  // UNCOMMENT TO SEE THE ERROR
  //takes_ownership_wrong(s);
  
  let x = 21;
  makes_copy(x);

  let z = give_ownership();
  
  // We cannot print that because the ownership was borrowed
  let s2 = String::from("finney");
  let s3 = takes_and_gives_back(s2);

  println!("END: {}, {}, {}, {}", s, x, z, s3);
}

// Moving ownership might be a tedious task, better use references
fn takes_and_gives_back(s2: String) -> String {
  // move the value out of the function to s3
  s2
}

fn give_ownership() -> String {
  let x = String::from("sats");
  x
}

// In that case, we do not move because it is an integer
fn makes_copy(x_copy: i32) {
  println!("{}", x_copy);
}

// In that case, we move the s to s_move and when is out of the scope
// Rust deallocates the variable
fn takes_ownership_wrong(s_move: String) {
  println!("{}", s_move);
}

fn takes_ownership_correct(s_pointer: &String) {
  println!("{}", s_pointer);
}

fn stack_vs_heap_variables() {
  let x = 5;
  let _y = x;

  let s1 = String::from("Rust");
  // This is not a shadow copy, it is a move because it is in the heap
  let s2 = s1;

  println!("{}", s2);
}