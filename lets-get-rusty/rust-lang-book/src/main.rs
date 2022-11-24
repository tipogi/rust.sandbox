// Strings are stored as a collection of UTF-8 encoded bytes
// Computers understand just 1 and 0. So, in memory a string is a collection
// of 1 and 0. The program needs to be able to interpret those 1 and 0 and print
// out the correct characters. An thats encoding comes in to play.
// In order to understand UTF-8, first we have to understand ASCII which is a string encoding
// so it defines how to takes 1/0 and turned into a string or viceversa
// History: Problem, each ASCII character is stored as a byte and only 7 bits of that
// byte are used to represent the character. That means ascii cna only represent 128 unique 
// characters. So, ASCII only represents the english alphabet.
// Because ASCII just represents english characters, other countries created their own encoding
// standards that could represent characters in the language. This is problematic becuase with 
// all these different encoding standards, how does a program know which standard to use when
// parsing a collection of bytes?
// To solve this problem UNICODE was creted. It is Universal Character Set, meaning that it 
// represents characters from all the well-known languages (also emoji).
// Another good thing of UNICODE is that it is backwards complatible with ASCII because the first 
// 128 symbols of unicode are ascii characters
// So we can use unicode encoding to parse ASCII text
// UTF-8 is a variable with character encoding for unicode. Variable width because each 
// character in UTF-8 could be represented as 1, 2, 3 or 4 bytes. This is very important, remember
// that in ASCII each character is represented by on byte but with UTF-8, each character could be
// a different size in terms of bytes. UTF-8 is the most popular encoding of unicode

use std::collections::HashMap;

fn main() {
  create_new_strings();
  appending_string();
  indexing_into_string();
  hash_map();
  update_hashmap();
  count_words();
}

fn count_words() {
  let text = "hello world rust world not rust";

  let mut counter = HashMap::new();

  for word in text.split_whitespace() {
    let count = counter.entry(word).or_insert(0);
    // Get the hashmap value pointer as a mutable reference and dereference again adding 1
    *count += 1;
  }

  println!("{:?}", counter);

}

fn update_hashmap() {
  let mut scores = HashMap::new();

  scores.insert(String::from("orange"), 10);
  scores.insert(String::from("orange"), 20);

  scores.entry(String::from("Yellow")).or_insert(30);
  scores.entry(String::from("Yellow")).or_insert(40);

}

// Allow us to store key,value pairs. Also is hashing function to determine how to
// place that pairs in the memory
fn hash_map() {
  // Bring to the scope hashmap
  let orange = String::from("orange");
  let white = String::from("white");

  let mut scores = HashMap::new();

  // Move the ownership to the hashmap keys (orange, white)
  scores.insert(orange, 10);
  scores.insert(white, 21);

  // Ownership lost
  //println!("Lost ownership {}", orange);

  let team_name = String::from("blue");
  // We get an Option type because we cannot guarantee that the key exists
  let team = scores.get(&team_name);

  // We get back a tuple 
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}

fn indexing_into_string() {
  let hello = String::from("RUst");
  //let c = hello[0];
  //https://youtu.be/Zs-pS-egQSs?t=886
}

fn appending_string() {
  // Similar vector
  let mut s = String::from("foo");
  s.push_str("bar");
  s.push('!');
  let s1 = String::from("Hello");
  let s2 = String::from("Rust");
  // Move the ownership from s1 to s3 and all the characters of s2
  // This save a little bit of memory in contrast to 'copy'
  let s3 = s1 + &s2;
  let s4 = String::from("Hello");
  let s5 = String::from("Rust");
  // Use format macro to create new string
  let s6 = format!("{}{}", s4, s5);
}

fn create_new_strings() {
  let s1 = String::new();
  // String slice
  let s2 = "Initial contents";
  let s3 = s2.to_string();
  let s4 = String::from("initial contents");
}