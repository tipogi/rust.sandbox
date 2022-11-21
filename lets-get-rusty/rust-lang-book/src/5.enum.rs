// Variants are namespaces under their identifier and to specify a 
// variant we use :: (colon colon)
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String)
}

struct IPAddr {
  kind: IpAddrKind,
  address: String
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn write_message(){
    println!("lets get Rusty")
  }
}

// Option enum: Many languages have null values and no values represent a useful concept
// A value could either exist or it could be null, no value.
// But the problem with null value is that the type system cannot guarantee that if we use a value
// it is not null  
// In Rust there is not null values. Instead we have option enum. It has just two variants:
// - Some(T) which stores some value
// - None: no value
// So if we have a value that could potentially be null or not exist, we will wrap it in option enum
// This allows to the type system to enforce that we handle the none case, when a value doesn't exist
// and guarantee that in the some case our value is present
// Optional values are so useful that the option enum and its variants are included in our program scope
// by default 

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
  //...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("luck Penny, keep it!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // Also can bind to values
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1)
  }
}

fn main() {
  let localhost = IpAddrKind::V4(127, 0, 0, 1);

  let x = 8;
  let y = Some(5);

  let sum = x + y.unwrap_or(0);

  value_in_cents(Coin::Quarter(UsState::Alabama));
  value_in_cents(Coin::Penny);

  println!("Plus one {:?}", plus_one(Some(21)));

  let three = Some(4);
  
  match three {
    Some(3) => println!("Three found"),
    _ => (),
  }
}