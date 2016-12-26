//! Functions to read the content of a dictionary and checking types.

use std::collections::HashMap;
use std::string::FromUtf8Error;

use value::Value;
use decoder::DecodeError;

#[derive(Debug)]
pub enum HelperDecodeError {
    BencodeDecodeError(DecodeError),
    BadType(String),
    MissingKey(String),
    FromUtf8Error(FromUtf8Error),
}

/// Pops a BValue::Integer from a HashMap.
pub fn pop_value_integer(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<i64, HelperDecodeError> {
    match map.remove(&key.clone().into_bytes()) {
        Some(Value::Integer(value)) => Ok(value),
        Some(v) => Err(HelperDecodeError::BadType(format!("Expected integer string for key '{}', got: {:?}", key, v))),
        None => Err(HelperDecodeError::MissingKey(key)),
    }
}

/// Pops an optional BValue::Integer from a HashMap.
pub fn pop_value_integer_option(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Option<i64>, HelperDecodeError> {
    let value = pop_value_integer(map, key);
    match value {
        Ok(value) => Ok(Some(value)),
        Err(HelperDecodeError::MissingKey(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Pops a BValue::String from a HashMap.
pub fn pop_value_bytestring(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Vec<u8>, HelperDecodeError> {
    match map.remove(&key.clone().into_bytes()) {
        Some(Value::String(value)) => Ok(value),
        Some(v) => Err(HelperDecodeError::BadType(format!("Expected UTF8 string for key '{}', got: {:?}", key, v))),
        None => Err(HelperDecodeError::MissingKey(key)),
    }
}

/// Pops an optional BValue::String from a HashMap.
pub fn pop_value_bytestring_option(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Option<Vec<u8>>, HelperDecodeError> {
    let value = pop_value_bytestring(map, key);
    match value {
        Ok(value) => Ok(Some(value)),
        Err(HelperDecodeError::MissingKey(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Pops a BValue::String from a HashMap and decode it into a Rust String.
pub fn pop_value_utf8_string(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<String, HelperDecodeError> {
    let encoded_value = try!(pop_value_bytestring(map, key));
    match String::from_utf8(encoded_value) {
        Ok(decoded_value) => Ok(decoded_value),
        Err(e) => Err(HelperDecodeError::FromUtf8Error(e)),
    }
}

/// Pops an optional BValue::String from a HashMap and decode it into a Rust String.
pub fn pop_value_utf8_string_option(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Option<String>, HelperDecodeError> {
    let value = pop_value_utf8_string(map, key);
    match value {
        Ok(value) => Ok(Some(value)),
        Err(HelperDecodeError::MissingKey(_)) => Ok(None),
        Err(e) => Err(e),
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use value::Value;
    use super::*;

    #[test]
    fn test_utf8() {
        let mut map = HashMap::new();
        map.insert(b"foo".to_vec(), Value::String(b"bar".to_vec()));
        map.insert(b"baz".to_vec(), Value::String(b"qux".to_vec()));
        map.insert(b"quux".to_vec(), Value::Integer(42));
        assert_eq!(pop_value_utf8_string(&mut map, "foo".to_owned()).unwrap(), "bar".to_owned());
        assert_eq!(pop_value_utf8_string(&mut map, "baz".to_owned()).unwrap(), "qux".to_owned());
        assert_eq!(pop_value_integer(&mut map, "quux".to_owned()).unwrap(), 42);
        assert_eq!(map, HashMap::new());
    }
}
