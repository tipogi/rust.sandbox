use std::{rc::Rc, sync::Arc};

fn main() {
    // URL: https://www.youtube.com/watch?v=PbR4ECFIckg
    fancy_print("Hello mistakes!");
    let s = "8 common mistakes".to_owned();
    // It is going to be coerced directly to string slice
    fancy_print(&s);
    // 2. Overusing slice indexing: [1], should use looping functions
    // 3. Using sentinel values: -1, "", null
    // 4. Error handling 
    // 5. Not using standard library traits: 
    //  - Default: Player::default() 
    //  - From: Convert between types
    //  - FromStr: Allows to parse user-defined type from a string
    // 6. Not taking advantage of standar library macros 
    //  - todo!, concat!, format! 
    // 7. Not using the tooling: cargo fmt, cargo clippy (linter for the rust code). We can configure clippy with github actions 

    // URL: https://www.youtube.com/watch?v=Nzclc6MswaI
    // A traits provides a basic functionality as these ones: Debug, Clone, Default, PartialEq, Send & Sync
    let user = User {
        id: 123,
        name: "Bogdan".to_owned(),
        role: Role::Admin,
        db: Arc::new(DB {}),
    };

    println!("{:?}", user);

    let user2 = user.clone();

    println!("{:?}", user2);

    let guest = User::default();

    let guest2 = User::default();

    assert_eq!(guest, guest2);

    // Create a JSON string
    let user_str = "{ \"id\": 123, \"name\": \"Bogdan\", \"role\": \"Admin\" }";

    // Create an object of user
    #[cfg(feature = "serde")]
    let user: User = serde_json::from_str(&user_str).unwrap();

    #[cfg(feature = "serde")]
    println!("{:?}", user);
}

fn fancy_print(s: &str) {
    println!("************** {} **************", s);
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Role {
    Admin,
    Standard,
    // Because it does not exist which is the default value for that type, we specify
    #[default]
    Guest,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct DB {}

// Implement Debug trait for better debugging. As we implement that trait all the parents also has to implement as DB, Role
// We use Clone to make copies of the types
// Default: Create an instances with the default values
// ParcialEq: To compare instances of a given type
// Send: if the type is safe to send between threads
// Sync: if the type is safe to be shared between threads via references
// The last two trades are automatically implemented for all the types if one of the variables has to implenent that trait
// Serialise, deserialise the object
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct User {
    id: u32,
    name: String,
    role: Role,
    #[cfg_attr(feature = "serde", serde(skip))]
    // AtomicReferenceCounter: Can be shared between threads
    db: Arc<DB>,
}

// A trick from Jon Gjengset
// To check if an type implement Send & Sync traits create this function. After create a test with the type that we want to test if it is Sync & Send
fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    is_normal::<User>();
}