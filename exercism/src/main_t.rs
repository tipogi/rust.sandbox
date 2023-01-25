struct Human {
    head: Option<bool>,
    legs: i8,
    arms: Option<bool>
}

impl Human {
    fn new(head: bool, legs: i8) -> Self {
        Self {
            head: Some(head),
            legs,
            arms: None
        }
    }

    fn has_arms(&self) {
        let arms = self.arms.as_ref();
        let has_arms = arms.is_some();
        println!("Human has arms: {:?}", has_arms);
    }
}

fn main() {
    let avatar = Human::new(true, 4);
    avatar.has_arms();
    let s = "Hello"; 
    let r1 = s; 
    let r2 = Some(s); 
    println!("{}", r1.len()); // 5 
    println!("{:?}", r2.unwrap().len()); // 5

    let rust = Some("Rust".to_owned());
    let rust_ref_value = rust.as_ref();
    let rust_ref_opt = &rust;

    let unwrap = rust_ref_value.unwrap();
    //let unwrap_own = rust_ref_opt.unwrap();

    match rust_ref_value {
        Some(x) => println!("{:?}", x),
        None => println!("None")
    }
    println!("ownership lost: {:?}", rust_ref_value);
    println!("ownership lost: {:?}", rust_ref_opt);
}