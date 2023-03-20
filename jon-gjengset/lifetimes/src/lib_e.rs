// LINK: https://www.youtube.com/watch?v=rAl-9HwD858

//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'a> {
    // 2...We kind of understand where Rust is comming from here
    // because we say that there is just one lifetime...
    remainder: Option<&'a str>,
    delimiter: &'a str
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    // 3... and the thing that returned from the iterator has the same lifetime
    type Item = &'a str;
    
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

fn until_char(s: &str, c: char) -> &str { // 58:00
    // 1.We know that StrSplit only ever returns some strings of s string, the first argument,
    // the haystack. It never returns references into the second string.
    // The lifetime of the second string does not matter for the purposes of what store split
    // returns 
    // But if we look to our definition...
    // 4... and so when rust sees that, it says that these two things has the same lifetime
    // So as we can see here, this two elements have different lifetimes:
    // right one (c) has the lifetime that is only the scope of this function whereas
    // the left one (s), has whatever the lifetime of the s is. 
    // So what rust does, in order to make them the same, it takes the longer lifetime and
    // turns it into the shorter lifetime. So the 'a for this StrSplit is gonna be the lifetime
    // of 'format!("{}", c) scope.
    // So when we try to return a reference, that reference has a lifetime tied to the scope
    // of this function. But what we want in the funtion is that the string (s) has the
    // lifetime of the returned &str
    // To agree in that contract, we need to change the lifetime of the struct
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