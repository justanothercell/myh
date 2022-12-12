use myh::{PrimVec, Serializable};

fn main() {
    let data = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
    let serialized = data.serialize().to_string();
    println!("{serialized}");
    // - "a"
    // - "b"
    // - "c"
    // - "d"
    let serialized = PrimVec::from(data).serialize().to_string();
    println!("{serialized}");
    // "a", "b", "c", "d"
}