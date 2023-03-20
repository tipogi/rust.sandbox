// LINK: https://www.youtube.com/watch?v=rAl-9HwD858

//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
// Usually WE DO NOT NEED, multiple livetimes
// Sometimes happens, when we have multiple references
pub struct StrSplit<'haystack, 'delimiter> {
    // We need two lifetimes
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
// This block '_ does not care what this lifetime is
//impl<'haystack> Iterator for StrSplit<'haystack, '_> {
    type Item = &'haystack str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello world", 'o'), "Hell");
}

#[test]
fn create_str_split()   {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail()   {
    let haystack = "a b c d ";
    let letters:Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""])
}