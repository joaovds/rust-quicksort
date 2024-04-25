use quicksort as qs;

mod quicksort;

fn main() {
    let mut to_sort = [9, 491, 23, 434, 533, 2, 28, 98, 1, 37, 68, 1, 355];

    qs::sort(&mut to_sort);
    println!("\x1b[93mSorted: {:?}", to_sort);
}
