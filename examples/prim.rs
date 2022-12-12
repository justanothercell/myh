use myh::{AnyRange, Primitive};

fn main() {
    let t: (i32, i32, (f32, AnyRange<i32>, char), Option<u8>, Option<i8>, Option<i8>, Option<i8>) =
        Primitive::from_string("1, 2, (3, 4..10, 'c'), 0, -1").unwrap();
    println!("{t:?}");  // (1, 2, (3.0, 4..10, 'c'), Some(0), None)
    let some: Option<u8> = Primitive::from_string("111").unwrap();
    println!("some: {some:?}");
    let none: Option<u8> = Primitive::from_string("").unwrap();
    println!("none: {none:?}");
}