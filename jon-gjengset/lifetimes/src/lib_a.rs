// LINK: https://www.youtube.com/watch?v=rAl-9HwD858

//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// There is not way to set a wrong lifetime because the compiler will through the error
// 'static lifetime: A thing that lives for the entire duration of the rest of the program

// We need to set the lifetime because it is a reference and Rust cannot 
// understand the lifetime of each references so, we specify the references
// lifetimes
// So setting the lifetime, we set: If we have StrSplit then the remainder and
// delimiter both live for this long ('a), the pointers are valid for that long
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str
}

// Anonymous lifetime: '_
// Say to the compiler to guess what lifetime and that only works, when there is only
// one possible guess
// Underscore(_) is a pattern that matches everything: lifetimes, types,... not completely true but kind of
// Could be something generic like T
// EXAMPLE
// There is only one other lifetime here, thats the lifetime to self so, the compiler can guess
// what this type is because the compiler understand when we give this ('_) that it must be the
// lifetime
// impl Foo {
//     fn get_ref(&self) -> &'_str {}
// }
// // NO NEED TO GIVE THIS
// impl Foo {
//     fn get_ref<'a>(&'a self) -> &'a str{}
// }


// Q: Can you '_ only get used if there is only possible lifetime? NO
// '_ you can also use it, if you have a function like below 
// Anonymous lifetime '_ (with multiple lifetimes)
// Return str of lifetime x
// fn foo<'x, 'y>(x: &'x str, y: &'y str) -> &'x str {}
// If we have above implementation, we can simplify with the below code
// fn foo<'x, 'y>(x: &'a str, y: &'_ str) -> &'_ str {}
// In which case y lifetime ('_) gets ignored. (y: &'_ str) basically gets turned into an
// arbitrary unique lifetime. In the output position, it means
// type inference (define variable type, in that case defining lifetime definition) 
// basically a lifetime inference and so it's gonna infer (deduce) that: This (the output lifetime)
// must be tied to X but must not be tied to Y because Y has a its own lifetime.

impl StrSplit<'_> {
    // haystack: is usually the thing that we are searching for
    // delimiter: The thing that we are splitting by
    // So basically, I want to split haystack by delimiter
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    // We have to specify how long is going to life the Item. Rust has to know
    // how long it has to keep alive that reference that points to the str content
    // Specifying the lifetime, it knows if it can hold on to that for the end of the lifetime,
    // the end of the program or drop str split and the still use it
    // Not doing that, it has to be a problem because it might use that pointer after the string it's pointing to, it has
    // already gone away after memory has been deallocated
    // And here we say: If the reminder is valid for this long, then the thing that we
    // return has the same lifetime
    // We could imagine other lifetimes
    type Item = &'a str;

    // When we call the next, it returns back a pointer to a string (in that case),
    // because we have (&str) and that one lives in the read-only memory
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
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn createStrSplit()   {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a b c d e"].into_iter());
}