

fn main() {
  create_structs();
  without_name_fields();
  calculate_area();
  create_rectangle();
}

// Adding here derive, allows the compiler to provide a basic
// implementation of the debug trait
#[derive(Debug)]
struct Rectangle {
  height: i32,
  width: i32
}

impl Rectangle {
  // Add method in the rectangle struct. Methods are tied to an instance of struct
  // The first argument in a method is always self which is the instance the method
  // has being called on
  fn rect_area(&self) -> i32 {
    self.width * self.height
  }
}

impl Rectangle {
  // This is associated function and not method, it does not have self as a first argument
  fn square(size: i32) -> Rectangle {
    Rectangle { height: size, width: size }

  }
}

fn create_rectangle() {
  let rect = Rectangle {
    height: 32,
    width: 5
  };

  let rect2 = Rectangle::square(4);

  println!("rectangle: {:?}", rect);

  println!(
    "The area of the rectagle is {}",
    // Automatic referencing and dereferencing
    rect.rect_area()
  )
}



fn calculate_area() {
  let rect = (43, 32);
  println!(
    "The area of the rectagle is {}",
    area(rect)
  )
}

fn area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn without_name_fields() {
  // That one call tuple structs, when we want to have same attribs but different type
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
}

struct User {
  // We want that our user will own username and email data
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn create_structs() {
  let mut user1 = User {
    email: String::from("rust@lang.rs"),
    username: String::from("Rusty"),
    active: true,
    sign_in_count: 1
  };

  let name = user1.username;
  user1.username = String::from("Rust Crap");

  let user2 = build_user(
    String::from("book@rust.rs"), 
    String::from("book")
  );

  let mut user1 = User {
    email: String::from("struct@rust.rs"),
    username: String::from("Struct"),
    ..user2
  };
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: false,
    sign_in_count: 0
  }
}