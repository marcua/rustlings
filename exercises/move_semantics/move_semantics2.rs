// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// This was quite helpful in understanding move, immutable borrow, and mutable borrow:
// https://users.rust-lang.org/t/rust-mutability-moving-and-borrowing-the-straight-dope/22166
// Notably: "Rust only allows one mutable reference to data in scope at any given time, period, no questions asked."
// The post doesn't explain why, but I imagine that this allows simpler (no) GC via reference counting, which causes unpredictable performance.

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_new_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);
    
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    fill_orig_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_new_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_orig_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
