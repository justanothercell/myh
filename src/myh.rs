use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::ops::Index;
use crate::Primitive;

pub trait Serializable {
    fn serialize(&self) -> Myh;
    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized;
}

impl<T: Primitive> Serializable for T {
    fn serialize(&self) -> Myh {
        let myh = Myh::new_empty();
        myh.myh(Some(self))
    }

    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized {
        let Myh { item, .. } = myh;
        item.as_ref().map(|s|T::from_string(s))?
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Myh {
        let mut myh = Myh::new_list();
        for item in self.iter() {
            myh.push(item)
        }
        myh.myh::<()>(None)
    }

    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized {
        let (_, mut myh) = myh.list::<()>()?;
        let mut v = vec![];
        while myh.len() > 0 {
            v.push(myh.pop()?)
        }
        v.reverse();
        Some(v)
    }
}

pub struct Myh {
    item: Option<String>,
    collection: MyhCollection
}

pub enum MyhType {
    Empty,
    List,
    Map
}

enum MyhCollection {
    Empty,
    List(Vec<Myh>),
    Map(HashMap<String, Myh>, Vec<String>)
}

impl Myh {
    pub fn new_empty() -> MyhEmpty{
        MyhEmpty
    }
    pub fn new_list() -> MyhList{
        MyhList(vec![])
    }
    pub fn new_map() -> MyhMap {
        MyhMap(HashMap::new(), vec![])
    }
    pub fn myh_type(&self) -> MyhType {
        match &self.collection {
            MyhCollection::Empty => MyhType::Empty,
            MyhCollection::List(_) => MyhType::List,
            MyhCollection::Map(_, _) => MyhType::Map
        }
    }
    pub fn empty<T: Primitive>(self) -> Option<T> {
        self.item.map(|i|T::from_string(&i))?
    }
    pub fn list<T: Primitive>(self) -> Option<(Option<T>, MyhList)>{
        if let Myh {
            item,
            collection: MyhCollection::List(list)
        } = self {
            Some((item.map(|i|T::from_string(&i))?, MyhList(list)))
        } else { None }
    }
    pub fn map<T: Primitive>(self) -> Option<(Option<T>, MyhMap)> {
        if let Myh {
            item,
            collection: MyhCollection::Map(dict, keys)
        } = self {
            Some((item.map(|i|T::from_string(&i))?, MyhMap(dict, keys)))
        } else { None }
    }

    pub fn to_string(&self) -> String{
        self.stringify(0)
    }

    fn stringify(&self, indent: usize) -> String{
        let mut out = String::new();
        match &self.item {
            None => {}
            Some(item) => out += item
        }
        match &self.collection {
            MyhCollection::Empty => (),
            MyhCollection::List(v) => {
                for item in v{
                    out = out + &format!("\n{}- ", "    ".repeat(indent)) + &item.stringify(indent + 1);
                }
            }
            MyhCollection::Map(d, keys) => {
                for key in keys {
                    let v = d.get(key).unwrap();
                    out = out + &format!("\n{}{}: ", "    ".repeat(indent), key) + &v.stringify(indent + 1);
                }
            }
        }
        out
    }
}

pub struct MyhEmpty;

impl MyhEmpty {
    pub fn myh<T: Primitive>(self, item: Option<&T>) -> Myh {
        Myh {
            item: item.map(|i|i.stringify()),
            collection: MyhCollection::Empty,
        }
    }
}

pub struct MyhList(Vec<Myh>);

impl MyhList {
    pub fn push<T: Serializable>(&mut self, item: &T) {
        self.0.push(item.serialize())
    }
    pub fn pop<T: Serializable>(&mut self) -> Option<T>{
        T::deserialize(self.0.pop()?)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get<T: Serializable>(&mut self, index: usize) -> Option<T> {
        if self.0.len() <= index {
            return None
        }
        T::deserialize(self.0.remove(index))
    }
    pub fn myh<T: Primitive>(self, item: Option<&T>) -> Myh {
        Myh {
            item: item.map(|i|i.stringify()),
            collection: MyhCollection::List(self.0),
        }
    }
}

pub struct MyhMap(HashMap<String, Myh>, Vec<String>);

impl MyhMap {
    pub fn set<T: Serializable>(&mut self, key: &str, item: &T){
        self.0.insert(key.to_string(), item.serialize());
        self.1.push(key.to_string())
    }
    pub fn has_key(&self, key: &str) -> bool{
        self.0.contains_key(key)
    }
    pub fn get<T: Serializable>(&mut self, key: &str) -> Option<T>{
        self.1.retain(|k|k != key);
        self.0.remove(key).map(|myh|T::deserialize(myh))?
    }
    pub fn keys(&self) -> Keys<String, Myh>{
        self.0.keys()
    }
    pub fn myh<T: Primitive>(self, item: Option<&T>) -> Myh {
        Myh {
            item: item.map(|i|i.stringify()),
            collection: MyhCollection::Map(self.0, self.1),
        }
    }
}


