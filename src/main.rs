use std::io;

use quicksort as qs;

mod quicksort;

fn main() {
    let mut to_sort = get_user_input_numbers();

    if to_sort.len() == 0 {
        println!("No data input!...");
        return;
    }

    qs::sort(&mut to_sort);
    println!("\x1b[93mSorted: {:?}", to_sort);
}

fn get_user_input_numbers() -> Vec<i32> {
    println!("Enter the integers one after the other:");

    let mut input_numbers = String::new();
    io::stdin()
        .read_line(&mut input_numbers)
        .expect("Failed to read line!");

    let input_numbers: Vec<i32> = input_numbers
        .trim()
        .split_whitespace()
        .filter_map(|number| match number.trim().parse() {
            Ok(n) => Some(n),
            Err(_) => {
                println!("'{}' is invalid number!", number);
                None
            }
        })
        .collect();

    input_numbers
}
