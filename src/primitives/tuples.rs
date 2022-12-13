use crate::error::{MyhErr, MyhError};
use crate::parsing::assert_str;
use crate::Primitive;

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

            fn from_string(str: &str) -> Result<Self, MyhError>{
                let mut parts = crate::parsing::split_tuple(str);
                parts.reverse();
                let mut index = 0;
                Ok(($({index += 1; $name::from_string(&parts.pop().unwrap_or(String::new())).map_err(|e: MyhError|e.at(format!("({index})")))?},)+))
            }
        }
    };
}

impl Primitive for (){
    fn stringify(&self) -> String {
        String::from("")
    }

    fn from_string(str: &str) -> Result<Self, MyhError>{
        assert_str(str, "", MyhErr::ParsePrimitiveError("()".to_string(), str.to_string()).into())?;
        Ok(())
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