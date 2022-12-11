use myh::{AnyRange, Primitive};

fn main() {
    let t: (i32, i32, f32, AnyRange<i32>, char, &str) = Primitive::from_string("1, 2, 3, 4..10, 'c', \"foo!\"").unwrap();
    println!("{t:?}");  // (1, 2, 3.0, 4..10)
}