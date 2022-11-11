// Bring HashMap to the scope
use std::collections::HashMap;

fn main() {
    let name = "hello";
    let mut mutable_string = String::from("hello ");
    // We use Standard (std) module (libray, namespace,...) to get the CLI arguments
    // args() function it is an iterator
    // skip() creates a new iterator. In that case we delete the first element of the iterator
    // In rust all the bindings are unmutable by default so, make the args binding mutable
    // BINDING = VARIABLE
    let mut arg = std::env::args().skip(1);   
    // Rust models as Option type that binding 'arg.next()' becuase it might be or not the value
    // To avoid that we chain another functions as expect
    // It returns String (as the key type is now) if the value is not 'None'
    // Else through an styled message. In the example above we will use unwrap which it does similar job
    let key = arg.next().expect("The key value is None, type back the command"); 
    // Another option is unwrap()
    // If the value is not there, in rust would be 'None', unwrap (crash the program). if not return 'String'
    let value = arg.next().unwrap(); 
    println!("[LOG] KVStore: The key/value is {}-{} ", key, value);
    //
    let contents = format!("{}\t{}\n", key, value);
    // write() returns a Result to avoid the io::Error if something goes wrong
    // The first unit is the success value which is empty Tuple known as unit. It is similar to void
    // but the second one is the error value
    let write_result = std::fs::write("kv.db", contents);
    // Other OPTION would be to use unwrap to crash the program directly
    //let write_result = std::fs::write("kv.db", contents).unwrap();
    // Evaluate with patter matching the result
    match write_result {
        Ok(()) => {
            print!("The key/value added successfuly");
        }
        Err(e) => {
            print!("{}", e);
        }
    }
    // MORE TO DIG: This is not a reference, it is allocated in the stack
    let database = Database::new();
}

// In Rust we create the type with its types and after we have the implementation
// where we add the methods and associated functions of the type
struct Database {
    map: HashMap<String, String>
}

impl Database {
    // Create associate method that construct the database
    // It does not matter the naming, there is not convention
    // MORE TO DIG: In Rust new does not imply allocation as in C++
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // read the kv.db file
        // Check in readme what does it do ? symbol
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // Deconstruct the tupble after split function
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            // Because key and values are borrowed and we need owned string, copy the memory to owned
            // If we would clone, we would have another view which is not owned
            // We can just have one owned string but multiple views
            map.insert(key.to_owned(), value.to_owned());
        }
        // Instantiate new Database struct
        return Result::Ok(Database { map: map });
    }
}

