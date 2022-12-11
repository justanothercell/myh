#![feature(generic_const_exprs)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(if_let_guard)]

mod primitives;
mod myh;
mod read;
mod builtins;
mod parsing;

pub use primitives::Primitive;
pub use builtins::ranges::AnyRange;
