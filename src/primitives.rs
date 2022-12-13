pub mod ranges;
mod strings;
mod tuples;

use std::str::FromStr;
use crate::error::{MyhErr, MyhError};
use crate::parsing::split_tuple;

pub trait Primitive {
    fn stringify(&self) -> String;

    fn from_string(str: &str) -> Result<Self, MyhError> where Self: Sized;
}

pub trait Prim: FromStr + ToString {
    const TY: &'static str;
}
macro_rules! impl_prim_for {
    ($($ty: ty)*) => {
        $(
            impl Prim for $ty {
                const TY: &'static str = stringify!($ty);
            }
        )*
    };
}

impl_prim_for!(bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);

impl<T: Prim> Primitive for T {
    fn stringify(&self) -> String {
        self.to_string()
    }

    fn from_string(str: &str) -> Result<Self, MyhError>{
        str.parse().map_err(|_e| MyhErr::ParsePrimitiveError(<T as Prim>::TY.to_string(), str.to_string()).into())
    }
}

impl<T: Prim> Primitive for Option<T> {
    fn stringify(&self) -> String {
        match self {
            None => String::new(),
            Some(v) => v.stringify()
        }
    }

    fn from_string(str: &str) -> Result<Self, MyhError>{
        Ok(T::from_string(str).ok())
    }
}

pub struct PrimVec<T>(Vec<T>);

impl<T> PrimVec<T> {
    pub fn from(vec: Vec<T>) -> Self{
        Self(vec)
    }
}

impl<T> From<Vec<T>> for PrimVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> Into<Vec<T>> for PrimVec<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

impl<T: Primitive> Primitive for PrimVec<T> {
    fn stringify(&self) -> String {
        self.0.iter().map(|i|i.stringify()).collect::<Vec<_>>().join(", ")
    }

    fn from_string(str: &str) -> Result<Self, MyhError> {
        split_tuple(str).into_iter().enumerate().map(|(i, s)|T::from_string(&s).map_err(|e: MyhError|e.at(format!("({i})")))).collect::<Result<Vec<T>, MyhError>>().map(|v|v.into())
    }
}