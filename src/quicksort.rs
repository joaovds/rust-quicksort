pub fn sort(items: &mut [i32]) {
    if items.len() <= 1 {
        return;
    };

    let pivot = partition(items);
    let (left, right) = items.split_at_mut(pivot);
    sort(left);
    sort(&mut right[1..]);
}

fn partition(items: &mut [i32]) -> usize {
    let vec_len = items.len();
    let mut pivot = vec_len / 2;
    items.swap(pivot, vec_len - 1);
    pivot = vec_len - 1;
    let mut small_index = 0;

    for i in 0..vec_len - 1 {
        println!(
            "\x1b[31m{:?} => P: {pivot} | SI: {small_index} | i: {i}",
            items
        );
        if items[i] <= items[pivot] {
            items.swap(small_index, i);
            small_index += 1;
        }
        println!(
            "\x1b[92m{:?} => P: {pivot} | SI: {small_index} | i: {i}",
            items
        );
    }
    items.swap(small_index, pivot);
    println!("\x1b[93m{:?} => P: {pivot} | SI: {small_index}", items);
    println!("----------- ... ------------");

    small_index
}
