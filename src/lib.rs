#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod comparisons;
mod consts;
mod enums;
mod functions;
mod macros;
mod modules;
mod name_resolution;
mod structs;
mod traits;

pub use self::consts::*;
pub use self::enums::*;
pub use self::functions::*;
pub use self::macros::*;
pub use self::modules::*;
pub use self::structs::*;
pub use self::traits::*;
