// URL: https://exercism.org/tracks/rust/exercises/anagram
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let mut word_in_chars: Vec<char> = word_lowercase.chars().collect();
    word_in_chars.sort_unstable();
    let mut result: HashSet<&str> = Default::default();
    for anagram in possible_anagrams {
        let anagram_lowercase = anagram.to_lowercase();
        if word == anagram_lowercase {
            continue;
        }
        let mut anagram_in_char: Vec<char> = anagram_lowercase.chars().collect();
        anagram_in_char.sort_unstable();
        // Because vector implements PartialEq trait, we can compare vectors
        if word_in_chars == anagram_in_char {
            result.insert(anagram);
        }
    }
    result
}

fn main() {
    let word = "mozseib";
    let possible_anagrams = ["hello", "world", "zombies", "pants"];
    let res = anagrams_for(word, &possible_anagrams);
    println!("{:?}", res);
    println!("{}", vec!["hello", "may"] == vec!["hello", "may"]);
}