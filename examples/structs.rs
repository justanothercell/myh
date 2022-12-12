use myh::{Myh, PrimVec, Serializable};
use myh::error::MyhError;

#[derive(Debug)]
struct Test {
    a_bool: bool,
    b_i32: i32,
    c_sub: SubStruct,
    d_vec: Vec<f32>,
    e_coords_vec: Vec<(String, Coord)>
}

impl Serializable for Test {
    fn serialize(&self) -> Result<Myh, MyhError> {
        let mut myh = Myh::new();
        myh.set("a_bool", &self.a_bool)?;
        myh.set("b_i32", &self.b_i32)?;
        myh.set("c_sub", &self.c_sub)?;
        myh.set("d_vec", &self.d_vec)?;
        myh.set("e_coords_vec", &self.e_coords_vec.iter().map(|(n, c)| SaveCoord { name: n.clone(), x: c.x, y: c.y }).collect::<Vec<_>>())?;
        Ok(myh)
    }

    fn deserialize(myh: &Myh) -> Result<Self, MyhError> {
        myh.no_title()?;
        Ok(Self{
            a_bool: myh.get("a_bool")?,
            b_i32: myh.get("b_i32")?,
            c_sub: myh.get("c_sub")?,
            d_vec: myh.get("d_vec")?,
            e_coords_vec: myh.get::<Vec<SaveCoord>>("e_coords_vec")?.into_iter().map(|sc| (sc.name, Coord{ x: sc.x, y: sc.y })).collect::<Vec<_>>()
        })
    }
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

struct SaveCoord {
    name: String,
    x: i32,
    y: i32,
}

impl Serializable for SaveCoord {
    fn serialize(&self) -> Result<Myh, MyhError> {
        let mut myh = Myh::new();
        myh.set_item(Some(&self.name));
        myh.set("x", &self.x)?;
        myh.set("y", &self.y)?;
        Ok(myh)
    }

    fn deserialize(myh: &Myh) -> Result<Self, MyhError> {
        Ok(Self{
            name: myh.get_item()?,
            x: myh.get("x")?,
            y: myh.get("y")?,
        })
    }
}

#[derive(Debug)]
struct SubStruct {
    title: String,
    sub_string: String,
    sub_tuple: (i32, char),
    inline_vec: Vec<char>,
    outline_vec: Vec<String>,
    toplevel_vec: Vec<String>
}

impl Serializable for SubStruct {
    fn serialize(&self) -> Result<Myh, MyhError> {
        let mut myh = Myh::new();
        myh.set_item(Some(&self.title));
        myh.set("sub_string", &self.sub_string)?;
        myh.set("sub_tuple", &self.sub_tuple)?;
        myh.set("inline_vec", &PrimVec::from(self.inline_vec.clone()))?;
        myh.set("outline_vec", &self.outline_vec)?;
        myh.extend(self.toplevel_vec.serialize()?);
        Ok(myh)
    }

    fn deserialize(myh: &Myh) -> Result<Self, MyhError> {
        Ok(Self{
            title: myh.get_item()?,
            sub_string: myh.get("sub_string")?,
            sub_tuple: myh.get("sub_tuple")?,
            inline_vec: myh.get::<PrimVec<_>>("inline_vec")?.into(),
            outline_vec: myh.get("outline_vec")?,
            toplevel_vec: myh.deserialize()?,
        })
    }
}

fn main() {
    /*
    let data = Test {
        a_bool: true,
        b_i32: 42,
        c_sub: SubStruct {
            title: "Data 1".to_string(),
            sub_string: "ABCDEF".to_string(),
            sub_tuple: (75, 'c'),
            inline_vec: vec!['x', 'y', 'z'],
            outline_vec: vec!["a good reason to put this on multiple lines".to_string(),
                              "yk this is rather long".to_string(),
                              "and really really really bothersome to read in one line".to_string()],
            toplevel_vec: vec!["a".to_string(), "toplevel".to_string(), "string".to_string()],
        },
        d_vec: vec![1f32, 2.7f32, std::f32::consts::PI, -4f32],
        e_coords_vec: vec![("marc".to_string(), Coord { x: 0, y: 0 }),
                           ("fedora".to_string(), Coord { x: 1, y: 2 }),
                           ("elmo".to_string(), Coord { x: 3, y: 2 })]
    };
    let serialized = data.serialize().unwrap();
    println!("{}", serialized.to_string());
    serialized.save("test_files/example.myh");
    */
    let s: Test = Myh::load("test_files/example.myh").unwrap().deserialize().unwrap();
    println!("{s:?}");
}