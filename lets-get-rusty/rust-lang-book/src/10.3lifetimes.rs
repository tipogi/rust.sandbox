// RUST BOOK: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// - BORROW CHECKER: Checks the lifetime of references if they are valid or no
// - GENERIC LIFETIME ANNOTATIONS: Describes the relationship between lifetime of multiple references
// and how they relate to each other. They do NOT CHANGE THE LIFETIME OF A REFERENCE but rather just 
// EXPLAIN (specify, create) THE RELATIONSHIP betweeen different lifetimes of multiple references.
// When the say lifetimes, they refer to generic lifetime annotations

use std::fmt::Display;

fn dangling_references() {
  //let r;
  // Inner scope it call that
  {
    let x = 5;
    //r = &x;

  }
  // Are is referecence to x but x is out of scope because
  // it leaves until the brackets are closed
  //println!("r: {}", r);
}

// 
fn generic_lifetime_annotations() {
  let string1 = String::from("abs");
  let string2 = String::from("car");

  let result = longest(string1.as_str(), &string2);
  // The borrow check here would say, which is the reference with the smallest life time
  // in that case, both the same because are in the same scope. After when the result is printed,
  // is the smallest lifetime still valid?
  println!("The longest string is {}", result)
}

fn wrong_lifetime_annotations() {
  let string1 = String::from("rabs");
  {
    let string2 = String::from("car");
    let result = longest(string1.as_str(), &string2);
  }
  // In that case, string2 does not exist in the scope because it was dropped from the memory
  // In that case, would be dangling reference  
  //println!("The longest string is {}", result)
  
}

// In that case, we need to help to the compiler because it is not able to check the lifetime of
// the return (output) reference. We need to use generic lifetime annotations
// MORE INFO IN ARTICLES FOLDER: 1
// To fix this we need to use generic lifetime annotations. Check above

//fn longest(x: &str, y: &str) -> &str {

// Generic life time annotations always starts with apostophe, followed by the name of the lifetime
// We can named as we want, but the convention is to name it a lowercase letter
// After we specify, who is going to use the life time
// We declare generic lifetime a ('a) and then we annotate x, y and the return value
// This means that x, y and the return value will have the same lifetime. NO!! the
// generic lifetime annotations do not change the lifetime, they just create the relationships between
// the lifetimes
// Here what we say, is that there is a relationship between x, y and return value
// The relationship is this: The lifetime of the returned reference, will be the same 
// as the smallest lifetime of the arguments
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn structs_lifetime() {
  let novel = String::from("Call me Isma. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find");
  // If we will use out of scope that struct, we will get compile error
  let i = ImportantExcerpt {
    part: first_sentence
  };
  println!("{:?}", i.part);
}

// There are some scenarios that the compiler can deterministically infer the lifetime
// annotations and it does this by checking the three lifetime elision rules:

// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all
// outputs lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut, the 
// lifetime of self is assigned to all output lifetime parameters. It only applies to methods

// Historically, we should add annotations to all the functions
//fn first_word_with_lifetime_elision<'a>(s: &'a str) -> &'a str {
// But with the time, they realised that it was not necessary. That patterns in Rust,
// are called lifetime elision rules
fn first_word_with_lifetime_elision(s: & str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

// Structs with lifetime annotations
struct ImportantExcerpt<'a> {
  part: &'a str,
}

// Because lifetime annotations are like generics write after impl and after struct
impl<'a> ImportantExcerpt<'a> {
  // In that case, the compiler applies the third rule and thats why we do not need to add
  // lifetime annotations
  fn return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

// Two string references and one generic type
fn longest_with_an_announcement<'a, T>(
  // Because we have more than one references, the compiler cannot do automatic lifetime
  // elision so, we have to manually specify the lifetime
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str
// We use here the trait bound to limited any type that implements display
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
      x
  } else {
      y
  }
}
        
fn main() {
  dangling_references();
  generic_lifetime_annotations();
  wrong_lifetime_annotations();
  structs_lifetime();
  let elision = String::from("elision");
  first_word_with_lifetime_elision(&elision);
  let excerpt = ImportantExcerpt {
    part: "Hello"
  };
  let part = excerpt.return_part(&elision);
  println!("Part of struct -> {}", part);
  // Static lifetime: The reference could life as long as the duration of the program
  // All the string literals has static lifetime because they are stored in the program binary
  let s: &'static str = "I have a static lifetime";
  longest_with_an_announcement();
}