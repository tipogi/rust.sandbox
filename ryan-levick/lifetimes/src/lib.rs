
// LINK: https://www.youtube.com/watch?v=MSi3E5Z8oRw
// GITHUB: https://gist.github.com/rylev/3a3dd4b0d8563eb2267f489e559bb70e
// We tied the MyInmutableIterator lifetime to the lifetime of the slice that is inside of it.
// The MyInmutableIterator lifetime is valid until the slice is valid
// In that case, we can have many non-exclusive references pointing that slice, because 
// it is inmutable
// So however long slice lives for MyInmutableIterator will live for. They are tied together
#[derive(Debug)]
struct MyInmutableIterator<'a, T> {
    slice: &'a[T],
}

// 14:05, if we do not write before Iterator the generic types
// rust will try to find the parameters that are in MyInmutableIterator
// but is not going to find what are that parameters in the scope
// So first declare the generics (a' and T) and after use
impl<'a, T> Iterator for MyInmutableIterator<'a, T> {
    // The item will be valid until MyInmutableIterator is not destroyed
    // When it iterates, it's going to yield out elements that are references
    // to elements that lives for however long MyInmutableIterator lives for
    // As above, we cannot have an element that lives for longer than MyInmutableIterator
    // lives for, they are tied together, they have the same lifetime
    type Item = &'a T;

    // here we do not have below next function problem because we are borrowing
    // inmutable elements. With inmutable references you can have as many as you 
    // want
    // CHECK: 57:00
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        /*if self.slice.is_empty() {
            return None;
        }*/
        let (element, rest) = self.slice.split_first()?;// or .unwrap();
        self.slice = rest;
        Some(element)
    }
}

// Here the iterator is exclusive reference because one we call the iterator function
// the slice cannot be shared with other references
struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T]
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    // The lifeme of item is as long as the iterator lifes 
    type Item = &'iter mut T;

    // Can we try to borrow an element for however long iter lasts
    // or in other way for a time that matches the amount of time that 
    // iter last for
    // So we need to disconnect the first element from our slice and saying
    // I don't neet to borrow this thing for however long the next function lasts for,
    // I need to borrow this thing for however long the slice inner slice itself is 
    // borrowed for
    // To solve that ryan has a trick: std::mem::replace.
    // Temporarily replace self.slice with a dummy variable in order to
    // get kind of full access to the thing that self.slice was previously
    // set to
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        // First we shadow variables, if not we can write slice_a and slice_b but using shadowing we achieve that
        // lets continue...
        // Here we borrow somthing that is borrowing, double borrow
        // And thats what we don't want we want full access
        let slice = &mut self.slice;
        // And that full access gives mem::replace
        // We say to rust: Hey! set this thing to an empty list and let me have full access to it
        // so I can do whatever I want with it
        // In other way, we are changing that actual pointer (that lives inside the of the slice field)
        // for another pointer that is an empty list
        // In that case we are owning the reference
        let slice = std::mem::replace(slice, &mut []);
        // Here we have a full access to the previous pointer of the slice which it was pointing
        // to all of our elements and we can do what ever
        let (first, rest) = slice.split_first_mut().unwrap();
        self.slice = rest;
        Some(first)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inmutable_test() {
        // Vector variable is in the stack, the elements in the heap, all the array
        let collection = vec![1, 2, 3, 4];
        let wrapper = MyInmutableIterator {
            slice: &collection[..],
        };
        for element in wrapper {
            println!("Element value: {}", element)
        }
    }

    #[test]
    fn mutable_test() {
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };
        for (index,elem) in wrapper.enumerate() {
            *elem += 1;
            println!("Element value: {}", elem);
            // We cannot write (above operation elem *= 1) and read (collection[index]) at the same time
            // because if we have a mutable variable that the rule
            // It is the same in the database, you can write just one, other operations lock
            // But you can read as many
            //assert_eq!(*elem, collection[index]);
        }
        assert_eq!(Some(&2), collection.get(0));
    }
}
