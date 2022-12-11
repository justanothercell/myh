use std::str::FromStr;

pub trait Primitive{
    fn stringify(&self) -> String;

    fn from_string(string: &str) -> Option<Self> where Self: Sized;
}

pub trait Prim: FromStr + ToString +{}
macro_rules! impl_prim_for {
    ($($ty: ty)*) => {
        $(
            impl Prim for $ty {}
        )*
    };
}
impl_prim_for!(bool char u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);

impl<T: Prim> Primitive for T {
    fn stringify(&self) -> String {
        self.to_string()
    }

    fn from_string(string: &str) -> Option<Self>{
        string.parse().ok()
    }
}

impl Primitive for &str {
    fn stringify(&self) -> String {
        self.to_string()
    }

    fn from_string(string: &str) -> Option<Self>{
        string.parse().ok()
    }
}

macro_rules! tuple_primitive_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Primitive),+> Primitive for ($($name,)+) {
            fn stringify(&self) -> String {
                let mut out = String::new();
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                $(out += &$name.stringify(); out += ", ";)+
                out.pop();
                out.pop();
                out
            }

            fn from_string(string: &str) -> Option<Self>{
                let mut parts = string.split(',').map(|s|s.trim()).collect::<Vec<_>>();
                parts.reverse();
                Some(($(if let Some(part) = parts.pop() { $name::from_string(part)?} else { None? },)+))
            }
        }
    };
}

impl Primitive for (){
    fn stringify(&self) -> String {
        String::from("()")
    }

    fn from_string(_string: &str) -> Option<Self>{
        Some(())
    }
}

tuple_primitive_impls! { A }
tuple_primitive_impls! { A B }
tuple_primitive_impls! { A B C }
tuple_primitive_impls! { A B C D }
tuple_primitive_impls! { A B C D E }
tuple_primitive_impls! { A B C D E F }
tuple_primitive_impls! { A B C D E F G }
tuple_primitive_impls! { A B C D E F G H }
tuple_primitive_impls! { A B C D E F G H I }
tuple_primitive_impls! { A B C D E F G H I J }
tuple_primitive_impls! { A B C D E F G H I J K }
tuple_primitive_impls! { A B C D E F G H I J K L }
tuple_primitive_impls! { A B C D E F G H I J K L M }
tuple_primitive_impls! { A B C D E F G H I J K L M N }
tuple_primitive_impls! { A B C D E F G H I J K L M N O }
tuple_primitive_impls! { A B C D E F G H I J K L M N O P }