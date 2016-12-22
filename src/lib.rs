mod value;
mod decoder;

pub use value::Value;
pub use decoder::{read, decode, DecodeError};
