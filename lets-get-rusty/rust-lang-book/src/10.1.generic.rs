enum Option<T> {
  Some(T),
  None,
}

enum Result<T, U> {
  Ok(T),
  Err(U),
}

#[derive(Debug)]
struct Point<T, U> {
  x: T,
  y: U,
}

// To use generics after impl define
impl<T, U> Point<T, U> {
    // method
    fn x(&self) -> &T {
      // Refernce of x
      &self.x
    }
}

// Available function for the ones that has x and y a float(f64)
// For other Point types y() is not going to be available.
// Test with point_a of generic_struct fntion
impl Point<f64, f64> {
  fn y(&self) -> f64 {
    self.y
  }
}

// WARNING: When is generic type always after `impl` add the generic types
// Implementation block has two generics for our Point struct and after
// `mixup` has its own set of generics: V and W and are scoped to mixup function
impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y
    }
  }
}

fn main() {
  generic_list();  
  generic_structs();
}

fn generic_structs() {
  let point_a = Point { x: 10.5, y: 'd' };
  let point_b = Point { x: 12, y: 5.0 };
  // point_b, cannot call to y() because x is Integer and it has to be float
  // to call the function
  //println!("{}", point_b.y());
  let mixed_point = point_a.mixup(point_b);
  // we need to add in the struct (#[derive(Debug)]) to print the element
  println!("{:?}", &mixed_point);
  // mixed_point can call to y() because x and y are float
  println!("{}", mixed_point.y());

}

fn generic_list() {
  let bigger_number = get_largest(vec![1,4,7,3,2]);
  let bigger_char = get_largest(vec!['e', 'd', 'z', 'w']);

  println!("Number: {}, Char: {}", bigger_number, bigger_char);
}

// Because we want to use generics, add after function name
// PartialOrd and Copy are traits and define the generic type
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];
  for element in list {
    if element > largest {
      largest = element;
    }
  }
  largest
}