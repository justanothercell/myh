use std::collections::hash_map::Keys;
use std::collections::HashMap;
use crate::Primitive;

pub trait Serializable {
    fn serialize(&self) -> Myh;
    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized;
}

impl<T: Primitive> Serializable for T where T: Primitive {
    fn serialize(&self) -> Myh {
        let myh = Myh::new_list();
        myh.myh(Some(self))
    }

    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized {
        let Myh { item, .. } = myh;
        item.as_ref().map(|s|T::from_string(s))?
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Myh {
        todo!()
    }

    fn deserialize(myh: Myh) -> Option<Self> where Self: Sized {
        todo!()
    }
}

pub struct Myh {
    item: Option<String>,
    collection: MyhCollection
}

enum MyhCollection {
    Dict(HashMap<String, Myh>),
    Vec(Vec<Myh>)
}

impl Myh {
    pub fn new_list() -> MyhList{
        MyhList(vec![])
    }
    pub fn new_map() -> MyhMap {
        MyhMap(HashMap::new())
    }
    pub fn list<T: Primitive>(self) -> Option<(Option<T>, MyhList)>{
        if let Myh {
            item,
            collection: MyhCollection::Vec(list)
        } = self {
            Some((item.map(|i|T::from_string(&i))?, MyhList(list)))
        } else { None }
    }
    pub fn map<T: Primitive>(self) -> Option<(Option<T>, MyhMap)> {
        if let Myh {
            item,
            collection: MyhCollection::Dict(dict)
        } = self {
            Some((item.map(|i|T::from_string(&i))?, MyhMap(dict)))
        } else { None }
    }
}

pub struct MyhMap(HashMap<String, Myh>);

impl MyhMap {
    pub fn set<T: Serializable>(&mut self, key: &str, item: &T){
        self.0.insert(key.to_string(), item.serialize());
    }
    pub fn has_key(&self, key: &str) -> bool{
        self.0.contains_key(key)
    }
    pub fn get<T: Serializable>(&mut self, key: &str) -> Option<T>{
        self.0.remove(key).map(|myh|T::deserialize(myh))?
    }
    pub fn keys(&self) -> Keys<String, Myh>{
        self.0.keys()
    }
    pub fn myh<T: Primitive>(self, item: Option<&T>) -> Myh {
        Myh {
            item: item.map(|i|i.stringify()),
            collection: MyhCollection::Dict(self.0),
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
            collection: MyhCollection::Vec(self.0),
        }
    }
}


