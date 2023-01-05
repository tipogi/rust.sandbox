#![allow(unused)]
fn main() {
    test_window_method(3);
    test_any_method();
    compare_two_slices_with_windows_method();
    raw_comparison_of_slices();
    get_slice_from_slice();
}

fn get_slice_from_slice() {
    let array_of_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let other_list = [ 2, 3, 4];
    let slice_chunk = &array_of_numbers[1..4];
    println!("From 1 to 3 index: {:?}", slice_chunk);
    if other_list == array_of_numbers[1..4] {
        println!("Equal comparison of slices!")
    }
}

fn raw_comparison_of_slices() {
    let first_slice = [1, 2, 3];
    let second_slice = [1, 2, 3];
    if first_slice == second_slice {
        println!("RAW: Equal slices");
    } else {
        println!("RAW: Not equal slices");
    }

    println!("RAW: Equal two slices? {}", first_slice.eq(&second_slice));
}

fn compare_two_slices_with_windows_method() {
    let first_list = [1, 2, 3, 4, 5, 6, 7, 8 ,9];
    let second_list = [4, 3, 4];
    let mut windows = first_list.windows(second_list.len());
    let superlist = windows.any(|x| {
        println!("x value: {:?}", x);
        x == second_list
    });
    println!("Is it a superlist? {}", superlist);
}

fn test_window_method(split: usize) {
    let slice = ['r', 'u', 's', 't', 'y', 'k', 'r', 'u', 's', 't', 'y'];
    let iter = slice.windows(split);
    for window in iter {
        println!("{:?}", window);
    }
}

fn test_any_method() {
    let slice2 = ['r', 'u', 's', 't'];
    let iter2 = slice2.iter().any(|&x|  {
        println!("Element: {}", x);
        x == 's'
    });
    println!("Has char: {}", iter2);
    println!("Length of iter2: {}", slice2.len());
}