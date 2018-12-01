#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod coord;
mod coord_arithmetic;

pub use coord::*;
