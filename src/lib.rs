#![feature(generic_const_exprs)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(if_let_guard)]
#![feature(let_chains)]

mod primitives;
mod myh;
mod builtins;
pub mod parsing;

pub use primitives::Primitive;
pub use builtins::ranges::AnyRange;
