// - BORROW CHECKER: Checks the lifetime of references if they are valid or no
// - GENERIC LIFETIME ANNOTATIONS: Describes the relationship between lifetime of multiple references
// and how they relate to each other. They do NOT CHANGE THE LIFETIME OF A REFERENCE but rather just 
// EXPLAIN (specify, create) THE RELATIONSHIP betweeen different lifetimes of multiple references.
// When the say lifetimes, they refer to generic lifetime annotations

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

  println!("The longest string is {}", result)
}

// In that case, we need to help to the compiler because it is not able to check the lifetime of
// the reference. We need to use generic lifetime annotations
fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  dangling_references();
  generic_lifetime_annotations();
}