use myh::{Myh, Serializable};

#[derive(Debug)]
struct Test {
    a_bool: bool,
    b_i32: i32,
    c_sub: SubStruct
}

impl Serializable for Test {
    fn serialize(&self) -> Myh {
        let mut myh = Myh::new_map();
        myh.set("a_bool", &self.a_bool);
        myh.set("b_i32", &self.b_i32);
        myh.set("c_sub", &self.c_sub);
        myh.myh::<()>(None)
    }

    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized {
        let (_, mut myh) = myh.map::<()>()?;
        Some(Self{
            a_bool: myh.get("a_bool")?,
            b_i32: myh.get("b_i32")?,
            c_sub: myh.get("c_sub")?,
        })
    }
}

#[derive(Debug)]
struct SubStruct {
    title: String,
    sub_string: String,
    sub_tuple: (i32, char)
}

impl Serializable for SubStruct {
    fn serialize(&self) -> Myh {
        let mut myh = Myh::new_map();
        myh.set("sub_string", &self.sub_string);
        myh.set("sub_tuple", &self.sub_tuple);
        myh.myh(Some(&self.title))
    }

    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized {
        let (name, mut myh) = myh.map()?;
        Some(Self{
            title: name?,
            sub_string: myh.get("sub_string")?,
            sub_tuple: myh.get("sub_tuple")?,
        })
    }
}

fn main() {
    let data = Test {
        a_bool: true,
        b_i32: 42,
        c_sub: SubStruct {
            title: "Data 1".to_string(),
            sub_string: "ABCDEF".to_string(),
            sub_tuple: (75, 'c'),
        },
    };
    let serialized = data.serialize();
}