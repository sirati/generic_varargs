#![feature(fundamental)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(never_type)]
#![allow(incomplete_features)]
#![feature(specialization)]

mod generic;
mod same_type;

pub use same_type::{Same, SameSelf};
pub use generic::*;