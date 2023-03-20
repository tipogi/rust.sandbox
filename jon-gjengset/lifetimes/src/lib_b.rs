// LINK: https://www.youtube.com/watch?v=rAl-9HwD858

//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str
}

impl<'a> StrSplit<'a> {
    // 26:55: The Self return it will have '_ lifetime
    // If we do not put any lifetime the the new arguments
    // rust compiler does not know which are the lifetimes
    // of the returned Self arguments. So, when it will return
    // the struck inmediately, reminder and delimiter will be dropped
    // and our StrSplit will not be available also
    // So, it has to be some relationship between the arguments lifetime and 
    // the lifetime of the Struck itself
    // As long as StrSplit is available, make sure also these strings (arguments ones) are
    // also available through the pointers we were given
    // How can we express that:
    // I can give you a str split with a lifetime, 'a, if you give me string pointers
    // that are also 'a
    // Here we are saying, the pointers you give me in, they can live for however long you
    // want but they have to live for at least some duration, 'a, and the type I give you back
    // has a lifetime that is the same as that lifetime.
    // The compiler is now going to check that as you can only keep using this, as long as that 
    // lifetime is still the life. Which implies by the fact that is connected to this lifetime
    // you can only keep using the strSplit for as long as the input string are still valid.
    // The impl<'a> means that is generic over lifetime 'a
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Check if the delimiter exist
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            // Any string that we write in double quotes, is compiled into the binary, it's stored in the program
            // that's stored on disk. And when the program is launched, the OS is gonna load that binary
            // into the memory and anything that's a value written into the binary is in sort of
            // read-only memory that will never move. So, if you take a pointer to it which effectively what it
            // does behind the scenes. It takes a pointer into a particular segment of your program then
            // that reference naturally lives for the rest of your program. That pointer is always going to be 
            // valid because that part of your programs memory never changes
            self.remainder = "";
            // Why is it correct that because we define self.reminder has &'a lifetime and 
            // the value that we are assigning has &'static lifetime?
            // &'a = &'static
            // Static lifetime is the lifetime that sticks until the end of the program
            // Think as basically never ends
            //
            // This is where subtyping relationship comes in.
            // So, if you have any lifetime, you can assign to it, if you have a reference of any
            // lifetime or the thing that contains any lifetime. You can assign to it anything of the
            // same type but a longer lifetime.
            // The reason of this is sort of straightforward, if I need something that lives for at least
            // 'a then some other lifetime that's longer than 'a, trivially can be reduced to that description
            Some(rest)
        }
    }
}

#[test]
fn create_str_split()   {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    // OR
    // let letters:Vec<_> = StrSplit::new(haystack, " ").collect();
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"])
}