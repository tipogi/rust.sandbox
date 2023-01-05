#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


//https://exercism.org/tracks/rust/exercises/sublist
fn main() {
    let a_list = [1, 2, 3, 4];
    let b_list = [1, 2, 3];
    println!("The comparison result was: {:?}", sublist(&a_list, &b_list));
}

fn compare_lists<T: PartialEq>(a_list: &[T], b_list: &[T], a_length: usize, b_length: usize) -> Comparison {
    let a_sublist = b_length >= a_length &&
        b_list
            .windows(a_length)
            .any(|b_chunk| b_chunk == a_list);

    let b_sublist = a_length >= b_length && 
        a_list
            .windows(b_length)
            .any(|a_chunks| a_chunks == b_list);

    match (a_sublist, b_sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal
    }
}

pub fn sublist<T: PartialEq>(a_list: &[T], b_list: &[T]) -> Comparison {
    let a_length = a_list.len();
    let b_length = b_list.len();

    match (a_length, b_length) {
        (0,0)   => Comparison::Equal,
        (0,_)   => Comparison::Sublist,
        (_,0)   => Comparison::Superlist,
        (_, _)  => compare_lists(a_list, b_list, a_length, b_length)
    }    
}