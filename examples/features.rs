use uuid::Uuid;
use myh::Primitive;

fn main() {
    let uuid = Uuid::new_v4();
    let uuid_ser = uuid.stringify();
    println!("uuid_ser: {uuid_ser}");
    let uuid_from_ser: Uuid =  Primitive::from_string(&uuid_ser).unwrap();
    println!("uuid_from_ser: {uuid_from_ser}");
}