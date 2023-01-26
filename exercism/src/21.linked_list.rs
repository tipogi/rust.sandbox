use std::{iter::FromIterator};

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self {
            data,
            next
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    // It does not have elements, the head value has to be None
    pub fn new() -> Self {
        Self {
            head: None
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut size: usize = 0;
        let mut next = self.head.as_ref();
        while let Some(node) = next {
            size += 1;
            next = node.next.as_ref();
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        // in self.head will be None and in next, the Option
        let next = self.head.take();
        self.head = Some(Box::new(Node::new(_element, next)))
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take()?;
        self.head = head.next.take();
        Some(head.data)
    }

    pub fn peek(&self) -> Option<&T> {
        // We cannot get the ownership from a borrowed variable
        // Thats why we need as_ref
        // == Correct-A ==
        // match &self.head/*.as_ref()*/ {
        //    Some(k) => Some(&k.data),
        //    _ => None
        //}
        // == Correct-B ==
        //Some(&self.head.as_ref()?.data)
        // == Correct-C ==
        // let node = self.head.as_ref()?;
        // Some(&node.data)
        let node = self.head.as_ref();
        Some(&node?.data)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reverse = Self::new();
        loop {
            let next = self.pop();
            //if next.is_none() {
            if let None = next {
                break;
            }
            reverse.push(next.unwrap());
        }
        reverse
    }
}

// Create from a iterator, a new LinkedList
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push(item)
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        self.rev().into_iter().collect()
    }
}

// Get the ownership of the simple list iterator
impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IteratorSimpleLinkedList<T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            head: self.head
        }
    }
}



pub struct IteratorSimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

// Create a custom iterator to not mutate the original
// and to duplicate as match as we want the list
impl <T> Iterator for IteratorSimpleLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.data)
    }
}

pub fn main() {
    let mut linked_list = SimpleLinkedList::<u32>::new();
    println!("{:?}", linked_list.len());
    linked_list.push(34);
    linked_list.push(21);
    linked_list.push(2);
    println!("{:?}", linked_list);
    println!("{:?}", linked_list.len());
    //let pop_a = linked_list.pop();
    //let pop_b = linked_list.pop();
    println!("LinkedList: {:?}", linked_list.rev());
    let greeting = vec!["hello", "bye", "morning", "night"];
    //let second_linked_list = SimpleLinkedList::from_iter(greeting);
    //println!("{:?}", second_linked_list);
    let mut string_linked_list = SimpleLinkedList::new();
    string_linked_list.push("hello");
    string_linked_list.push("bye");
    string_linked_list.push("morning");
    string_linked_list.push("night");
    //let greeting_linked_list = greeting.into();
    //assert_eq!(string_linked_list, greeting_linked_list);
    

}