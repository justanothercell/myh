use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::error::MyhError;
use crate::parsing::{key_index, validate_key};
use crate::Primitive;

pub trait Serializable {
    fn serialize(&self) -> Result<Myh, MyhError>;
    fn deserialize(myh: &Myh) -> Result<Self, MyhError> where Self: Sized;
}

impl<T: Primitive> Serializable for T {
    fn serialize(&self) -> Result<Myh, MyhError> {
        let mut myh = Myh::new();
        myh.set_item(Some(self));
        Ok(myh)
    }

    fn deserialize(myh: &Myh) -> Result<Self, MyhError> where Self: Sized {
        myh.get_item()
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Result<Myh, MyhError> {
        let mut myh = Myh::new();
        for item in self.iter() {
            myh.push(item)?;
        }
        Ok(myh)
    }

    fn deserialize(myh: &Myh) -> Result<Self, MyhError> where Self: Sized {
        let mut v = vec![];
        for i in 0..myh.len() {
            v.push(myh.at(i)?)
        }
        Ok(v)
    }
}

pub struct Myh {
    item: String,
    order: MyhOrder,
    list: Vec<Myh>,
    map: HashMap<String, Myh>,
    map_order: Vec<String>
}

#[derive(PartialEq)]
enum MyhOrder {
    None,
    ListFirst,
    MapFirst
}

impl MyhOrder {
    fn update(&mut self, order: MyhOrder) {
        if let MyhOrder::None = self {
            *self = order;
        }
    }
}

impl Myh {
    pub fn new() -> Self {
        Self {
            item: String::new(),
            order: MyhOrder::None,
            list: vec![],
            map: Default::default(),
            map_order: vec![],
        }
    }
    pub fn extend(&mut self, other: Myh) {
        let Self {
            item,
            order,
            list,
            map,
            map_order,
        } = other;
        if let MyhOrder::None = self.order {
            self.order = order;
        }
        if !item.is_empty() {
            if self.has_item() {
                self.item = format!("{}, {}", self.item, item);
            } else {
                self.item = item;
            }
        }
        self.list.extend(list.into_iter());
        self.map.extend(map.into_iter());
        self.map_order.extend(map_order.into_iter());
    }

    // == item ==
    pub fn has_item(&self) -> bool {
        !self.item.is_empty()
    }
    pub fn get_item<T: Primitive>(&self) -> Result<T, MyhError>{
        T::from_string(&self.item)
    }
    pub fn no_title(&self) -> Result<(), MyhError> {
        let _: () = self.get_item()?;
        Ok(())
    }
    pub fn set_item<T: Primitive>(&mut self, item: Option<&T>) {
        self.item = item.map(|i|i.stringify()).unwrap_or(String::new())
    }

    // == list ==
    pub fn push<T: Serializable>(&mut self, item: &T) -> Result<(), MyhError> {
        self.order.update(MyhOrder::ListFirst);
        self.list.push(item.serialize()?);
        Ok(())
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn at<T: Serializable>(&self, index: usize) -> Result<T, MyhError> {
        if self.list.len() <= index {
            return Err(MyhError::IndexOutOfBounds(index, self.list.len()))
        }
        T::deserialize(self.list.get(index).unwrap())
    }

    // == map ==
    pub fn set<T: Serializable>(&mut self, key: &str, item: &T) -> Result<(), MyhError>{
        validate_key(key)?;
        self.order.update(MyhOrder::MapFirst);
        self.map.insert(key.to_string(), item.serialize()?);
        self.map_order.push(key.to_string());
        Ok(())
    }
    pub fn has_key(&self, key: &str) -> bool{
        self.map.contains_key(key)
    }
    pub fn get<T: Serializable>(&self, key: &str) -> Result<T, MyhError>{
        self.map.get(key).map(|myh|T::deserialize(myh)).ok_or(MyhError::KeyNotFound(key.to_string()))?
    }
    pub fn keys(&self) -> Keys<String, Myh>{
        self.map.keys()
    }

    // == conversion ==
    pub fn deserialize<T: Serializable>(&self) -> Result<T, MyhError>{
        T::deserialize(self)
    }

    pub fn load(path: &str) -> Result<Self, MyhError> {
        let mut f = File::open(path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        Self::from_string(&buf)
    }
    pub fn from_string(string: &str) -> Result<Self, MyhError>{
        let strings = string.split('\n').collect::<Vec<_>>();
        let strings = strings.into_iter().map(|s|s.trim_end()).collect();
        Self::from_strings(strings)
    }
    fn from_strings(mut strings: Vec<&str>) -> Result<Self, MyhError>{
        fn collect_item<'a>(strings: &mut Vec<&'a str>) -> Vec<&'a str>{
            let mut sub = vec![];
            while let Some(s) = strings.get(0) {
                if s.starts_with("    ") {
                    sub.push(s.split_at(4).1);
                    let _ = strings.remove(0);
                } else { break }
            }
            sub
        }
        let mut myh = Self::new();
        while strings.len() > 0 {
            let s = strings.remove(0);
            if s.trim().is_empty() { continue }
            if s.starts_with("- ") {
                let mut item = vec![s.split_at(2).1.trim_start()];
                item.extend(collect_item(&mut strings));
                myh.list.push(Self::from_strings(item)?);
                myh.order.update(MyhOrder::ListFirst);
            } else if let Some(i) = key_index(s) {
                let (k, it) = s.split_at(i + 1);
                let k = k.split_at(k.len()-1).0;
                validate_key(k)?;
                let mut item = vec![it.trim_start()];
                item.extend(collect_item(&mut strings));
                myh.map.insert(k.to_string(), Myh::from_strings(item)?);
                myh.map_order.push(k.to_string());
                myh.order.update(MyhOrder::MapFirst);
            } else if myh.order == MyhOrder::None && !myh.has_item() {
                myh.item = s.to_string();
            } else { return Err(MyhError::DeserializationError("invalid input".to_string(), s.to_string())) }
        }
        Ok(myh)
    }

    pub fn save(&self, path: &str) {
        let mut f = File::create(path).unwrap();
        let _ = f.write(self.to_string().as_bytes());
    }
    pub fn to_string(&self) -> String{
        self.stringify(0)
    }
    fn stringify(&self, indent: usize) -> String{
        let item = &self.item;
        let mut list = String::new();
        for item in &self.list{
            list = list + &format!("\n{}- ", "    ".repeat(indent)) + &item.stringify(indent + 1);
        }
        let mut map = String::new();
        for key in &self.map_order {
            let v = self.map.get(key).unwrap();
            map = map + &format!("\n{}{}: ", "    ".repeat(indent), key) + &v.stringify(indent + 1);
        }
        if let MyhOrder::ListFirst = self.order{
            format!("{item}{list}{map}")
        } else if let MyhOrder::MapFirst = self.order {
            format!("{item}{map}{list}")
        } else {
            item.to_string()
        }
    }
}

