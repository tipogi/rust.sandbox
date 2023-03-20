// LINK: https://www.youtube.com/watch?v=rAl-9HwD858

//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'a> {
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
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        // We do not want to move self.reminder, we just want the reference, thats why we use ref
        // &mut &'a str = Option<&'a str>
        // We add mut because we want to mutate that value with *
        // So ref means that we are matching into a reference. Like we want a reference to the thing 
        // we are matching rather than the thing we are matching itself
        // Similarly mut ref means, we want to get a mutable reference (mut ref) to the thing we are 
        // matching (&str - reminder) rather than get the thing we are matching
        // What about if we will write: &mut remainder
        // This does the opposite, it says take what the right hand side is and try to match it against
        // this pattern. So, the mutable reference here is a part of the pattern(&mut remainder). 
        // So in that case, the matching will bind just the str as reminder because it will do the 
        // dereferencing and this is wrong
        // But if we set as ref mut then the reminder is going to be a mutable reference to that element 
        // in that case reminder
        // CHECK: https://stackoverflow.com/questions/34717001/whats-the-difference-between-ref-and-when-assigning-a-variable-from-a-referen
        // https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html
        // This one also would be equal if let Some(reminder) = &mut self.reminder 
        // For Jon, it does not like that way...
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                // The type of reminder here is &mut &'a str, as we explain above, and the right side is &'a str
                // We cannot assign something like this because they are not the same 
                // type. So we need to dereference because we want to assign this into where 
                // reminder is pointing
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                // Takes the value if it is Some and return as an Option
                // and lets in the old Option None
                // If it is None, it returns None
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[test]
fn create_str_split()   {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
// The last element has to be empty because after d we have the delimiter
// So technically it should produce element
fn tail()   {
    let haystack = "a b c d ";
    let letters:Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""])
}