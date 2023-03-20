// LINK: https://www.youtube.com/watch?v=rAl-9HwD858

//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
// Why self reminder is not mutable?
// The mutable references are only one level deep so, if we have a mute
// self (next function) what that means is you are allowed to modify any
// of the fields of self. So we are allowed to modify the reminder and delimiter
// But what delimiter is, it is an inmutable pointer to some string
// While we can change delimiter itself to point somewhere else, we cannot
// change the thing that the delimiter is pointing to. For that delimiter 
// itself would have to be a mutable reference
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
        // ERROR: let remainder = self.remainder?;
        // If self.reminder is None it return None otherwise 
        // returns the value inside the Some. Normally, that would move
        // the thing that is inside Some but because the thing that is inside
        // reminder option is a copy (beause &), we get copy semanthics instead
        // of move semanthics. So it copies this reference out of the ocean.
        // This means that the remainder is not longer the same remainder as the 
        // one that's in here, it's not a mutable reference, it is just a separete
        // reference pointer. 
        // This means when we modify down here (*reminder) we are just modifying 
        // the copy of that pointer, it's not modifying the pointer that is stored
        // inside self. So for that we need as_mut()
        // With that we will get the mutable reference to the thing that is inside
        // the option. With that the reminder will be mutable reference inside
        // of star split
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