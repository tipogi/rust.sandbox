// URL: https://exercism.org/tracks/rust/exercises/reverse-string
// A grapheme cluster is a sequence of one or more Unicode code
// points that should be treated as a single unit by various processes

// Add in cargo.toml dependcies
// [dependencies]
// unicode-segmentation = "1.10.0"

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input
        // Split the string into an Iterator of &strs, where each element is an
        // extended grapheme cluster.
        .graphemes(true)
        // Reverse the order of the grapheme iterator.
        .rev()
        // Collect all the chars into a new owned String.
        .collect()
}

/// Process a single test case for the property `reverse`
fn process_reverse_case(input: &str, expected: &str) {
    assert_eq!(&reverse(input), expected)
}

fn main() {
    process_reverse_case("robot", "tobor");
    process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    process_reverse_case("uüu", "uüu");
}