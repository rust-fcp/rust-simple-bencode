use std::collections::HashMap;

/// The possible value types in a bencode object.
#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<Value>),
    Dictionary(HashMap<Vec<u8>, Value>),
}
