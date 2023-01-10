// URL: https://exercism.org/tracks/rust/exercises/magazine-cutout

#![allow(unused)]
use std::collections::HashMap;

//URL: https://exercism.org/tracks/rust/exercises/magazine-cutout

pub fn create_words_hash_map(list: &[&str]) -> HashMap<String, u32> {
    let mut words_counter_list : HashMap<String, u32> = HashMap::new();

    for word in list.iter() {
        let count = words_counter_list.entry(word.to_string()).or_insert(0);
        // Get the hashmap value pointer as a mutable reference and dereference again adding 1
        *count += 1;
    }
    words_counter_list
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = create_words_hash_map(magazine);
    let note_words = create_words_hash_map(note);
    
    for (word, times) in note_words {
        match magazine_words.get(&word) {
            Some(x) => {
                if *x < times {
                    return false;
                }
            },
            None => return false
        }
        
    }
    true
}

fn main() {
    let magazine = "two times three is not four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "two times two is four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));

    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(can_construct_note(&magazine, &note));
}