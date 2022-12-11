use std::str::FromStr;

pub trait Primitive{
    fn stringify(&self) -> String;

    fn from_string(str: &str) -> Option<Self> where Self: Sized;
}

pub trait Prim: FromStr + ToString +{}
macro_rules! impl_prim_for {
    ($($ty: ty)*) => {
        $(
            impl Prim for $ty {}
        )*
    };
}
impl_prim_for!(bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);

impl<T: Prim> Primitive for T {
    fn stringify(&self) -> String {
        self.to_string()
    }

    fn from_string(str: &str) -> Option<Self>{
        str.parse().ok()
    }
}

impl<T: Prim> Primitive for Option<T> {
    fn stringify(&self) -> String {
        match self {
            None => String::new(),
            Some(v) => v.stringify()
        }
    }

    fn from_string(str: &str) -> Option<Self>{
        Some(T::from_string(str))
    }
}