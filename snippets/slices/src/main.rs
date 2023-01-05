#![allow(unused)]
fn main() {
    test_window_method(3);
    test_any_method();
    compare_two_slices();
}

fn compare_two_slices() {
    let first_list = [1, 2, 3, 4, 5, 6, 7, 8 ,9];
    let second_list = [2, 3, 4];
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