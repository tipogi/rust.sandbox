// GITHUB: https://github.com/rust-lang/rust/blob/master/library/alloc/src/raw_vec.rs

use std::ptr::NonNull;
use std::alloc;

pub struct MyVec<T> {
    // pointer: https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl <T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(),  0, "No zero sized types");
        // The same as above
        /*if std::mem::size_of::<T>() == 0 {
            panic!("No zero sized types");
        }*/
        // Initialise the pointer. https://doc.rust-lang.org/stable/std/alloc/index.html
        if self.capacity == 0 {
            // Reserve 4 size capacity for new MyVec (bytes)
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate");
            // alloc expects that layout is describing a piece of memory that's bigger than zero bytes
            // And we are guaranteeing that doesn't happen. https://doc.rust-lang.org/nightly/core/alloc/trait.GlobalAlloc.html#tymethod.alloc
            // SAFETY TIP: The layout is harcoded to be 4 * size_of<T> and size_of<T> is > 0
            // Cast the pointer and initialise a pointer(*) to a T
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            // We do shadowing here
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");
            // Convert a NotNull pointer to raw pointer (and 3 more)
            // We now have enough space in the heap so, write in that space our item
            // SAFETY TIP: ptr in non-null and we have just allocated enough space for the item. The
            // memory previously at ptr is not read
            // https://doc.rust-lang.org/nightly/core/ptr/fn.write.html
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            unsafe { 
                let offset = self
                    .len
                    .checked_mul(std::mem::size_of::<T>())
                    .expect("Cannot reach memory location");
                assert!(offset < isize::MAX as usize, "Wrapped isize");
                // With the above code, we achieve: Offset cannot wrap around and pointer is pointing to valid memory
                // And writing to an offset at self.len is valid

                // OFFSET (wiki def): an offset within an array or other data structure object is an integer
                // indicating the distance (displacement) between the beginning of the object and a given element
                // or point, presumably within the same object
                // 'add' calculates the offset of the pointer. IMportant that count in units of T
                // Resuming with add we skip ahead the vector len
                unsafe { 
                    self.ptr
                        // Move from NonNull pointer to raw pointer
                        .as_ptr()
                        .add(self.len)
                        .write(item) 
                    }
                self.len += 1;
            }
        } else { // 1:08:00 -> Implementing Rust's Vec From Scratch
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity");
            // ALIGN: Every type has a memory addresses where it needs to be because the processor expects some
            // chunks of memory to be a certain offset of memory. i.e: If we have a type of alignment of 2
            // for instance, it must always start at a memory address that is power of, that it can be divided
            // evenly by two. If we have a type of aligment of 16 that type must always be at a memory address
            // that is evenly divisible by 16 
            // The method returns the aligment over Type is
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Cannot allocate");
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(
                    self.ptr.as_ptr() as * mut u8,
                    layout,
                    new_size
                );
                let ptr = NonNull::new(ptr as *mut T).expect("Could not ");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            }
            self.ptr = ptr;
            self.len = +1;
            self.capacity = new_capacity;
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}


#[cfg(test)]
mod tests {
    // To use MyVec
    use super::*;
    #[test]
    fn it_works() {
        //let mut vec: MyVec<usize> = MyVec::new();
        // The same as above
        let mut vec = MyVec::<usize>::new();


        
        /*let mut vec = Vec::new();
        // Define here the vector elements types adding explicit type
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        // Number of elements could feet in the heap, the space that occupies in the memory
        // without reallocation. In that case, we could push 3 more elements without causing 
        // reallocation
        assert_eq!(vec.capacity(), 8);
        // ELements that has the vector
        assert_eq!(vec.len(), 5);*/
    }
}


// layout, align, size, capacity